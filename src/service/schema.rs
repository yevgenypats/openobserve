// Copyright 2023 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    sync::Arc,
};

use ahash::AHashMap;
use datafusion::arrow::{
    datatypes::{DataType, Field, Schema},
    error::ArrowError,
};
use itertools::Itertools;

use crate::{
    common::{
        infra::{
            config::{CONFIG, LOCAL_SCHEMA_LOCKER},
            db::etcd,
        },
        meta::{
            ingestion::StreamSchemaChk, prom::METADATA_LABEL, stream::SchemaEvolution, StreamType,
        },
        utils::{json, schema::infer_json_schema, schema_ext::SchemaExt},
    },
    service::{db, search::server_internal_error},
};

#[tracing::instrument(name = "service:schema:schema_evolution", skip(inferred_schema))]
pub async fn schema_evolution(
    org_id: &str,
    stream_name: &str,
    stream_type: StreamType,
    inferred_schema: Arc<Schema>,
    min_ts: i64,
) {
    let schema = db::schema::get(org_id, stream_name, stream_type)
        .await
        .unwrap();

    if schema == Schema::empty() {
        let mut metadata = HashMap::new();
        metadata.insert("created_at".to_string(), min_ts.to_string());
        log::info!("schema_evolution: setting schema for {:?}", stream_name);
        db::schema::set(
            org_id,
            stream_name,
            stream_type,
            &Arc::into_inner(inferred_schema)
                .unwrap()
                .with_metadata(metadata),
            Some(min_ts),
            false,
        )
        .await
        .unwrap();
    } else if !inferred_schema.fields().eq(schema.fields()) {
        let schema_fields: HashSet<_> = schema.fields().iter().collect();
        let mut field_datatype_delta: Vec<_> = vec![];
        let mut new_field_delta: Vec<_> = vec![];

        for item in inferred_schema.fields.iter() {
            let item_name = item.name();
            let item_data_type = item.data_type();

            match schema_fields.iter().find(|f| f.name() == item_name) {
                Some(existing_field) => {
                    if existing_field.data_type() != item_data_type
                        && existing_field.data_type() != &DataType::Utf8
                    {
                        field_datatype_delta.push(format!("{}:[{}]", item_name, item_data_type));
                    }
                }
                None => {
                    new_field_delta.push(format!("{}:[{}]", item_name, item_data_type));
                }
            }
        }
        if field_datatype_delta.is_empty() && new_field_delta.is_empty() {
            return;
        }
        log::info!(
            "schema_evolution: updating schema for {:?} field data type delta is {:?} ,newly added fields are {:?}",
            stream_name,
            field_datatype_delta,
            new_field_delta
        );
        match try_merge(vec![schema, Arc::into_inner(inferred_schema).unwrap()]) {
            Err(e) => {
                log::error!(
                    "schema_evolution: schema merge failed for {:?} err: {:?}",
                    stream_name,
                    e
                );
            }
            Ok(merged) => {
                if !field_datatype_delta.is_empty() || !new_field_delta.is_empty() {
                    let is_field_delta = !field_datatype_delta.is_empty();
                    let mut final_fields = vec![];

                    let metadata = merged.metadata().clone();

                    for mut field in merged.to_cloned_fields().into_iter() {
                        if field.metadata().contains_key("zo_cast") {
                            let mut new_meta = field.metadata().clone();
                            new_meta.remove_entry("zo_cast");
                            field.set_metadata(new_meta);
                        }
                        final_fields.push(field);
                    }
                    let final_schema = Schema::new(final_fields.to_vec()).with_metadata(metadata);
                    db::schema::set(
                        org_id,
                        stream_name,
                        stream_type,
                        &final_schema,
                        Some(min_ts),
                        is_field_delta,
                    )
                    .await
                    .unwrap();
                }
            }
        };
    }
}

// Hack to allow widening conversion, method overrides Schema::try_merge
fn try_merge(schemas: impl IntoIterator<Item = Schema>) -> Result<Schema, ArrowError> {
    let mut merged_metadata: HashMap<String, String> = HashMap::new();
    let mut merged_fields: Vec<Field> = Vec::new();
    // TODO : this dummy initialization is to avoid compiler complaining for
    // uninitialized value
    let mut temp_field = Field::new("dummy", DataType::Utf8, false);

    for schema in schemas {
        for (key, value) in schema.metadata() {
            // merge metadata
            if let Some(old_val) = merged_metadata.get(key) {
                if old_val != value {
                    return Err(ArrowError::SchemaError(
                        "Fail to merge schema due to conflicting metadata.".to_string(),
                    ));
                }
            }
            merged_metadata.insert(key.to_string(), value.to_string());
        }

        // merge fields
        let mut found_at = 0;
        for field in schema.to_cloned_fields().iter().sorted_by_key(|v| v.name()) {
            let mut new_field = true;
            let mut allowed = false;
            for (stream, mut merged_field) in merged_fields.iter_mut().enumerate() {
                if field.name() != merged_field.name() {
                    continue;
                }
                new_field = false;
                if merged_field.data_type() != field.data_type() {
                    if !CONFIG.common.widening_schema_evolution {
                        return Err(ArrowError::SchemaError(format!(
                            "Fail to merge schema due to conflicting data type[{}:{}].",
                            merged_field.data_type(),
                            field.data_type()
                        )));
                    }
                    allowed = is_widening_conversion(merged_field.data_type(), field.data_type());
                    if allowed {
                        temp_field = Field::new(
                            merged_field.name(),
                            field.data_type().to_owned(),
                            merged_field.is_nullable(),
                        );
                        merged_field = &mut temp_field;
                    }
                }
                found_at = stream;
                match merged_field.try_merge(field) {
                    Ok(_) => {}
                    Err(_) => {
                        let mut meta = field.metadata().clone();
                        meta.insert("zo_cast".to_owned(), true.to_string());
                        merged_field.set_metadata(meta);
                    }
                };
            }
            // found a new field, add to field list
            if new_field {
                merged_fields.push(field.clone());
            }
            if allowed {
                let _ = std::mem::replace(&mut merged_fields[found_at], temp_field.to_owned());
            }
        }
    }
    let merged = Schema::new_with_metadata(merged_fields, merged_metadata);
    Ok(merged)
}

fn is_widening_conversion(from: &DataType, to: &DataType) -> bool {
    let allowed_type = match from {
        DataType::Boolean => vec![DataType::Utf8],
        DataType::Int8 => vec![
            DataType::Utf8,
            DataType::Int16,
            DataType::Int32,
            DataType::Int64,
            DataType::Float16,
            DataType::Float32,
            DataType::Float64,
        ],
        DataType::Int16 => vec![
            DataType::Utf8,
            DataType::Int32,
            DataType::Int64,
            DataType::Float16,
            DataType::Float32,
            DataType::Float64,
        ],
        DataType::Int32 => vec![
            DataType::Utf8,
            DataType::Int64,
            DataType::UInt32,
            DataType::UInt64,
            DataType::Float32,
            DataType::Float64,
        ],
        DataType::Int64 => vec![DataType::Utf8, DataType::UInt64, DataType::Float64],
        DataType::UInt8 => vec![
            DataType::Utf8,
            DataType::UInt16,
            DataType::UInt32,
            DataType::UInt64,
        ],
        DataType::UInt16 => vec![DataType::Utf8, DataType::UInt32, DataType::UInt64],
        DataType::UInt32 => vec![DataType::Utf8, DataType::UInt64],
        DataType::UInt64 => vec![DataType::Utf8],
        DataType::Float16 => vec![DataType::Utf8, DataType::Float32, DataType::Float64],
        DataType::Float32 => vec![DataType::Utf8, DataType::Float64],
        DataType::Float64 => vec![DataType::Utf8],
        _ => vec![DataType::Utf8],
    };
    allowed_type.contains(to)
}

pub async fn check_for_schema(
    org_id: &str,
    stream_name: &str,
    stream_type: StreamType,
    val_str: &str,
    stream_schema_map: &mut AHashMap<String, Schema>,
    record_ts: i64,
    is_arrow: bool,
) -> SchemaEvolution {
    let mut schema = if stream_schema_map.contains_key(stream_name) {
        stream_schema_map.get(stream_name).unwrap().clone()
    } else {
        let schema = db::schema::get(org_id, stream_name, stream_type)
            .await
            .unwrap();
        stream_schema_map.insert(stream_name.to_string(), schema.clone());
        schema
    };

    if !schema.fields().is_empty() && CONFIG.common.skip_schema_validation {
        // return (true, None, schema.fields().to_vec());
        return SchemaEvolution {
            schema_compatible: true,
            types_delta: None,
            schema_fields: schema.to_cloned_fields(),
            is_schema_changed: false,
            record_schema: schema,
        };
    }

    let mut schema_reader = BufReader::new(val_str.as_bytes());
    let mut inferred_schema = infer_json_schema(&mut schema_reader, None, stream_type).unwrap();
    filter_schema_null_fields(&mut inferred_schema);

    if schema.fields.eq(&inferred_schema.fields) {
        // return (true, None, schema.fields().to_vec());
        return SchemaEvolution {
            schema_compatible: true,
            types_delta: None,
            schema_fields: schema.to_cloned_fields(),
            is_schema_changed: false,
            record_schema: schema,
        };
    }

    if inferred_schema.fields.len() > CONFIG.limit.req_cols_per_record_limit {
        // return (false, None, inferred_schema.fields().to_vec());
        return SchemaEvolution {
            schema_compatible: false,
            types_delta: None,
            schema_fields: inferred_schema.to_cloned_fields(),
            is_schema_changed: false,
            record_schema: schema,
        };
    }

    if schema.fields().is_empty() {
        if let Some(value) = handle_new_schema(
            &mut schema,
            &inferred_schema,
            stream_schema_map,
            stream_name,
            org_id,
            stream_type,
            &record_ts,
        )
        .await
        {
            return value;
        }
    };

    let (field_datatype_delta, is_schema_changed, final_fields, record_schema) =
        get_schema_changes(&schema, &inferred_schema, is_arrow);

    if is_schema_changed {
        if let Some(value) = handle_existing_schema(
            stream_name,
            org_id,
            stream_type,
            &inferred_schema,
            record_ts,
            stream_schema_map,
            is_arrow,
        )
        .await
        {
            value
        } else {
            SchemaEvolution {
                schema_compatible: true,
                types_delta: Some(field_datatype_delta),
                schema_fields: schema.to_cloned_fields(),
                is_schema_changed: false,
                record_schema,
            }
        }
    } else {
        SchemaEvolution {
            schema_compatible: true,
            types_delta: Some(field_datatype_delta),
            schema_fields: final_fields,
            is_schema_changed,
            record_schema,
        }
    }
}

async fn handle_existing_schema(
    stream_name: &str,
    org_id: &str,
    stream_type: StreamType,
    inferred_schema: &Schema,
    record_ts: i64,
    stream_schema_map: &mut AHashMap<String, Schema>,
    is_arrow: bool,
) -> Option<SchemaEvolution> {
    if !CONFIG.common.local_mode {
        let mut lock = etcd::Locker::new(&format!("schema/{org_id}/{stream_type}/{stream_name}"));
        lock.lock(0).await.map_err(server_internal_error).unwrap();
        let schema = db::schema::get_from_db(org_id, stream_name, stream_type)
            .await
            .unwrap();
        let (field_datatype_delta, is_schema_changed, final_fields, _) =
            get_schema_changes(&schema, inferred_schema, is_arrow);
        let is_field_delta = !field_datatype_delta.is_empty();
        let mut metadata = schema.metadata().clone();
        if !metadata.contains_key("created_at") {
            metadata.insert(
                "created_at".to_string(),
                chrono::Utc::now().timestamp_micros().to_string(),
            );
        }
        metadata.extend(inferred_schema.metadata().to_owned());
        let final_schema = Schema::new(final_fields.clone()).with_metadata(metadata);
        if is_schema_changed {
            log::info!(
                "Acquired lock for cluster stream {} to update schema",
                stream_name
            );
            db::schema::set(
                org_id,
                stream_name,
                stream_type,
                &final_schema,
                Some(record_ts),
                is_field_delta,
            )
            .await
            .unwrap();
            lock.unlock().await.map_err(server_internal_error).unwrap();
            stream_schema_map.insert(stream_name.to_string(), final_schema.clone());
        } else {
            lock.unlock().await.map_err(server_internal_error).unwrap();
            stream_schema_map.insert(stream_name.to_string(), schema.clone());
        }
        Some(SchemaEvolution {
            schema_compatible: true,
            types_delta: Some(field_datatype_delta),
            schema_fields: final_fields,
            is_schema_changed,
            record_schema: final_schema,
        })
    } else {
        let key = format!(
            "{}/schema/lock/{org_id}/{stream_type}/{stream_name}",
            &CONFIG.sled.prefix
        );
        let local_map = LOCAL_SCHEMA_LOCKER.clone();
        let mut schema_locker = local_map.write().await;
        let value = schema_locker
            .entry(key)
            .or_insert_with(|| tokio::sync::RwLock::new(false));

        let lock_acquired = value.write().await; // lock acquired

        if !*lock_acquired {
            let schema = db::schema::get_from_db(org_id, stream_name, stream_type)
                .await
                .unwrap();
            let (field_datatype_delta, is_schema_changed, final_fields, _) =
                get_schema_changes(&schema, inferred_schema, is_arrow);
            let is_field_delta = !field_datatype_delta.is_empty();
            let mut metadata = schema.metadata().clone();
            if !metadata.contains_key("created_at") {
                metadata.insert(
                    "created_at".to_string(),
                    chrono::Utc::now().timestamp_micros().to_string(),
                );
            }
            metadata.extend(inferred_schema.metadata().to_owned());
            let final_schema = Schema::new(final_fields.clone()).with_metadata(metadata);
            if is_schema_changed {
                log::info!(
                    "Acquired lock for local stream {} to update schema",
                    stream_name
                );
                db::schema::set(
                    org_id,
                    stream_name,
                    stream_type,
                    &final_schema,
                    Some(record_ts),
                    is_field_delta,
                )
                .await
                .unwrap();
                stream_schema_map.insert(stream_name.to_string(), final_schema.clone());
            } else {
                // No Change in schema.
                stream_schema_map.insert(stream_name.to_string(), schema);
            }
            drop(lock_acquired); // release lock

            Some(SchemaEvolution {
                schema_compatible: true,
                types_delta: Some(field_datatype_delta),
                schema_fields: final_fields,
                is_schema_changed,
                record_schema: final_schema,
            })
        } else {
            // Some other request has already acquired the lock.
            let schema = db::schema::get_from_db(org_id, stream_name, stream_type)
                .await
                .unwrap();
            let (field_datatype_delta, _is_schema_changed, final_fields, _) =
                get_schema_changes(&schema, inferred_schema, is_arrow);
            stream_schema_map.insert(stream_name.to_string(), schema.clone());
            log::info!("Schema exists for stream {} ", stream_name);
            drop(lock_acquired); // release lock
            Some(SchemaEvolution {
                schema_compatible: true,
                types_delta: Some(field_datatype_delta),
                schema_fields: final_fields,
                is_schema_changed: false,
                record_schema: schema,
            })
        }
    }
}

async fn handle_new_schema(
    schema: &mut Schema,
    inferred_schema: &Schema,
    stream_schema_map: &mut AHashMap<String, Schema>,
    stream_name: &str,
    org_id: &str,
    stream_type: StreamType,
    record_ts: &i64,
) -> Option<SchemaEvolution> {
    if *schema == Schema::empty() {
        let mut metadata = inferred_schema.metadata.clone();
        if !metadata.contains_key("created_at") {
            metadata.insert(
                "created_at".to_string(),
                chrono::Utc::now().timestamp_micros().to_string(),
            );
        }
        let final_schema = inferred_schema.clone().with_metadata(metadata.clone());
        stream_schema_map.insert(stream_name.to_string(), final_schema.clone());

        if !CONFIG.common.local_mode {
            let mut lock =
                etcd::Locker::new(&format!("schema/{org_id}/{stream_type}/{stream_name}"));
            lock.lock(0).await.map_err(server_internal_error).unwrap();
            log::info!("Aquired lock for stream {} as schema is empty", stream_name);

            // try getting schema

            let chk_schema = db::schema::get_from_db(org_id, stream_name, stream_type)
                .await
                .unwrap();

            if chk_schema.fields().is_empty() {
                log::info!(
                    "Setting schema for stream {} as schema is empty",
                    stream_name
                );
                db::schema::set(
                    org_id,
                    stream_name,
                    stream_type,
                    &final_schema,
                    Some(*record_ts),
                    false,
                )
                .await
                .unwrap();
                lock.unlock().await.map_err(server_internal_error).unwrap();
                log::info!(
                    "Releasing lock for stream {} after schema is set",
                    stream_name
                );

                // return (true, None, final_schema.fields().to_vec());
                return Some(SchemaEvolution {
                    schema_compatible: true,
                    types_delta: None,
                    schema_fields: final_schema.to_cloned_fields(),
                    is_schema_changed: true,
                    record_schema: final_schema,
                });
            } else {
                stream_schema_map.insert(stream_name.to_string(), chk_schema.clone());
                *schema = chk_schema;

                lock.unlock().await.map_err(server_internal_error).unwrap();
                log::info!(
                    "Releasing lock for stream {} after schema is set",
                    stream_name
                );
            }
        } else {
            let key = format!(
                "{}/schema/lock/{org_id}/{stream_type}/{stream_name}",
                &CONFIG.sled.prefix
            );

            let map = LOCAL_SCHEMA_LOCKER.clone(); // get a copy to read the value

            let mut schema_locker = map.write().await; // lock it for writing a key for stream
            let value = schema_locker
                .entry(key)
                .or_insert_with(|| tokio::sync::RwLock::new(false)); // if stream schema doesn't exist, create a new key set value as false or for existing key get the value

            let lock_acquired = value.write().await; //  acquire lock for writing
            if !*lock_acquired {
                log::info!(
                    "Acquired lock for stream {} as schema is empty",
                    stream_name
                );
                let chk_schema = db::schema::get_from_db(org_id, stream_name, stream_type)
                    .await
                    .unwrap();
                if chk_schema.fields().is_empty() {
                    log::info!(
                        "Setting schema for stream {} as schema is empty",
                        stream_name
                    );
                    db::schema::set(
                        org_id,
                        stream_name,
                        stream_type,
                        &final_schema,
                        Some(*record_ts),
                        false,
                    )
                    .await
                    .unwrap();
                    drop(lock_acquired); // release lock
                    return Some(SchemaEvolution {
                        schema_compatible: true,
                        types_delta: None,
                        schema_fields: final_schema.to_cloned_fields(),
                        is_schema_changed: true,
                        record_schema: final_schema,
                    });
                } else {
                    // No schema change
                    stream_schema_map.insert(stream_name.to_string(), chk_schema.clone());
                    drop(lock_acquired); // release lock
                    *schema = chk_schema;
                    log::info!(
                        "Schema exists for stream {} and No schema change",
                        stream_name
                    );
                }
            } else {
                // Some other request has already acquired the lock.
                //*lock_acquired = false;
                drop(lock_acquired); // release lock
                let chk_schema = db::schema::get_from_db(org_id, stream_name, stream_type)
                    .await
                    .unwrap();
                *schema = chk_schema;
                log::info!("Schema exists for stream {} ,already locked", stream_name);
            }
        }
    }
    None
}

fn get_schema_changes(
    schema: &Schema,
    inferred_schema: &Schema,
    _is_arrow: bool,
) -> (Vec<Field>, bool, Vec<Field>, Schema) {
    let mut field_datatype_delta: Vec<_> = vec![];
    let mut new_field_delta: Vec<_> = vec![];
    let mut merged_fields: AHashMap<String, Field> = AHashMap::new();

    let mut is_schema_changed = false;

    for f in schema.fields.iter() {
        merged_fields.insert(f.name().to_owned(), (**f).clone());
    }

    for item in inferred_schema.fields.iter() {
        let item_name = item.name();
        let item_data_type = item.data_type();

        match merged_fields.get(item_name) {
            Some(existing_field) => {
                if existing_field.data_type() != item_data_type {
                    if !CONFIG.common.widening_schema_evolution {
                        field_datatype_delta.push(existing_field.clone());
                    } else {
                        let allowed =
                            is_widening_conversion(existing_field.data_type(), item_data_type);
                        if allowed {
                            is_schema_changed = true;
                            field_datatype_delta.push((**item).clone());
                            merged_fields.insert(item_name.to_owned(), (**item).clone());
                        } else {
                            let mut meta = existing_field.metadata().clone();
                            meta.insert("zo_cast".to_owned(), true.to_string());
                            field_datatype_delta.push(existing_field.clone().with_metadata(meta));
                        }
                    }
                }
            }
            None => {
                is_schema_changed = true;
                new_field_delta.push(item);
                merged_fields.insert(item_name.to_owned(), (**item).clone());
            }
        }
    }
    // let mut rec_schema = Schema::empty();
    // if !field_datatype_delta.is_empty() {
    // update data type
    // match try_merge(vec![
    // schema.clone(),
    // Arc::into_inner(inferred_schema.clone().into()).unwrap(),
    // ]) {
    // Err(e) => {
    // log::error!("get_schema_changes: schema merge failed err: {:?}", e);
    // }
    // Ok(merged) => {
    // rec_schema = merged;
    // }
    // }
    // }

    let final_fields: Vec<Field> = merged_fields.drain().map(|(_key, value)| value).collect();
    (
        field_datatype_delta,
        is_schema_changed,
        final_fields,
        Schema::empty(),
    )
}

pub async fn stream_schema_exists(
    org_id: &str,
    stream_name: &str,
    stream_type: StreamType,
    stream_schema_map: &mut AHashMap<String, Schema>,
) -> StreamSchemaChk {
    let mut schema_chk = StreamSchemaChk {
        conforms: true,
        has_fields: false,
        has_partition_keys: false,
        has_metadata: false,
    };
    let schema = match stream_schema_map.get(stream_name) {
        Some(schema) => schema.clone(),
        None => {
            let schema = db::schema::get(org_id, stream_name, stream_type)
                .await
                .unwrap();
            stream_schema_map.insert(stream_name.to_string(), schema.clone());
            schema
        }
    };
    if !schema.fields().is_empty() {
        schema_chk.has_fields = true;
    }
    if let Some(value) = schema.metadata().get("settings") {
        let settings: json::Value = json::from_slice(value.as_bytes()).unwrap();
        if settings.get("partition_keys").is_some() {
            schema_chk.has_partition_keys = true;
        }
    }
    if schema.metadata().contains_key(METADATA_LABEL) {
        schema_chk.has_metadata = true;
    }
    schema_chk
}

pub async fn add_stream_schema(
    org_id: &str,
    stream_name: &str,
    stream_type: StreamType,
    file: &File,
    stream_schema_map: &mut AHashMap<String, Schema>,
    min_ts: i64,
) {
    let mut local_file = file;
    local_file.seek(SeekFrom::Start(0)).unwrap();
    let mut schema_reader = BufReader::new(local_file);
    let mut inferred_schema = infer_json_schema(&mut schema_reader, None, stream_type).unwrap();
    filter_schema_null_fields(&mut inferred_schema);

    let existing_schema = stream_schema_map.get(&stream_name.to_string());
    let mut metadata = match existing_schema {
        Some(schema) => schema.metadata().clone(),
        None => HashMap::new(),
    };
    metadata.insert("created_at".to_string(), min_ts.to_string());
    if stream_type == StreamType::Traces {
        let settings = crate::common::meta::stream::StreamSettings {
            partition_keys: vec!["service_name".to_string()],
            partition_time_level: None,
            full_text_search_keys: vec![],
            bloom_filter_fields: vec![],
            data_retention: 0,
        };
        metadata.insert(
            "settings".to_string(),
            json::to_string(&settings).unwrap_or_default(),
        );
    }
    db::schema::set(
        org_id,
        stream_name,
        stream_type,
        &inferred_schema.clone().with_metadata(metadata),
        Some(min_ts),
        false,
    )
    .await
    .unwrap();
    stream_schema_map.insert(stream_name.to_string(), inferred_schema.clone());
}

pub async fn set_schema_metadata(
    org_id: &str,
    stream_name: &str,
    stream_type: StreamType,
    extra_metadata: &AHashMap<String, String>,
) -> Result<(), anyhow::Error> {
    let schema = db::schema::get(org_id, stream_name, stream_type).await?;
    let mut metadata = schema.metadata().clone();
    let mut updated = false;
    for (key, value) in extra_metadata {
        if metadata.contains_key(key) {
            continue;
        }
        metadata.insert(key.to_owned(), value.to_owned());
        updated = true;
    }
    if !updated {
        return Ok(());
    }
    if !metadata.contains_key("created_at") {
        metadata.insert(
            "created_at".to_string(),
            chrono::Utc::now().timestamp_micros().to_string(),
        );
    }
    db::schema::set(
        org_id,
        stream_name,
        stream_type,
        &schema.with_metadata(metadata),
        None,
        false,
    )
    .await
}

pub fn filter_schema_null_fields(schema: &mut Schema) {
    let fields = schema.fields();
    if fields
        .iter()
        .filter(|f| f.data_type() == &DataType::Null)
        .count()
        > 0
    {
        let fields = fields
            .iter()
            .filter_map(|f| {
                if f.data_type() == &DataType::Null {
                    None
                } else {
                    Some(f.as_ref().to_owned())
                }
            })
            .collect::<Vec<_>>();
        *schema = Schema::new(fields.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use ahash::AHashMap;
    use datafusion::arrow::datatypes::{DataType, Field, Schema};

    use super::*;

    #[test]
    fn test_is_widening_conversion() {
        assert!(is_widening_conversion(&DataType::Int8, &DataType::Int32));
    }

    #[test]
    fn test_try_merge() {
        let merged = try_merge(vec![
            Schema::new(vec![
                Field::new("c1", DataType::Int64, false),
                Field::new("c2", DataType::Utf8, false),
            ]),
            Schema::new(vec![
                Field::new("c1", DataType::Int64, true),
                Field::new("c2", DataType::Utf8, false),
                Field::new("c3", DataType::Utf8, false),
            ]),
        ])
        .unwrap();

        assert_eq!(
            merged,
            Schema::new(vec![
                Field::new("c1", DataType::Int64, true),
                Field::new("c2", DataType::Utf8, false),
                Field::new("c3", DataType::Utf8, false),
            ]),
        );
    }

    #[actix_web::test]
    async fn test_check_for_schema() {
        let stream_name = "Sample";
        let org_name = "nexus";
        let record = r#"{"Year": 1896, "City": "Athens", "_timestamp": 1234234234234}"#;

        let schema = Schema::new(vec![
            Field::new("Year", DataType::Int64, false),
            Field::new("City", DataType::Utf8, false),
            Field::new("_timestamp", DataType::Int64, false),
        ]);
        let mut map: AHashMap<String, Schema> = AHashMap::new();
        map.insert(stream_name.to_string(), schema);
        let result = check_for_schema(
            org_name,
            stream_name,
            StreamType::Logs,
            record,
            &mut map,
            1234234234234,
            false,
        )
        .await;
        assert!(result.schema_compatible);
    }
}

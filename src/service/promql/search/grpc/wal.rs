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

use std::{collections::HashMap, io::Cursor, sync::Arc};

use arrow::{ipc::reader::StreamReader, record_batch::RecordBatch};
use datafusion::{
    arrow::datatypes::Schema,
    common::FileType,
    datasource::MemTable,
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use futures::future::try_join_all;
use tonic::{codec::CompressionEncoding, metadata::MetadataValue, transport::Channel, Request};
use tracing::{info_span, Instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    common::{
        infra::{
            cache::tmpfs,
            cluster::{get_cached_online_ingester_nodes, get_internal_grpc_token},
            config::{CONFIG, FILE_EXT_ARROW, FILE_EXT_JSON},
        },
        meta::{
            search::{SearchType, Session as SearchSession},
            stream::ScanStats,
            StreamType,
        },
    },
    handler::grpc::cluster_rpc,
    service::{
        db,
        search::{
            datafusion::{
                exec::{prepare_datafusion_context, register_table},
                storage::StorageType,
            },
            MetadataMap,
        },
    },
};

#[tracing::instrument(name = "promql:search:grpc:wal:create_context", skip_all)]
pub(crate) async fn create_context(
    session_id: &str,
    org_id: &str,
    stream_name: &str,
    time_range: (i64, i64),
    _filters: &[(&str, Vec<&str>)],
) -> Result<Vec<(SessionContext, Arc<Schema>, ScanStats)>> {
    let mut resp = vec![];
    // get file list
    let files = get_file_list(session_id, org_id, stream_name, time_range).await?;
    if files.is_empty() {
        return Ok(vec![(
            SessionContext::new(),
            Arc::new(Schema::empty()),
            ScanStats::default(),
        )]);
    }

    let mut num_arrow_files = 0;
    let mut num_json_files = 0;
    let mut arrow_scan_stats = ScanStats::new();
    let mut json_scan_stats = ScanStats::new();

    let metadata = HashMap::new();
    let mut record_batches_meta: HashMap<String, (Schema, Vec<RecordBatch>)> = HashMap::new();

    let work_dir = session_id.to_string();

    for file in files {
        let file_name = format!("/{work_dir}/{}", file.name);

        if file.name.ends_with(FILE_EXT_JSON) {
            num_json_files += 1;
            json_scan_stats.original_size += file.body.len() as i64;
            tmpfs::set(&file_name, file.body.into()).expect("tmpfs set success");
        } else if file.name.ends_with(FILE_EXT_ARROW) {
            num_arrow_files += 1;
            let record_batch_meta = record_batches_meta
                .entry(file.schema)
                .or_insert_with(|| (Schema::empty().with_metadata(metadata.clone()), Vec::new()));

            let buf_reader = Cursor::new(file.body.clone());
            let stream_reader = StreamReader::try_new(buf_reader, None)?;
            for read_result in stream_reader {
                let record_batch = read_result?;
                if record_batch.num_rows() > 0 {
                    if record_batch_meta.0.fields().is_empty() {
                        record_batch_meta.0 = record_batch
                            .schema()
                            .as_ref()
                            .clone()
                            .with_metadata(metadata.clone());
                    }
                    record_batch_meta.1.push(record_batch);
                }
            }
            arrow_scan_stats.original_size += file.body.len() as i64;
        }
    }

    arrow_scan_stats.files = num_arrow_files;
    json_scan_stats.files = num_json_files;

    log::info!(
        "promql->wal->search: load files json :{} , scan_size {} , arrow :{} , scan_size {}",
        json_scan_stats.files,
        json_scan_stats.original_size,
        arrow_scan_stats.files,
        arrow_scan_stats.original_size
    );

    // fetch all schema versions, get latest schema
    let stream_type = StreamType::Metrics;
    let schema = db::schema::get(org_id, stream_name, stream_type)
        .await
        .map_err(|err| {
            log::error!("get schema error: {}", err);
            DataFusionError::Execution(err.to_string())
        })?;
    for (_, (mut arrow_schema, record_batches)) in record_batches_meta {
        if !record_batches.is_empty() {
            let ctx = prepare_datafusion_context(&SearchType::Normal)?;
            // calulate schema diff
            let mut diff_fields = HashMap::new();
            let group_fields = arrow_schema.fields();
            for field in group_fields {
                if let Ok(v) = schema.field_with_name(field.name()) {
                    if v.data_type() != field.data_type() {
                        diff_fields.insert(v.name().clone(), v.data_type().clone());
                    }
                }
            }
            // add not exists field for wal infered schema
            let mut new_fields = Vec::new();
            for field in schema.fields() {
                if arrow_schema.field_with_name(field.name()).is_err() {
                    new_fields.push(field.clone());
                }
            }
            if !new_fields.is_empty() {
                let new_schema = Schema::new(new_fields);
                arrow_schema = Schema::try_merge(vec![arrow_schema, new_schema])?;
            }
            let arrow_schema = Arc::new(arrow_schema);

            let schema = if let Some(first_batch) = record_batches.first() {
                first_batch.schema()
            } else {
                arrow_schema
            };
            let mem_table = Arc::new(MemTable::try_new(schema.clone(), vec![record_batches])?);
            ctx.register_table(stream_name, mem_table)?;
            resp.push((ctx, schema, arrow_scan_stats));
        }
    }

    let schema = Arc::new(
        schema
            .to_owned()
            .with_metadata(std::collections::HashMap::new()),
    );
    let session = SearchSession {
        id: session_id.to_string(),
        storage_type: StorageType::Tmpfs,
        search_type: SearchType::Normal,
    };

    let ctx = register_table(&session, schema.clone(), stream_name, &[], FileType::JSON).await?;
    resp.push((ctx, schema, json_scan_stats));
    Ok(resp)
}

/// get file list from local cache, no need match_source, each file will be
/// searched
#[tracing::instrument(name = "promql:search:grpc:wal:get_file_list")]
async fn get_file_list(
    session_id: &str,
    org_id: &str,
    stream_name: &str,
    time_range: (i64, i64),
) -> Result<Vec<cluster_rpc::MetricsWalFile>> {
    let nodes = get_cached_online_ingester_nodes();
    if nodes.is_none() && nodes.as_deref().unwrap().is_empty() {
        return Ok(vec![]);
    }
    let nodes = nodes.unwrap();

    let mut tasks = Vec::new();
    for node in nodes {
        let session_id = session_id.to_string();
        let node_addr = node.grpc_addr.clone();
        let org_id = org_id.to_string();
        let req = cluster_rpc::MetricsWalFileRequest {
            org_id: org_id.clone(),
            stream_name: stream_name.to_string(),
            start_time: time_range.0,
            end_time: time_range.1,
        };
        let grpc_span =
            info_span!("promql:search:grpc:wal:grpc_wal_file", session_id = ?session_id);
        let task: tokio::task::JoinHandle<
            std::result::Result<cluster_rpc::MetricsWalFileResponse, DataFusionError>,
        > = tokio::task::spawn(
            async move {
                let org_id: MetadataValue<_> = org_id
                    .parse()
                    .map_err(|_| DataFusionError::Execution("invalid org_id".to_string()))?;
                let mut request = tonic::Request::new(req);
                // request.set_timeout(Duration::from_secs(CONFIG.grpc.timeout));

                opentelemetry::global::get_text_map_propagator(|propagator| {
                    propagator.inject_context(
                        &tracing::Span::current().context(),
                        &mut MetadataMap(request.metadata_mut()),
                    )
                });

                let token: MetadataValue<_> = get_internal_grpc_token()
                    .parse()
                    .map_err(|_| DataFusionError::Execution("invalid token".to_string()))?;
                let channel = Channel::from_shared(node_addr)
                    .unwrap()
                    .connect()
                    .await
                    .map_err(|_| {
                        DataFusionError::Execution("connect search node error".to_string())
                    })?;
                let mut client = cluster_rpc::metrics_client::MetricsClient::with_interceptor(
                    channel,
                    move |mut req: Request<()>| {
                        req.metadata_mut().insert("authorization", token.clone());
                        req.metadata_mut()
                            .insert(CONFIG.grpc.org_header_key.as_str(), org_id.clone());
                        Ok(req)
                    },
                );
                client = client
                    .send_compressed(CompressionEncoding::Gzip)
                    .accept_compressed(CompressionEncoding::Gzip);
                let response: cluster_rpc::MetricsWalFileResponse =
                    match client.wal_file(request).await {
                        Ok(response) => response.into_inner(),
                        Err(err) => {
                            log::error!("[session_id {session_id}] get wal file list from search node error: {}", err);
                            return Err(DataFusionError::Execution(
                                "get wal file list from search node error".to_string(),
                            ));
                        }
                    };
                Ok(response)
            }
            .instrument(grpc_span),
        );
        tasks.push(task);
    }

    let mut results: Vec<cluster_rpc::MetricsWalFile> = Vec::new();
    let task_results = match try_join_all(tasks).await {
        Ok(res) => res,
        Err(err) => {
            return Err(DataFusionError::Execution(format!(
                "get wal file list from search node error: {}",
                err
            )));
        }
    };
    for task_result in task_results {
        results.extend(task_result?.files);
    }

    Ok(results)
}

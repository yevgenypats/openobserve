use std::collections::HashMap;

use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes};
use chrono::{DateTime, Duration, TimeZone, Utc};
use tokio::sync::Semaphore;

use crate::common::infra::config::CONFIG;
use crate::common::infra::db::dynamo_db::DYNAMO_DB_CLIENT;
use crate::common::meta::common::FileKey;
use crate::common::meta::StreamType;
use crate::service::db;

pub async fn run_stats() -> Result<(), anyhow::Error> {
    let semaphore = std::sync::Arc::new(Semaphore::new(CONFIG.limit.file_move_thread_num));
    let orgs = db::schema::list_organizations_from_cache();
    let stream_types = [
        StreamType::Logs,
        StreamType::Metrics,
        StreamType::Traces,
        StreamType::EnrichmentTables,
    ];
    let mut tasks = Vec::with_capacity(orgs.len());
    for org_id in orgs {
        let mut stream_list = vec![];
        for stream_type in stream_types {
            let streams = db::schema::list_streams_from_cache(&org_id, stream_type);

            for stream_name in streams {
                // check if we are allowed to merge or just skip
                if db::compact::retention::is_deleting_stream(
                    &org_id,
                    &stream_name,
                    stream_type,
                    None,
                ) {
                    log::info!(
                        "[STATS] the stream [{}/{}/{}] is deleting, just skip",
                        &org_id,
                        stream_type,
                        &stream_name,
                    );
                    continue;
                }
                stream_list.push(format!("{}/{}/{}", &org_id, stream_type, &stream_name));
            }
        }
        let task = tokio::task::spawn(async move {
            stream_list = vec![];
            stream_list.push("default/logs/default".to_string());
            if let Err(e) = batch_read(&stream_list).await {
                log::error!("[STATS] batch_read error: {}", e);
            }
        });
        tasks.push(task);
    }
    Ok(())
}

pub async fn batch_read(streams: &[String]) -> Result<Vec<FileKey>, anyhow::Error> {
    for batch in streams.chunks(25) {
        let mut keys: HashMap<String, AttributeValue> = HashMap::new();
        for stream in batch {
            keys.insert("stream".to_string(), AttributeValue::S(stream.to_string()));
        }
        let keys_and_attributes = KeysAndAttributes::builder().keys(keys).build();

        match DYNAMO_DB_CLIENT
            .get()
            .await
            .batch_get_item()
            .request_items(
                CONFIG.common.dynamo_stats_table.clone(),
                keys_and_attributes,
            )
            .send()
            .await
        {
            Ok(resp) => {
                let items = resp.responses().unwrap();
                let mut stats_local: Vec<HashMap<String, AttributeValue>> = Vec::new();

                for values in items.values() {
                    stats_local.extend(values.clone());
                }
                for item in &stats_local {
                    println!("{:?}", FileKey::from(item));
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    Ok(vec![])
}

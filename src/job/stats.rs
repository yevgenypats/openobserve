use crate::{common::infra::cluster, service};
use tokio::{sync::Semaphore, task, time};

pub async fn run() -> Result<(), anyhow::Error> {
    if !cluster::is_ingester(&cluster::LOCAL_NODE_ROLE) {
        return Ok(()); // not an ingester, no need to init job
    }
    // TODO do we need env variable??
    let mut interval = time::interval(time::Duration::from_secs(30));
    interval.tick().await; // trigger the first run
    loop {
        interval.tick().await;
        let ret = service::db::dynamo::stats::run_stats().await;
        if ret.is_err() {
            log::error!("[ALERT MANAGER] run error: {}", ret.err().unwrap());
        }
    }
}

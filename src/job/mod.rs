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

use regex::Regex;

use crate::{
    common::{
        infra::{
            cluster,
            config::{CONFIG, INSTANCE_ID, SYSLOG_ENABLED},
            file_list as infra_file_list, ider,
        },
        meta::{organization::DEFAULT_ORG, user::UserRequest},
        utils::file::clean_empty_dirs,
    },
    service::{compact::stats::update_stats_from_file_list, db, users},
};

mod alert_manager;
mod compact;
pub(crate) mod file_list;
pub(crate) mod files;
mod metrics;
mod mmdb_downloader;
mod prom;
mod stats;
pub(crate) mod syslog_server;
mod telemetry;

pub async fn init() -> Result<(), anyhow::Error> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.-]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .expect("Email regex is valid");

    // init root user
    if !db::user::root_user_exists().await {
        if CONFIG.auth.root_user_email.is_empty()
            || !email_regex.is_match(&CONFIG.auth.root_user_email)
            || CONFIG.auth.root_user_password.is_empty()
        {
            panic!(
                "Please set root user email-id & password using ZO_ROOT_USER_EMAIL & ZO_ROOT_USER_PASSWORD environment variables. This can also indicate an invalid email ID. Email ID must comply with ([a-z0-9_+]([a-z0-9_+.-]*[a-z0-9_+])?)@([a-z0-9]+([\\-\\.]{{1}}[a-z0-9]+)*\\.[a-z]{{2,6}})"
            );
        }
        let _ = users::post_user(
            DEFAULT_ORG,
            UserRequest {
                email: CONFIG.auth.root_user_email.clone(),
                password: CONFIG.auth.root_user_password.clone(),
                role: crate::common::meta::user::UserRole::Root,
                first_name: "root".to_owned(),
                last_name: "".to_owned(),
                is_external: false,
            },
        )
        .await;
    }

    if !CONFIG.common.mmdb_disable_download {
        // Try to download the mmdb files, if its not disabled.
        tokio::task::spawn(async move { mmdb_downloader::run().await });
    }
    // cache users
    tokio::task::spawn(async move { db::user::watch().await });
    db::user::cache().await.expect("user cache failed");

    db::organization::cache()
        .await
        .expect("organization cache sync failed");

    // set instance id
    let instance_id = match db::get_instance().await {
        Ok(Some(instance)) => instance,
        Ok(None) | Err(_) => {
            let id = ider::generate();
            let _ = db::set_instance(&id).await;
            id
        }
    };
    INSTANCE_ID.insert("instance_id".to_owned(), instance_id);

    // check version
    db::version::set().await.expect("db version set failed");

    // Router doesn't need to initialize job
    if cluster::is_router(&cluster::LOCAL_NODE_ROLE) {
        return Ok(());
    }

    // telemetry run
    if CONFIG.common.telemetry_enabled {
        tokio::task::spawn(async move { telemetry::run().await });
    }

    // initialize metadata watcher
    tokio::task::spawn(async move { db::schema::watch().await });
    tokio::task::spawn(async move { db::functions::watch().await });
    tokio::task::spawn(async move { db::compact::retention::watch().await });
    tokio::task::spawn(async move { db::metrics::watch_prom_cluster_leader().await });
    tokio::task::spawn(async move { db::alerts::templates::watch().await });
    tokio::task::spawn(async move { db::alerts::destinations::watch().await });
    tokio::task::spawn(async move { db::alerts::watch().await });
    tokio::task::spawn(async move { db::alerts::triggers::watch().await });
    tokio::task::spawn(async move { db::organization::watch().await });
    tokio::task::yield_now().await; // yield let other tasks run

    // cache core metadata
    db::schema::cache().await.expect("stream cache failed");
    db::functions::cache()
        .await
        .expect("functions cache failed");
    db::compact::retention::cache()
        .await
        .expect("compact delete cache failed");
    db::metrics::cache_prom_cluster_leader()
        .await
        .expect("prom cluster leader cache failed");

    // cache alerts
    db::alerts::templates::cache()
        .await
        .expect("alerts templates cache failed");
    db::alerts::destinations::cache()
        .await
        .expect("alerts destinations cache failed");
    db::alerts::cache().await.expect("alerts cache failed");
    db::alerts::triggers::cache()
        .await
        .expect("alerts triggers cache failed");
    db::syslog::cache().await.expect("syslog cache failed");
    db::syslog::cache_syslog_settings()
        .await
        .expect("syslog settings cache failed");

    // cache file list
    if !CONFIG.common.meta_store_external {
        if cluster::is_ingester(&cluster::LOCAL_NODE_ROLE) {
            // load the wal file_list into memory
            db::file_list::local::load_wal_in_cache()
                .await
                .expect("load wal file list failed");
        }
        if cluster::is_querier(&cluster::LOCAL_NODE_ROLE)
            || cluster::is_compactor(&cluster::LOCAL_NODE_ROLE)
        {
            db::file_list::remote::cache("", false)
                .await
                .expect("file list remote cache failed");
            infra_file_list::create_table_index()
                .await
                .expect("file list create table index failed");
            infra_file_list::set_initialised()
                .await
                .expect("file list set initialised failed");
            update_stats_from_file_list()
                .await
                .expect("file list remote calculate stats failed");
        }
    }

    infra_file_list::create_table_index().await?;
    infra_file_list::set_initialised().await?;
    db::file_list::remote::cache_stats()
        .await
        .expect("Load stream stats failed");

    // check wal directory
    if cluster::is_ingester(&cluster::LOCAL_NODE_ROLE) {
        // create wal dir
        if let Err(e) = std::fs::create_dir_all(&CONFIG.common.data_wal_dir) {
            log::error!("Failed to create wal dir: {}", e);
        }
        // clean empty sub dirs
        _ = clean_empty_dirs(&CONFIG.common.data_wal_dir);
    }

    tokio::task::spawn(async move { files::run().await });
    tokio::task::spawn(async move { file_list::run().await });
    tokio::task::spawn(async move { stats::run().await });
    tokio::task::spawn(async move { compact::run().await });
    tokio::task::spawn(async move { metrics::run().await });
    tokio::task::spawn(async move { prom::run().await });
    tokio::task::spawn(async move { alert_manager::run().await });

    // Shouldn't serve request until initialization finishes
    log::info!("Job initialization complete");

    // Syslog server start
    tokio::task::spawn(async move { db::syslog::watch().await });
    tokio::task::spawn(async move { db::syslog::watch_syslog_settings().await });

    let start_syslog = *SYSLOG_ENABLED.read();
    if start_syslog {
        syslog_server::run(start_syslog, true)
            .await
            .expect("syslog server run failed");
    }

    Ok(())
}

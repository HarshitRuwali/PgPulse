use crate::collectors::{queries, replication};
use crate::guc;
use crate::health::evaluator;
use crate::models::MetricSnapshot;
use crate::shared_mem;
use pgrx::bgworkers::{BackgroundWorker, BackgroundWorkerBuilder, SignalWakeFlags};
use pgrx::prelude::*;
use std::time::Duration;

pub fn init() {
    BackgroundWorkerBuilder::new("postgres monitoring worker")
        .set_function("pgpulse_worker_main")
        .set_library("pgpulse")
        .enable_spi_access()
        .set_restart_time(Some(Duration::from_secs(10))) // Restart after 10 seconds if it crashes
        .load();
}

pub fn collect_and_store_metrics() -> anyhow::Result<MetricSnapshot> {
    let (replication_clients, replication_client_count) =
        replication::collect_replication_clients()
            .map_err(|e| anyhow::anyhow!("SPI error collecting replication clients: {e:?}"))?;

    let long_running_queries = queries::get_long_running_queries()
        .map_err(|e| anyhow::anyhow!("SPI error collecting long-running queries: {e:?}"))?;

    let replica_replay_lag_seconds = replication::collect_replica_time_lag();

    let mut snapshot = MetricSnapshot {
        replication_clients,
        replica_replay_lag_seconds,
        long_running_queries,
        collected_at: chrono::Utc::now().timestamp(),
        health_status: crate::models::HealthStatus::default(),
    };

    snapshot.health_status = evaluator::evaluate_health(&snapshot);

    Ok(snapshot)
}

#[pg_guard]
pub extern "C-unwind" fn pgpulse_worker_main(_arg: pg_sys::Datum) {
    // do bgworker stuff here
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);

    // Connect to SPI (Specify the database name, and optionally the user)
    BackgroundWorker::connect_worker_to_spi(Some("postgres"), None);

    // pgpulse background worker started
    loop {
        if BackgroundWorker::sighup_received() {
            // reload configuration
            // No need to re-init the GUCs again
        }

        let interval = Duration::from_secs(guc::POLL_INTERVAL_SECONDS.get() as u64);
        let wake = BackgroundWorker::wait_latch(Some(interval));

        if !wake {
            // received SIGTERM or other shutdown signal, time to exit
            break;
        }

        // collect metrics and store in shared memory which will be read by the exporter

        match collect_and_store_metrics() {
            Ok(snapshot) => {
                shared_mem::write_snapshot(snapshot);
            } // write the snapshot to shared memory which will be read by the exporter
            Err(e) => {
                // Log the error but keep the worker running
                warning!("Error collecting/storing metrics: {:?}", e);
            }
        }
    }
}

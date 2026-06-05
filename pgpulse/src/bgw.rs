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
    // SPI requires an active transaction in a BGW
    BackgroundWorker::transaction(|| {
        warning!("pgpulse: collecting replication clients");
        let (replication_clients, _replication_client_count) =
            replication::collect_replication_clients()
                .map_err(|e| anyhow::anyhow!("SPI error collecting replication clients: {e:?}"))?;

        warning!("pgpulse: collecting long-running queries");
        let long_running_queries = queries::get_long_running_queries()
            .map_err(|e| anyhow::anyhow!("SPI error collecting long-running queries: {e:?}"))?;

        warning!("pgpulse: collecting replica lag");
        let replica_replay_lag_seconds = replication::collect_replica_time_lag();

        warning!("pgpulse: building snapshot");
        let mut snapshot = MetricSnapshot {
            replication_clients,
            replica_replay_lag_seconds,
            long_running_queries,
            collected_at: chrono::Utc::now().timestamp(),
            health_status: crate::models::HealthStatus::default(),
        };

        snapshot.health_status = evaluator::evaluate_health(&snapshot);

        Ok(snapshot)
    })
}

#[pg_guard]
#[no_mangle]
pub extern "C-unwind" fn pgpulse_worker_main(_arg: pg_sys::Datum) {
    // do bgworker stuff here
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);

    // Connect to SPI (Specify the database name, and optionally the user)
    BackgroundWorker::connect_worker_to_spi(Some("postgres"), None);

    warning!("pgpulse: background worker started, entering poll loop");
    // pgpulse background worker started
    loop {
        if BackgroundWorker::sighup_received() {
            // reload configuration
            // No need to re-init the GUCs again
        }

        let interval = Duration::from_secs(guc::POLL_INTERVAL_SECONDS.get() as u64);
        warning!(
            "pgpulse: waiting for latch ({} seconds)",
            guc::POLL_INTERVAL_SECONDS.get()
        );
        let alive = BackgroundWorker::wait_latch(Some(interval));
        warning!("pgpulse: latch returned alive={}", alive);

        if !alive {
            // postmaster died or SIGTERM — time to exit
            break;
        }

        // collect metrics and store in shared memory which will be read by the exporter

        match collect_and_store_metrics() {
            Ok(snapshot) => {
                warning!("pgpulse: writing snapshot to shared memory");
                shared_mem::write_snapshot(snapshot);
                warning!("pgpulse: snapshot written successfully");
            } // write the snapshot to shared memory which will be read by the exporter
            Err(e) => {
                // Log the error but keep the worker running
                warning!("Error collecting/storing metrics: {:?}", e);
            }
        }
    }
}

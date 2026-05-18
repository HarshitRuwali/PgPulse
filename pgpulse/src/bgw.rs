use crate::collectors::{queries, replication, wal};
use crate::guc;
use crate::health::evaluator;
use crate::models::MetricSnapshot;
use crate::shared_mem;
use pgrx::bgworkers::{BackgroundWorker, BackgroundWorkerBuilder, SignalWakeFlags};
use pgrx::prelude::*;
use std::time::Duration;

pub fn init() {
    BackgroundWorkerBuilder::new("postgres monitoring worker")
        .set_function("pgpulse__worker_main")
        .set_library("pgpulse")
        .enable_spi_access()
        .set_restart_time(Some(Duration::from_secs(10))) // Restart after 10 seconds if it crashes
        .load();
}

pub fn collect_and_store_metrics() -> anyhow::Result<MetricSnapshot> {
    let replication_metrics = replication::collect_replica_metrics()?;
    let primary_metrics = wal::collect_primary_metrics()?;
    let health_status = evaluator::evaluate_health(&replication_metrics, &primary_metrics);
    let long_running_queries = queries::get_long_running_queries()?;

    Ok(MetricSnapshot {
        replication_metrics,
        primary_metrics,
        health_status,
        collected_at: chrono::Utc::now(),
        long_running_queries,
    })
}

#[pg_guard]
pub extern "C-unwind" fn pgpulse__worker_main(_arg: pg_sys::Datum) {
    // do bgworker stuff here
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);

    // pgpulse background worker started
    loop {
        if BackgroundWorker::sighup_received() {
            // reload configuration
            guc::init();
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

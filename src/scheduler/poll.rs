use crate::collectors::{replication::collect_replica_metrics, wal::collect_primary_metrics};
use crate::config::Config;
use crate::db::{primary, replica};
use crate::health::evaluator;
use crate::models::MetricSnapshot;
use crate::storage::in_memory::MetricStore;
use std::time::Duration;
use tokio;
use tokio::time::interval;

pub async fn poll_and_update_snapshot(config: Config, metric_store: MetricStore) {
    let primary_client = primary::connect(&config.primary).await.unwrap();
    let replica_client = replica::connect(&config.replica).await.unwrap();
    let metric_store = metric_store.clone();

    tokio::spawn(async move {
        let interval_duration = Duration::from_secs(config.polling.interval_seconds);
        let mut ticker = interval(interval_duration);
        loop {
            ticker.tick().await;

            let primary_metrics = collect_primary_metrics(&primary_client).await.unwrap();
            let replia_metrics = collect_replica_metrics(&replica_client).await.unwrap();
            let health_status = evaluator::evaluate_health(&replia_metrics, &config.threshold);

            let snapshot = MetricSnapshot {
                replication_metrics: replia_metrics,
                primary_metrics,
                health_status,
                collected_at: chrono::Utc::now(),
            };
            metric_store.update_snapshot(snapshot).await;
        }
    });
}

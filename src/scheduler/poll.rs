use crate::collectors::queries;
use crate::collectors::{
    queries::get_long_running_queries, replication::collect_replica_metrics,
    wal::collect_primary_metrics,
};
use crate::config::Config;
use crate::db::{primary, replica};
use crate::health::evaluator;
use crate::models::MetricSnapshot;
use crate::storage::in_memory::MetricStore;
use std::time::Duration;
use tokio;
use tokio::time::interval;
use tracing::error;

pub async fn poll_and_update_snapshot(config: Config, metric_store: MetricStore) {
    let primary_client = match primary::connect(&config.primary).await {
        Ok(client) => client,
        Err(e) => {
            error!("Error connecting to primary daabase: {}", e);
            return;
        }
    };

    let replica_client = match replica::connect(&config.replica).await {
        Ok(client) => client,
        Err(e) => {
            error!("Error connecting to replica database: {}", e);
            return;
        }
    };

    let metric_store = metric_store.clone();

    tokio::spawn(async move {
        let interval_duration = Duration::from_secs(config.polling.interval_seconds);
        let mut ticker = interval(interval_duration);
        loop {
            ticker.tick().await;

            let primary_metrics = match collect_primary_metrics(&primary_client).await {
                Ok(metrics) => metrics,
                Err(e) => {
                    error!("Failed to collect primary metrics: {}", e);
                    continue;
                }
            };

            let replica_metrics = match collect_replica_metrics(&replica_client).await {
                Ok(metrics) => metrics,
                Err(e) => {
                    error!("Failed to collect replica metrics: {}", e);
                    continue;
                }
            };

            let health_status =
                evaluator::evaluate_health(&replica_metrics, &primary_metrics, &config.threshold);

            let long_queries = match get_long_running_queries(
                &replica_client,
                config.polling.long_query_threshold_seconds,
            )
            .await
            {
                Ok(queries) => queries,
                Err(e) => {
                    error!("Failred to collect long running queries: {}", e);
                    continue;
                }
            };

            let snapshot = MetricSnapshot {
                replication_metrics: replica_metrics,
                primary_metrics,
                health_status,
                long_running_queries: long_queries,
                collected_at: chrono::Utc::now(),
            };
            metric_store.update_snapshot(snapshot).await;
        }
    });
}

mod api;
mod collectors;
mod config;
mod db;
mod health;
mod models;
mod storage;

use crate::collectors::{replication::collect_replica_metrics, wal::collect_primary_metrics};
use crate::storage::in_memory::MetricStore;
use clap::Parser;
use health::evaluator::evaluate_health;
use models::MetricSnapshot;
use tokio;
use tracing::info;
use tracing_subscriber;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting PgPulse...");
    let cli = Cli::parse();
    let config = config::load_config(&cli.config)?;
    info!("Config loaded!");
    info!(
        "Checking if config is loaded, ex: host: {}",
        config.server.host
    );

    let primary_client = db::primary::connect(&config.primary).await?;
    let replica_client = db::replica::connect(&config.replica).await?;

    let replia_metrics = collect_replica_metrics(&replica_client).await?;
    info!("Replication Metrics: {:?}", replia_metrics);
    let primary_metrics = collect_primary_metrics(&primary_client).await?;
    info!("Primary Metrics: {:?}", primary_metrics);

    // check the health status between primary and replica
    let health_status = evaluate_health(&replia_metrics, &config.threshold);
    info!("Health Status: {:?}", health_status);

    let snapshot = MetricSnapshot {
        replication_metrics: replia_metrics,
        primary_metrics,
        health_status,
        collected_at: chrono::Utc::now(),
    };

    // poll and update the snapshot periodically
    let metric_store = MetricStore::new();
    metric_store.update_snapshot(snapshot).await;

    // API server
    let app = api::create_router(metric_store);
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

mod api;
mod collectors;
mod config;
mod db;
mod health;
mod models;
mod scheduler;
mod storage;

use crate::storage::in_memory::MetricStore;
use clap::Parser;
use scheduler::poll::poll_and_update_snapshot;
use tracing::info;

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

    let metric_store = MetricStore::new();

    // poll and update the snapshot periodically
    poll_and_update_snapshot(config.clone(), metric_store.clone()).await;

    // API server
    let app = api::create_router(metric_store);
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

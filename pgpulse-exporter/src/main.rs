mod storage;
use crate::storage::metrics;
use std::sync::Arc;
use tracing::info;
mod api;
mod config;
use clap::Parser;
use tokio_postgres::NoTls;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting pgpulse-exporter...");

    let cli = Cli::parse();
    let config = config::load_config(&cli.config)?;
    info!("Config loaded!");

    if config.primary.ssl_enabled {
        anyhow::bail!(
            "SSL is enabled for primary connection, but SSL support is not implemented in pgpulse-exporter yet. Please disable SSL or implement SSL support."
        );
    }
    let ssl_mode = "disable";

    let pg_url = format!(
        "host={} port={} dbname={} user={} password={} sslmode={}",
        config.primary.host,
        config.primary.port,
        config.primary.name,
        config.primary.user,
        config.primary.password,
        ssl_mode,
    );

    let (client, connection) = tokio_postgres::connect(&pg_url, NoTls).await?;

    // Drive the connection in a background task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("postgres connection error: {e}");
        }
    });

    let metrics = Arc::new(metrics::Metrics::new()?);
    let client = Arc::new(client);

    let app = api::create_router(metrics.clone(), client.clone());
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
            .await?;

    info!(
        "pgpulse-exporter listening on {}:{}",
        config.server.host, config.server.port
    );

    axum::serve(listener, app).await?;

    Ok(())
}

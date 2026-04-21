mod config;

use clap::Parser;
use tokio;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("PgPulse starting...");
    let cli = Cli::parse();
    let config = config::load_config(&cli.config)?;
    println!("config loaded!");
    println!("Checking if config is loaded, ex: host: {}", config.server.host);
    Ok(())
}

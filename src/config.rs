use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub ssl_enabled: bool,
}

#[derive(Deserialize, Clone)]
pub struct PollingConfig {
    pub interval_seconds: u64,
    pub long_query_threshold_seconds: u64,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct ThresholdConfig {
    pub replay_lag_warning_seconds: u64,
    pub replay_lag_critical_seconds: u64,
    pub lsn_gap_warning_bytes: u64,
    pub lsn_gap_critical_bytes: u64,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub polling: PollingConfig,
    pub primary: DbConfig,
    pub replica: DbConfig,
    pub server: ServerConfig,
    pub threshold: ThresholdConfig,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let file_content = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&file_content)?;
    Ok(config)
}

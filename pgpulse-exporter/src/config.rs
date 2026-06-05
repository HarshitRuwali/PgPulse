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
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub primary: DbConfig,
    pub server: ServerConfig,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let file_content = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&file_content)?;
    Ok(config)
}

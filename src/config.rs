use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub debug: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> anyhow::Result<Config> {
    let toml_str = fs::read_to_string("Config.toml")?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}

use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub fhir: FhirConfig,
    #[serde(default)]
    pub sync: SyncConfig,
    pub debug: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    #[serde(default = "default_schema")]
    pub schema: String,
    /// Must be non-zero and distinct from the source server's (Oscar runs
    /// `--server-id=1`, E2) — zero causes the master to disconnect after
    /// the last available event (F12).
    #[serde(default = "default_server_id")]
    pub server_id: u64,
}

fn default_schema() -> String {
    "oscar".to_string()
}

fn default_server_id() -> u64 {
    4321
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_health_port")]
    pub health_port: u16,
    #[serde(default = "default_webhook_port")]
    pub webhook_port: u16,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            health_port: default_health_port(),
            webhook_port: default_webhook_port(),
            grpc_port: default_grpc_port(),
        }
    }
}

fn default_health_port() -> u16 {
    8080
}

fn default_webhook_port() -> u16 {
    8081
}

fn default_grpc_port() -> u16 {
    50051
}

#[derive(Debug, Clone, Deserialize)]
pub struct FhirConfig {
    #[serde(default = "default_fhir_base_url")]
    pub base_url: String,
    #[serde(default = "default_oscar_demographic_system")]
    pub oscar_demographic_system: String,
    #[serde(default = "default_oscar_hin_system")]
    pub oscar_hin_system: String,
    pub token_env: Option<String>,
}

impl Default for FhirConfig {
    fn default() -> Self {
        Self {
            base_url: default_fhir_base_url(),
            oscar_demographic_system: default_oscar_demographic_system(),
            oscar_hin_system: default_oscar_hin_system(),
            token_env: None,
        }
    }
}

fn default_fhir_base_url() -> String {
    "http://localhost:8082/fhir".to_string()
}

fn default_oscar_demographic_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-demographic-no".to_string()
}

fn default_oscar_hin_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-hin".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct SyncConfig {
    #[serde(default = "default_checkpoint_path")]
    pub checkpoint_path: String,
    #[serde(default = "default_retry_max_attempts")]
    pub retry_max_attempts: u32,
    #[serde(default = "default_retry_base_ms")]
    pub retry_base_ms: u64,
    #[serde(default = "default_dead_letter_path")]
    pub dead_letter_path: String,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            checkpoint_path: default_checkpoint_path(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            dead_letter_path: default_dead_letter_path(),
        }
    }
}

fn default_checkpoint_path() -> String {
    "/var/lib/fhir-sync/checkpoint.json".to_string()
}

fn default_dead_letter_path() -> String {
    "/var/lib/fhir-sync/dead_letter.jsonl".to_string()
}

fn default_retry_max_attempts() -> u32 {
    5
}

fn default_retry_base_ms() -> u64 {
    500
}

/// Loads `Config.toml` from `CONFIG_PATH` if set, otherwise from the CWD.
pub fn load_config() -> anyhow::Result<Config> {
    let path = env::var("CONFIG_PATH").unwrap_or_else(|_| "Config.toml".to_string());
    let toml_str = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("failed to read config at {path}: {e}"))?;
    let mut config: Config = toml::from_str(&toml_str)?;

    if let Ok(v) = env::var("FHIR_SYNC_HEALTH_PORT") {
        config.server.health_port = v.parse()?;
    }
    if let Ok(v) = env::var("FHIR_SYNC_WEBHOOK_PORT") {
        config.server.webhook_port = v.parse()?;
    }
    if let Ok(v) = env::var("FHIR_SYNC_GRPC_PORT") {
        config.server.grpc_port = v.parse()?;
    }

    Ok(config)
}

use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub fhir: FhirConfig,
    #[serde(default)]
    pub sync: SyncConfig,
    pub debug: Option<bool>,
    #[serde(default)]
    pub replication: ReplicationConfig,
    /// Oscar CDC pipeline (binlog source + sink + backfill). Off for
    /// replication-only deployments, which have no MySQL to tail.
    #[serde(default = "default_true")]
    pub oscar_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
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

fn default_true() -> bool {
    true
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

#[derive(Debug, Clone, Deserialize)]
pub struct ReplicationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_state_dir")]
    pub state_dir: String,
    #[serde(default = "default_poll_interval_ms")]
    pub poll_interval_ms: u64,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    #[serde(default)]
    pub doorbell_port: u16,
    #[serde(default)]
    pub nodes: Vec<ReplicationNode>,
    #[serde(default)]
    pub links: Vec<ReplicationLink>,
}

impl Default for ReplicationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            state_dir: default_state_dir(),
            poll_interval_ms: default_poll_interval_ms(),
            page_size: default_page_size(),
            doorbell_port: default_doorbell_port(),
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }
}

fn default_state_dir() -> String {
    "/var/lib/fhir-sync".to_string()
}

fn default_poll_interval_ms() -> u64 {
    5000
}

fn default_page_size() -> usize {
    100
}

fn default_doorbell_port() -> u16 {
    8082
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReplicationNode {
    pub name: String,
    pub base_url: String,
    pub token_env: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReplicationLink {
    pub name: String,
    pub source: String,
    pub target: String,
    #[serde(default = "default_link_mode")]
    pub mode: ReplicationMode,
    pub resources: Vec<String>,
    #[serde(default = "default_link_provenance")]
    pub provenance: bool,
    #[serde(default = "default_link_conflict_policy")]
    pub conflict_policy: ConflictPolicy,
    pub federate_identifier_system: Option<String>,
    #[serde(default)]
    pub subscription_doorbell: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReplicationMode {
    Mirror,
    Federate,
}

impl Default for ReplicationMode {
    fn default() -> Self {
        ReplicationMode::Mirror
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictPolicy {
    DeadLetter,
    SourceWins,
}

impl Default for ConflictPolicy {
    fn default() -> Self {
        ConflictPolicy::DeadLetter
    }
}

fn default_link_mode() -> ReplicationMode {
    ReplicationMode::Mirror
}

fn default_link_provenance() -> bool {
    true
}

fn default_link_conflict_policy() -> ConflictPolicy {
    ConflictPolicy::DeadLetter
}

/// Validates the replication configuration (node references, duplicates,
/// federate requirements, forbidden resource types). Fatal on error.
pub fn validate_replication(cfg: &ReplicationConfig) -> anyhow::Result<()> {
    if !cfg.enabled {
        return Ok(());
    }

    let mut names = std::collections::HashSet::new();
    for node in &cfg.nodes {
        if !names.insert(node.name.clone()) {
            anyhow::bail!("replication node name '{}' is duplicated", node.name);
        }
    }

    let mut link_names = std::collections::HashSet::new();
    for link in &cfg.links {
        if !link_names.insert(link.name.clone()) {
            anyhow::bail!("replication link name '{}' is duplicated", link.name);
        }
        if !names.contains(&link.source) {
            anyhow::bail!("replication link '{}' references unknown source node '{}'", link.name, link.source);
        }
        if !names.contains(&link.target) {
            anyhow::bail!("replication link '{}' references unknown target node '{}'", link.name, link.target);
        }
        if matches!(link.mode, ReplicationMode::Federate) && link.federate_identifier_system.is_none() {
            anyhow::bail!("replication link '{}' uses federate mode but has no federate_identifier_system", link.name);
        }
        if link
            .resources
            .iter()
            .any(|r| r.eq_ignore_ascii_case("Provenance") || r.eq_ignore_ascii_case("AuditEvent"))
        {
            anyhow::bail!("replication link '{}' may not include Provenance or AuditEvent in resources", link.name);
        }
    }

    Ok(())
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

    validate_replication(&config.replication)?;

    Ok(config)
}

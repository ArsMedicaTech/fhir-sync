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
    #[serde(default)]
    pub dispatch: DispatchConfig,
    /// Oscar CDC pipeline (binlog source + sink + backfill). Off for
    /// replication-only deployments, which have no MySQL to tail.
    #[serde(default = "default_true")]
    pub oscar_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
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
    #[serde(default)]
    pub keycloak: Option<KeycloakConfig>,
}

impl Default for FhirConfig {
    fn default() -> Self {
        Self {
            base_url: default_fhir_base_url(),
            oscar_demographic_system: default_oscar_demographic_system(),
            oscar_hin_system: default_oscar_hin_system(),
            token_env: None,
            keycloak: None,
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
pub struct KeycloakConfig {
    pub token_url: String,
    pub client_id: String,
    pub client_secret_env: String,
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

const KNOWN_DISPATCH_RESOURCE_TYPES: &[&str] = &["Patient"];
const KNOWN_DISPATCH_OPS: &[&str] = &["upsert", "delete"];

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_dispatch_dead_letter_dir")]
    pub dead_letter_dir: String,
    #[serde(default = "default_retry_max_attempts")]
    pub retry_max_attempts: u32,
    #[serde(default = "default_retry_base_ms")]
    pub retry_base_ms: u64,
    #[serde(default = "default_dispatch_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub consumers: Vec<DispatchConsumer>,
}

impl Default for DispatchConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            dead_letter_dir: default_dispatch_dead_letter_dir(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            timeout_ms: default_dispatch_timeout_ms(),
            consumers: Vec::new(),
        }
    }
}

fn default_dispatch_dead_letter_dir() -> String {
    "/var/lib/fhir-sync/dispatch-dlq".to_string()
}

fn default_dispatch_timeout_ms() -> u64 {
    10000
}

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchConsumer {
    pub name: String,
    pub url: String,
    pub secret_env: String,
    #[serde(default)]
    pub resource_types: Vec<String>,
    #[serde(default)]
    pub ops: Vec<String>,
    #[serde(default)]
    pub enabled: bool,
}

/// Validates the dispatch configuration. Fatal on error.
pub fn validate_dispatch(cfg: &DispatchConfig) -> anyhow::Result<()> {
    if !cfg.enabled {
        return Ok(());
    }

    let mut names = std::collections::HashSet::new();
    for consumer in &cfg.consumers {
        if !consumer.enabled {
            continue;
        }

        if consumer.name.trim().is_empty() {
            anyhow::bail!("dispatch consumer has an empty name");
        }
        if !names.insert(consumer.name.clone()) {
            anyhow::bail!("dispatch consumer name '{}' is duplicated", consumer.name);
        }

        let url = url::Url::parse(&consumer.url)
            .map_err(|e| anyhow::anyhow!("dispatch consumer '{}' url '{}' is invalid: {e}", consumer.name, consumer.url))?;
        if url.scheme() != "https"
            && !(url.scheme() == "http"
                && matches!(url.host_str(), Some("localhost") | Some("127.0.0.1")))
        {
            anyhow::bail!(
                "dispatch consumer '{}' url '{}' must use https (or http to localhost/127.0.0.1)",
                consumer.name,
                consumer.url
            );
        }

        if consumer.secret_env.trim().is_empty() {
            anyhow::bail!("dispatch consumer '{}' has an empty secret_env", consumer.name);
        }
        if std::env::var(&consumer.secret_env).is_err() {
            anyhow::bail!(
                "dispatch consumer '{}' secret_env '{}' is not set in the environment",
                consumer.name,
                consumer.secret_env
            );
        }

        if consumer.resource_types.is_empty() {
            anyhow::bail!("dispatch consumer '{}' has an empty resource_types list", consumer.name);
        }
        for rt in &consumer.resource_types {
            if !KNOWN_DISPATCH_RESOURCE_TYPES
                .iter()
                .any(|k| k.eq_ignore_ascii_case(rt))
            {
                anyhow::bail!(
                    "dispatch consumer '{}' has unknown resource_type '{}'",
                    consumer.name,
                    rt
                );
            }
        }

        if consumer.ops.is_empty() {
            anyhow::bail!("dispatch consumer '{}' has an empty ops list", consumer.name);
        }
        for op in &consumer.ops {
            if !KNOWN_DISPATCH_OPS.iter().any(|k| k.eq_ignore_ascii_case(op)) {
                anyhow::bail!("dispatch consumer '{}' has unknown op '{}'", consumer.name, op);
            }
        }
    }

    Ok(())
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
    validate_dispatch(&config.dispatch)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_consumer(name: &str, url: &str, secret_env: &str) -> DispatchConsumer {
        DispatchConsumer {
            name: name.to_string(),
            url: url.to_string(),
            secret_env: secret_env.to_string(),
            resource_types: vec!["Patient".to_string()],
            ops: vec!["upsert".to_string()],
            enabled: true,
        }
    }

    #[test]
    fn validate_dispatch_catches_duplicate_names() {
        let cfg = DispatchConfig {
            enabled: true,
            dead_letter_dir: default_dispatch_dead_letter_dir(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            timeout_ms: default_dispatch_timeout_ms(),
            consumers: vec![
                make_consumer("a", "https://example.invalid/1", "D_A"),
                make_consumer("a", "https://example.invalid/2", "D_A2"),
            ],
        };
        let err = validate_dispatch(&cfg).unwrap_err().to_string();
        assert!(err.contains("'a' is duplicated"));
    }

    #[test]
    fn validate_dispatch_catches_http_not_localhost() {
        let cfg = DispatchConfig {
            enabled: true,
            dead_letter_dir: default_dispatch_dead_letter_dir(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            timeout_ms: default_dispatch_timeout_ms(),
            consumers: vec![make_consumer(
                "a",
                "http://example.invalid/hook",
                "D_A",
            )],
        };
        let err = validate_dispatch(&cfg).unwrap_err().to_string();
        assert!(err.contains("must use https"));
    }

    #[test]
    fn validate_dispatch_allows_http_localhost() {
        std::env::set_var("D_LOCAL", "secret");
        let cfg = DispatchConfig {
            enabled: true,
            dead_letter_dir: default_dispatch_dead_letter_dir(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            timeout_ms: default_dispatch_timeout_ms(),
            consumers: vec![make_consumer("a", "http://localhost:3000/hook", "D_LOCAL")],
        };
        assert!(validate_dispatch(&cfg).is_ok());
        std::env::remove_var("D_LOCAL");
    }

    #[test]
    fn validate_dispatch_catches_missing_secret_env() {
        std::env::remove_var("D_MISSING");
        let cfg = DispatchConfig {
            enabled: true,
            dead_letter_dir: default_dispatch_dead_letter_dir(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            timeout_ms: default_dispatch_timeout_ms(),
            consumers: vec![make_consumer("a", "https://example.invalid/hook", "D_MISSING")],
        };
        let err = validate_dispatch(&cfg).unwrap_err().to_string();
        assert!(err.contains("not set"));
    }

    #[test]
    fn validate_dispatch_catches_unknown_resource_type() {
        std::env::set_var("D_X", "secret");
        let cfg = DispatchConfig {
            enabled: true,
            dead_letter_dir: default_dispatch_dead_letter_dir(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            timeout_ms: default_dispatch_timeout_ms(),
            consumers: vec![DispatchConsumer {
                name: "a".to_string(),
                url: "https://example.invalid/hook".to_string(),
                secret_env: "D_X".to_string(),
                resource_types: vec!["Foo".to_string()],
                ops: vec!["upsert".to_string()],
                enabled: true,
            }],
        };
        let err = validate_dispatch(&cfg).unwrap_err().to_string();
        assert!(err.contains("unknown resource_type"));
        std::env::remove_var("D_X");
    }

    #[test]
    fn validate_dispatch_disabled_is_noop() {
        let cfg = DispatchConfig::default();
        assert!(!cfg.enabled);
        assert!(validate_dispatch(&cfg).is_ok());
    }
}

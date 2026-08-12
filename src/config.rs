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
    /// Oscar-specific settings (timezone for appointment conversion, etc.).
    #[serde(default)]
    pub oscar: OscarConfig,
    /// AMT → Oscar write-back pipeline. Off by default; see
    /// `TASK_FEATURES_SPEC_OSCAR_WRITEBACK.md`.
    #[serde(default)]
    pub writeback: WritebackConfig,
}

use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct OscarConfig {
    /// IANA timezone name used to interpret Oscar's local date/time columns
    /// (appointments, provider schedules). See chrono-tz. Required when
    /// `oscar_enabled` is `true`.
    pub timezone: Option<String>,
    /// Optional region used to filter the `diagnosticcode` startup cache.
    #[serde(default)]
    pub region: Option<String>,
    /// Config overrides for `appointment.status` codes not covered by the
    /// stock mapping (the `a`–`e` custom codes). Maps the raw Oscar code to
    /// a FHIR `Appointment.status` value (e.g. "booked").
    #[serde(default = "default_appointment_status_map")]
    pub appointment_status_map: HashMap<String, String>,
    /// Fallback MRP when `demographic.provider_no` is null, empty, or `-1`.
    /// When omitted, patients with no usable MRP produce no CareTeam.
    #[serde(default)]
    pub default_mrp_provider_no: Option<String>,
    /// Default Oscar `appointment.location` for AMT-authored appointments.
    #[serde(default)]
    pub default_appointment_location: Option<String>,
    /// Default Oscar `appointment.type` for AMT-authored appointments.
    #[serde(default)]
    pub default_appointment_type: Option<String>,
    /// Master switch for the Oscar → FHIR CareTeam sync.
    #[serde(default = "default_true")]
    pub care_team_enabled: bool,
    /// Oscar `consultationResponse.status` → FHIR `DiagnosticReport.status`.
    /// Shipped empty intentionally; unmapped statuses dead-letter rather than
    /// defaulting.
    #[serde(default = "default_consult_response_status_map")]
    pub consult_response_status_map: HashMap<String, String>,
    /// Oscar `consultationRequests.status` -> FHIR `ServiceRequest.status`,
    /// for resyncing status changes made in Oscar's own consult UI back to
    /// AMT. Defaults reflect the four Status radio options in Oscar's
    /// ConsultationFormRequest.jsp (confirmed live: "Nothing"=1, "Pending
    /// Specialist Callback"=2, "Pending Patient Callback"=3,
    /// "Completed"=4). Override here if a given Oscar build's codes differ.
    #[serde(default = "default_consult_request_status_map")]
    pub consult_request_status_map: HashMap<String, String>,
    /// Oscar `admission.program_id` used when creating the CAISI program
    /// enrollment row required for a new AMT-authored patient to have
    /// program-domain access to Encounters and other gated modules.
    /// Required when `oscar_enabled` is true; there is no safe default
    /// since valid program IDs are clinic-specific.
    pub default_program_id: Option<String>,
}

impl Default for OscarConfig {
    fn default() -> Self {
        Self {
            timezone: None,
            region: None,
            appointment_status_map: default_appointment_status_map(),
            default_mrp_provider_no: None,
            default_appointment_location: None,
            default_appointment_type: None,
            care_team_enabled: true,
            consult_response_status_map: default_consult_response_status_map(),
            consult_request_status_map: default_consult_request_status_map(),
        }
    }
}

fn default_appointment_status_map() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("t".to_string(), "booked".to_string());
    m.insert("T".to_string(), "booked".to_string());
    m.insert("h".to_string(), "booked".to_string());
    m.insert("H".to_string(), "checked-in".to_string());
    m.insert("P".to_string(), "checked-in".to_string());
    m.insert("E".to_string(), "checked-in".to_string());
    m.insert("B".to_string(), "fulfilled".to_string());
    m.insert("C".to_string(), "cancelled".to_string());
    m.insert("N".to_string(), "noshow".to_string());
    m
}

fn default_consult_response_status_map() -> HashMap<String, String> {
    HashMap::new()
}

fn default_consult_request_status_map() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("1".to_string(), "active".to_string()); // Nothing (not yet actioned)
    m.insert("2".to_string(), "active".to_string()); // Pending Specialist Callback
    m.insert("3".to_string(), "on-hold".to_string()); // Pending Patient Callback
    m.insert("4".to_string(), "completed".to_string()); // Completed
    m
}

fn default_sentinel_update_user() -> String {
    "999998".to_string()
}

fn default_hapi_resource_types() -> Vec<String> {
    vec![
        "Patient".to_string(),
        "Appointment".to_string(),
        "Encounter".to_string(),
        "DocumentReference".to_string(),
    ]
}

fn default_writeback_port() -> u16 {
    3316
}

/// MariaDB connection used only by the AMT → Oscar write-back sink.
/// Kept separate from [database] because the sink needs a different grant set.
#[derive(Debug, Clone, Deserialize)]
pub struct WritebackDatabaseConfig {
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub host: String,
    #[serde(default = "default_schema")]
    pub schema: String,
    #[serde(default = "default_writeback_port")]
    pub port: u16,
}

impl Default for WritebackDatabaseConfig {
    fn default() -> Self {
        Self {
            user: String::new(),
            password: String::new(),
            host: String::new(),
            schema: default_schema(),
            port: default_writeback_port(),
        }
    }
}

/// AMT → Oscar write-back configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct WritebackConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_state_dir")]
    pub state_dir: String,
    #[serde(default = "default_poll_interval_ms")]
    pub poll_interval_ms: u64,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    #[serde(default = "default_retry_max_attempts")]
    pub retry_max_attempts: u32,
    #[serde(default = "default_retry_base_ms")]
    pub retry_base_ms: u64,
    #[serde(default = "default_dead_letter_path")]
    pub dead_letter_path: String,
    #[serde(default = "default_sentinel_update_user")]
    pub sentinel_update_user: String,
    #[serde(default = "default_hapi_resource_types")]
    pub hapi_resource_types: Vec<String>,
    /// FHIR `Appointment.status` → Oscar `status` char.
    /// Example: cancelled → "C".
    #[serde(default)]
    pub appointment_status_map: HashMap<String, String>,
    /// Fallback `providerNo` for `consultationRequests` when the FHIR
    /// `requester` reference does not resolve to an Oscar provider.
    #[serde(default)]
    pub default_consult_provider_no: Option<String>,
    /// FHIR `ServiceRequest.code` text (lowercased) → Oscar `serviceId`.
    #[serde(default)]
    pub consult_service_map: HashMap<String, String>,
    /// Oscar `consultationRequests.status` → FHIR `Task.status`.
    /// Reserved for Phase 3; unused in Phase 1.
    #[serde(default)]
    pub consult_status_map: HashMap<String, String>,
    /// AMT `Practitioner` FHIR id -> Oscar `providerNo`. Consulted only when
    /// live identifier resolution (`resolve_identifier` against
    /// `oscar_provider_system`) returns `None` for a given Practitioner
    /// reference. See TASK_FEATURES_SPEC_PRACTITIONER_PROVIDER_MAP.md.
    #[serde(default)]
    pub practitioner_provider_map: HashMap<String, String>,
    #[serde(default)]
    pub db: WritebackDatabaseConfig,
}

impl Default for WritebackConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            state_dir: default_state_dir(),
            poll_interval_ms: default_poll_interval_ms(),
            page_size: default_page_size(),
            retry_max_attempts: default_retry_max_attempts(),
            retry_base_ms: default_retry_base_ms(),
            dead_letter_path: default_dead_letter_path(),
            sentinel_update_user: default_sentinel_update_user(),
            hapi_resource_types: default_hapi_resource_types(),
            appointment_status_map: HashMap::new(),
            default_consult_provider_no: None,
            consult_service_map: HashMap::new(),
            consult_status_map: HashMap::new(),
            practitioner_provider_map: HashMap::new(),
            db: WritebackDatabaseConfig::default(),
        }
    }
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
    #[serde(default = "default_oscar_provider_system")]
    pub oscar_provider_system: String,
    #[serde(default = "default_oscar_appointment_system")]
    pub oscar_appointment_system: String,
    #[serde(default = "default_oscar_consult_request_system")]
    pub oscar_consult_request_system: String,
    #[serde(default = "default_oscar_care_team_system")]
    pub oscar_care_team_system: String,
    #[serde(default = "default_oscar_note_system")]
    pub oscar_note_system: String,
    #[serde(default = "default_oscar_note_revision_system")]
    pub oscar_note_revision_system: String,
    #[serde(default = "default_oscar_note_document_system")]
    pub oscar_note_document_system: String,
    #[serde(default = "default_msp_service_code_system")]
    pub msp_service_code_system: String,
    #[serde(default = "default_oscar_dxresearch_system")]
    pub oscar_dxresearch_system: String,
    #[serde(default = "default_oscar_consult_response_system")]
    pub oscar_consult_response_system: String,
    #[serde(default = "default_oscar_cpp_condition_system")]
    pub oscar_cpp_condition_system: String,
    #[serde(default = "default_icd9_system")]
    pub icd9_system: String,
    #[serde(default = "default_bc_phn_system")]
    pub bc_phn_system: String,
    #[serde(default = "default_bc_msp_practitioner_system")]
    pub bc_msp_practitioner_system: String,
    pub token_env: Option<String>,
    #[serde(default)]
    pub keycloak: Option<KeycloakConfig>,
}

impl Default for FhirConfig {
    fn default() -> Self {
        Self {
            base_url: default_fhir_base_url(),
            oscar_demographic_system: default_oscar_demographic_system(),
            oscar_provider_system: default_oscar_provider_system(),
            oscar_appointment_system: default_oscar_appointment_system(),
            oscar_consult_request_system: default_oscar_consult_request_system(),
            oscar_care_team_system: default_oscar_care_team_system(),
            oscar_note_system: default_oscar_note_system(),
            oscar_note_revision_system: default_oscar_note_revision_system(),
            oscar_note_document_system: default_oscar_note_document_system(),
            msp_service_code_system: default_msp_service_code_system(),
            oscar_dxresearch_system: default_oscar_dxresearch_system(),
            oscar_consult_response_system: default_oscar_consult_response_system(),
            oscar_cpp_condition_system: default_oscar_cpp_condition_system(),
            icd9_system: default_icd9_system(),
            bc_phn_system: default_bc_phn_system(),
            bc_msp_practitioner_system: default_bc_msp_practitioner_system(),
            token_env: None,
            keycloak: None,
        }
    }
}

fn default_fhir_base_url() -> String {
    "http://localhost:8082/fhir".to_string()
}

fn default_oscar_demographic_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-demographic".to_string()
}

fn default_oscar_provider_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-provider".to_string()
}

fn default_oscar_appointment_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-appointment".to_string()
}

fn default_oscar_consult_request_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-consult-request".to_string()
}

fn default_oscar_care_team_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-care-team".to_string()
}

fn default_oscar_note_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-note".to_string()
}

fn default_oscar_note_revision_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-note-revision".to_string()
}

fn default_oscar_note_document_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-note-document".to_string()
}

fn default_msp_service_code_system() -> String {
    "https://arsmedicatech.com/fhir/sid/bc-msp-service-code".to_string()
}

fn default_oscar_dxresearch_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-dxresearch".to_string()
}

fn default_oscar_consult_response_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-consult-response".to_string()
}

fn default_oscar_cpp_condition_system() -> String {
    "https://arsmedicatech.com/fhir/sid/oscar-cpp-condition".to_string()
}

fn default_icd9_system() -> String {
    "http://hl7.org/fhir/sid/icd-9-cm".to_string()
}

fn default_bc_phn_system() -> String {
    "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-patient-healthcare-id".to_string()
}

fn default_bc_msp_practitioner_system() -> String {
    "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-provider-billing-number".to_string()
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
    /// OAuth2 client-credentials token source. Takes precedence over
    /// `token_env` when both are set.
    #[serde(default)]
    pub oauth: Option<NodeOAuthConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeOAuthConfig {
    pub token_url: String,
    pub client_id: String,
    pub client_secret_env: String,
    #[serde(default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReplicationLink {
    pub name: String,
    pub source: String,
    /// `None` marks an observe-only link: poll `source`'s `_history` and emit
    /// dispatch notifications without replicating to any target. Replication
    /// behaviour is unchanged when a target is present.
    #[serde(default)]
    pub target: Option<String>,
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

const KNOWN_DISPATCH_RESOURCE_TYPES: &[&str] = &["Patient", "Practitioner", "Appointment"];
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
    }

    for consumer in &cfg.consumers {
        if !consumer.enabled {
            continue;
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
/// federate requirements, forbidden resource types, observe-only rules,
/// OAuth credentials). Fatal on error.
pub fn validate_replication(cfg: &ReplicationConfig, dispatch: &DispatchConfig) -> anyhow::Result<()> {
    if !cfg.enabled {
        return Ok(());
    }

    let mut names = std::collections::HashSet::new();
    for node in &cfg.nodes {
        if !names.insert(node.name.clone()) {
            anyhow::bail!("replication node name '{}' is duplicated", node.name);
        }

        if let Some(oauth) = &node.oauth {
            if std::env::var(&oauth.client_secret_env).is_err() {
                anyhow::bail!(
                    "replication node '{}' oauth client_secret_env '{}' is not set in the environment",
                    node.name, oauth.client_secret_env
                );
            }

            if node.token_env.is_some() {
                tracing::warn!(
                    "replication node '{}' has both token_env and oauth; oauth takes precedence",
                    node.name
                );
            }
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
        if let Some(target) = &link.target {
            if !names.contains(target) {
                anyhow::bail!("replication link '{}' references unknown target node '{}'", link.name, target);
            }
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

        if link.target.is_none() {
            if link.resources.is_empty() {
                anyhow::bail!("replication link '{}' is observe-only but has no resources", link.name);
            }

            if !dispatch.enabled {
                anyhow::bail!("replication link '{}' is observe-only but dispatch is not enabled", link.name);
            }

            let has_consumer = dispatch.consumers.iter().any(|c| {
                c.enabled && c.resource_types.iter().any(|rt| link.resources.iter().any(|lr| lr.eq_ignore_ascii_case(rt)))
            });
            if !has_consumer {
                anyhow::bail!(
                    "replication link '{}' is observe-only but no enabled dispatch consumer matches its resources",
                    link.name
                );
            }
        }
    }

    Ok(())
}

/// Validates Oscar-specific config. Fatal on error when `oscar_enabled`.
pub fn validate_oscar(cfg: &Config) -> anyhow::Result<()> {
    if !cfg.oscar_enabled {
        return Ok(());
    }

    let tz = cfg
        .oscar
        .timezone
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("[oscar] timezone is required when oscar_enabled is true"))?;

    if tz.parse::<chrono_tz::Tz>().is_err() {
        anyhow::bail!("[oscar] timezone '{tz}' is not a valid IANA timezone");
    }

    // Status map values must be valid FHIR Appointment.status codes.
    const VALID_STATUS: &[&str] = &[
        "proposed", "pending", "booked", "arrived", "fulfilled", "cancelled", "noshow",
        "entered-in-error", "checked-in", "waitlist",
    ];
    for (code, status) in &cfg.oscar.appointment_status_map {
        if !VALID_STATUS.iter().any(|v| v.eq_ignore_ascii_case(status)) {
            anyhow::bail!(
                "[oscar.appointment_status_map] code '{code}' maps to unknown FHIR status '{status}'"
            );
        }
    }

    Ok(())
}

/// Validates writeback configuration. Fatal on error when `writeback.enabled`.
pub fn validate_writeback(cfg: &Config) -> anyhow::Result<()> {
    if !cfg.writeback.enabled {
        return Ok(());
    }

    let tz = cfg
        .oscar
        .timezone
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("[writeback] requires [oscar].timezone to be set"))?;
    if tz.parse::<chrono_tz::Tz>().is_err() {
        anyhow::bail!("[oscar] timezone '{tz}' is not a valid IANA timezone");
    }

    if cfg.oscar.default_mrp_provider_no.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        anyhow::bail!("[writeback] requires [oscar].default_mrp_provider_no");
    }
    if cfg.oscar.default_appointment_location.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        anyhow::bail!("[writeback] requires [oscar].default_appointment_location");
    }
    if cfg.oscar.default_appointment_type.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        anyhow::bail!("[writeback] requires [oscar].default_appointment_type");
    }

    if cfg.writeback.sentinel_update_user.is_empty() || cfg.writeback.sentinel_update_user.len() > 6
    {
        anyhow::bail!("[writeback].sentinel_update_user must be 1-6 characters");
    }

    if cfg.writeback.db.host.is_empty()
        || cfg.writeback.db.user.is_empty()
        || cfg.writeback.db.password.is_empty()
    {
        anyhow::bail!("[writeback.db] host, user, and password are required");
    }

    const VALID_STATUS: &[&str] = &[
        "proposed", "pending", "booked", "arrived", "fulfilled", "cancelled", "noshow",
        "entered-in-error", "checked-in", "waitlist",
    ];
    for (fhir, oscar) in &cfg.writeback.appointment_status_map {
        if !VALID_STATUS.iter().any(|v| v.eq_ignore_ascii_case(fhir)) {
            anyhow::bail!("[writeback.appointment_status_map] unknown FHIR status '{fhir}'");
        }
        if oscar.is_empty() || oscar.len() > 2 {
            anyhow::bail!(
                "[writeback.appointment_status_map] status '{fhir}' maps to '{oscar}', which must be 1-2 chars"
            );
        }
    }

    const VALID_RESOURCE_TYPES: &[&str] =
        &["Patient", "Appointment", "Encounter", "DocumentReference", "ServiceRequest"];
    for r in &cfg.writeback.hapi_resource_types {
        if !VALID_RESOURCE_TYPES.contains(&r.as_str()) {
            anyhow::bail!(
                "[writeback].hapi_resource_types contains unknown resource type '{r}'"
            );
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

    validate_replication(&config.replication, &config.dispatch)?;
    validate_dispatch(&config.dispatch)?;
    validate_oscar(&config)?;
    validate_writeback(&config)?;

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

    #[test]
    fn validate_oscar_requires_timezone() {
        let cfg = Config {
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
            fhir: FhirConfig::default(),
            sync: SyncConfig::default(),
            replication: ReplicationConfig::default(),
            dispatch: DispatchConfig::default(),
            oscar_enabled: true,
            oscar: OscarConfig::default(),
            debug: None,
            writeback: WritebackConfig::default(),
        };
        let err = validate_oscar(&cfg).unwrap_err().to_string();
        assert!(err.contains("timezone is required"));
    }

    #[test]
    fn validate_oscar_rejects_invalid_timezones() {
        let cfg = Config {
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
            fhir: FhirConfig::default(),
            sync: SyncConfig::default(),
            replication: ReplicationConfig::default(),
            dispatch: DispatchConfig::default(),
            oscar_enabled: true,
            oscar: OscarConfig {
                timezone: Some("Mars/Opportunity".to_string()),
                region: None,
                appointment_status_map: OscarConfig::default().appointment_status_map,
                default_mrp_provider_no: None,
                default_appointment_location: None,
                default_appointment_type: None,
                care_team_enabled: true,
                consult_response_status_map: OscarConfig::default().consult_response_status_map,
                consult_request_status_map: OscarConfig::default().consult_request_status_map,
            },
            debug: None,
            writeback: WritebackConfig::default(),
        };
        let err = validate_oscar(&cfg).unwrap_err().to_string();
        assert!(err.contains("not a valid IANA timezone"));
    }

    #[test]
    fn validate_oscar_allows_valid_timezones() {
        let cfg = Config {
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
            fhir: FhirConfig::default(),
            sync: SyncConfig::default(),
            replication: ReplicationConfig::default(),
            dispatch: DispatchConfig::default(),
            oscar_enabled: true,
            oscar: OscarConfig {
                timezone: Some("America/Vancouver".to_string()),
                region: None,
                appointment_status_map: OscarConfig::default().appointment_status_map,
                default_mrp_provider_no: None,
                default_appointment_location: None,
                default_appointment_type: None,
                care_team_enabled: true,
                consult_response_status_map: OscarConfig::default().consult_response_status_map,
                consult_request_status_map: OscarConfig::default().consult_request_status_map,
            },
            debug: None,
            writeback: WritebackConfig::default(),
        };
        assert!(validate_oscar(&cfg).is_ok());
    }

    #[test]
    fn validate_oscar_skipped_when_disabled() {
        let cfg = Config {
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
            fhir: FhirConfig::default(),
            sync: SyncConfig::default(),
            replication: ReplicationConfig::default(),
            dispatch: DispatchConfig::default(),
            oscar_enabled: false,
            oscar: OscarConfig::default(),
            debug: None,
            writeback: WritebackConfig::default(),
        };
        assert!(validate_oscar(&cfg).is_ok());
    }
}

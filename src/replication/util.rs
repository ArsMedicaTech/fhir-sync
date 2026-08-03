//! Shared replication helpers: checkpoint I/O, retry, fabric source tags,
//! and FHIR JSON utilities. Kept inside `src/replication/` to satisfy the
//! Isolation Rule; a TODO marks the retry helper for unification later.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::warn;

pub const FABRIC_SOURCE_PREFIX: &str = "urn:arsmedicatech:fhir-sync:";

/// A persisted checkpoint for a single replication link.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkCheckpoint {
    /// RFC3339 `lastUpdated` value; used for the next `_since` parameter.
    pub since: String,
    /// Last target-assigned `versionId` seen for each target resource.
    /// Key = `{resourceType}/{target_id}`.
    #[serde(default)]
    pub last_versionids_seen: HashMap<String, String>,
}

/// Returns the fabric `meta.source` tag for writes from `node_name`.
pub fn fabric_tag(node_name: &str) -> String {
    format!("{FABRIC_SOURCE_PREFIX}{node_name}")
}

/// Resolves the bearer token for a node from its `token_env` variable, if set.
pub fn token_for_node(token_env: &Option<String>) -> Option<String> {
    token_env.as_ref().and_then(|key| std::env::var(key).ok())
}

/// Atomically persists a checkpoint to disk.
pub fn save_checkpoint(path: &str, cp: &LinkCheckpoint) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let tmp = format!("{path}.tmp");
    let json = serde_json::to_string(cp).context("serializing checkpoint")?;

    {
        let mut file = std::fs::File::create(&tmp).with_context(|| format!("creating {tmp}"))?;
        file.write_all(json.as_bytes())
            .context("writing checkpoint")?;
        file.sync_all().ok();
    }

    std::fs::rename(&tmp, path).with_context(|| format!("renaming {tmp} -> {path}"))?;
    Ok(())
}

/// Loads a checkpoint if present and valid; otherwise returns a default.
pub fn load_checkpoint(path: &str) -> LinkCheckpoint {
    match std::fs::read_to_string(path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => LinkCheckpoint::default(),
    }
}

/// Record appended to the conflicts file for a link.
#[derive(Debug, Serialize)]
pub struct ConflictRecord<'a> {
    pub timestamp: String,
    pub link: &'a str,
    pub resource_type: &'a str,
    pub source_id: &'a str,
    pub target_id: &'a str,
    pub source_version_id: &'a str,
    pub source_meta: Option<Value>,
    pub target_meta: Option<Value>,
    pub policy: &'a str,
    pub identifiers: Value,
}

/// Appends one conflict record.
pub fn write_conflict(path: &str, record: &ConflictRecord) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("opening conflict file {path}"))?;

    writeln!(file, "{}", serde_json::to_string(record)?)
        .context("writing conflict record")?;
    Ok(())
}

/// Record appended to the dead-letter file for a link.
#[derive(Debug, Serialize)]
pub struct DeadLetterRecord<'a> {
    pub timestamp: String,
    pub link: &'a str,
    pub resource_type: &'a str,
    pub id: &'a str,
    pub reason: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Appends one dead-letter record. Identifiers and reason only by default.
pub fn write_dead_letter(path: &str, record: &DeadLetterRecord) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("opening dead letter file {path}"))?;

    writeln!(file, "{}", serde_json::to_string(record)?)
        .context("writing dead letter record")?;
    Ok(())
}

/// Strips `meta.versionId` and `meta.lastUpdated` from a resource and sets
/// `meta.source` to the fabric tag for `node_name`.
pub fn adjust_meta(resource: &mut Value, node_name: &str) {
    if let Some(meta) = resource.get_mut("meta") {
        if let Some(m) = meta.as_object_mut() {
            m.remove("versionId");
            m.remove("lastUpdated");
            m.insert("source".to_string(), Value::String(fabric_tag(node_name)));
        }
    } else {
        resource.as_object_mut().map(|r| {
            r.insert(
                "meta".to_string(),
                serde_json::json!({ "source": fabric_tag(node_name) }),
            )
        });
    }
}

/// Extracts a numeric version id from an HAPI `ETag` header (`W/"1"` or `1`).
pub fn parse_etag(etag: &str) -> Option<String> {
    let t = etag.trim();
    let t = t.strip_prefix("W/").unwrap_or(t).trim();
    let t = t.trim_matches('"');
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

/// Extracts the resource id from a `Location` header like
/// `Patient/123/_history/2` or `http://.../Patient/123/_history/2`.
pub fn parse_location_id(location: &str) -> Option<String> {
    let parts: Vec<&str> = location.trim_end_matches('/').split('/').collect();
    if parts.len() >= 3 && parts[parts.len() - 2] == "_history" {
        return Some(parts[parts.len() - 3].to_string());
    }
    if parts.len() >= 2 {
        return Some(parts.last().unwrap().to_string());
    }
    None
}

/// Helper to resolve the first identifier matching `system`.
pub fn find_identifier_value(resource: &Value, system: &str) -> Option<String> {
    resource
        .get("identifier")
        .and_then(Value::as_array)
        .and_then(|ids| {
            ids.iter().find_map(|id| {
                if id.get("system").and_then(Value::as_str) == Some(system) {
                    id.get("value").and_then(Value::as_str).map(String::from)
                } else {
                    None
                }
            })
        })
}

/// Retries an async fallible operation with exponential backoff.
/// // TODO: unify with sink retry after Oscar phases land
pub async fn retry_with_backoff<F, Fut>(
    mut op: F,
    max_attempts: u32,
    base_ms: u64,
    label: &str,
) -> anyhow::Result<()>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    let mut last = None;
    for attempt in 0..max_attempts.max(1) {
        match op().await {
            Ok(()) => return Ok(()),
            Err(e) => {
                warn!("replication retry {}/{} for {label}: {e:?}", attempt + 1, max_attempts);
                last = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_ms.saturating_mul(1u64 << attempt.min(10));
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }
    Err(last.unwrap_or_else(|| anyhow::anyhow!("retry exhausted for {label}")))
}

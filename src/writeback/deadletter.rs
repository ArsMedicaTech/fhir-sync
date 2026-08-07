//! Write-side dead-letter sink.
//!
//! Appends one JSON line per failed write-back resource. The file is created
//! if it does not exist; new records are appended, never truncated.

use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Serialize)]
pub struct DeadLetter {
    pub timestamp: String,
    pub id: String,
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

/// Appends `entry` to `path` as a single JSON line.
pub async fn write(path: &str, entry: &DeadLetter) -> Result<()> {
    let line = serde_json::to_string(entry)? + "\n";
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .await
        .with_context(|| format!("opening dead-letter file {path}"))?;
    file.write_all(line.as_bytes())
        .await
        .with_context(|| format!("writing dead-letter file {path}"))?;
    Ok(())
}

//! Per-consumer delivery: HMAC signing, HTTPS POST, retry, and dead-letter.
//!
//! Each enabled consumer runs in its own tokio task with a dedicated channel.
//! Delivery is at-least-once; the consumer must be idempotent on the
//! `idempotency_key` in the notification body.

use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use tokio::sync::mpsc::Receiver;
use tracing::{error, warn};

use crate::config::DispatchConsumer;
use crate::dispatch::DispatchNotification;

/// Delivery failure classification. Mirrors the sink's `SyncFailure` pattern.
#[derive(Debug)]
enum DeliveryError {
    Retryable(anyhow::Error),
    Permanent(anyhow::Error),
}

impl DeliveryError {
    fn is_permanent(&self) -> bool {
        matches!(self, DeliveryError::Permanent(_))
    }
}

impl std::fmt::Display for DeliveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeliveryError::Retryable(e) => write!(f, "retryable: {e}"),
            DeliveryError::Permanent(e) => write!(f, "permanent: {e}"),
        }
    }
}

/// Runs the delivery loop for one consumer.
pub async fn consume(
    consumer: DispatchConsumer,
    mut rx: Receiver<DispatchNotification>,
    dead_letter_dir: String,
    timeout_ms: u64,
    retry_max_attempts: u32,
    retry_base_ms: u64,
) {
    let client = Client::new();
    let secret = std::env::var(&consumer.secret_env).unwrap_or_default();
    let dlq_path = PathBuf::from(&dead_letter_dir).join(format!("{}.jsonl", consumer.name));

    while let Some(notification) = rx.recv().await {
        if !consumer
            .resource_types
            .iter()
            .any(|r| r.eq_ignore_ascii_case(&notification.resource_type))
        {
            continue;
        }
        if !consumer
            .ops
            .iter()
            .any(|o| o.eq_ignore_ascii_case(notification.op.as_str()))
        {
            continue;
        }

        if let Err(e) = deliver_with_retry(
            &client,
            &consumer.url,
            &secret,
            &notification,
            timeout_ms,
            retry_max_attempts,
            retry_base_ms,
        )
        .await
        {
            error!(
                "dispatch: exhausted delivery for consumer '{}' {}: {e}",
                consumer.name, notification.idempotency_key
            );
            if let Err(dlq_err) = write_dlq(&dlq_path, &notification, &e) {
                error!(
                    "dispatch: failed to write DLQ for consumer '{}' {}: {dlq_err:?}",
                    consumer.name, notification.idempotency_key
                );
            }
        }
    }
}

async fn deliver_with_retry(
    client: &Client,
    url: &str,
    secret: &str,
    notification: &DispatchNotification,
    timeout_ms: u64,
    max_attempts: u32,
    base_ms: u64,
) -> Result<(), DeliveryError> {
    let delivery_id = uuid::Uuid::new_v4().to_string();
    let mut last_err = None;

    for attempt in 0..max_attempts.max(1) {
        match deliver_one(client, url, secret, notification, &delivery_id, timeout_ms).await {
            Ok(()) => return Ok(()),
            Err(e) if e.is_permanent() => return Err(e),
            Err(e) => {
                warn!(
                    "dispatch: delivery attempt {}/{} failed for {}: {e}",
                    attempt + 1,
                    max_attempts,
                    notification.idempotency_key
                );
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let backoff = base_ms.saturating_mul(1u64 << attempt.min(10));
                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                }
            }
        }
    }

    Err(last_err.unwrap_or_else(|| {
        DeliveryError::Permanent(anyhow::anyhow!("delivery attempts exhausted"))
    }))
}

async fn deliver_one(
    client: &Client,
    url: &str,
    secret: &str,
    notification: &DispatchNotification,
    delivery_id: &str,
    timeout_ms: u64,
) -> Result<(), DeliveryError> {
    let body = build_body(notification);
    let timestamp = Utc::now().timestamp().to_string();
    let signed_payload = format!("{}.{}", timestamp, body);
    let signature = hmac_signature(secret, &signed_payload)
        .map_err(|e| DeliveryError::Permanent(anyhow::anyhow!("failed to sign payload: {e}")))?;

    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("X-FhirSync-Signature", format!("sha256={signature}"))
        .header("X-FhirSync-Timestamp", &timestamp)
        .header("X-FhirSync-Delivery", delivery_id)
        .body(body)
        .timeout(Duration::from_millis(timeout_ms))
        .send()
        .await;

    match resp {
        Ok(r) => {
            let status = r.status();
            if status.is_success() {
                return Ok(());
            }
            let text = r.text().await.unwrap_or_default();
            if status.is_server_error() || status.as_u16() == 429 {
                Err(DeliveryError::Retryable(anyhow::anyhow!("HTTP {status}: {text}")))
            } else if status.is_client_error() {
                Err(DeliveryError::Permanent(anyhow::anyhow!("HTTP {status}: {text}")))
            } else {
                Err(DeliveryError::Retryable(anyhow::anyhow!("HTTP {status}: {text}")))
            }
        }
        Err(e) => Err(DeliveryError::Retryable(anyhow::anyhow!("request error: {e}"))),
    }
}

fn build_body(notification: &DispatchNotification) -> String {
    #[derive(serde::Serialize)]
    struct Body {
        spec_version: u32,
        resource_type: String,
        fhir_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        fhir_version_id: Option<String>,
        op: String,
        source: String,
        idempotency_key: String,
        occurred_at: String,
        fhir_base_url: String,
    }

    let body = Body {
        spec_version: 1,
        resource_type: notification.resource_type.clone(),
        fhir_id: notification.fhir_id.clone(),
        fhir_version_id: notification.fhir_version_id.clone(),
        op: notification.op.as_str().to_string(),
        source: notification.source.as_str().to_string(),
        idempotency_key: notification.idempotency_key.clone(),
        occurred_at: notification.occurred_at.to_rfc3339(),
        fhir_base_url: notification.fhir_base_url.clone(),
    };

    serde_json::to_string(&body).unwrap_or_default()
}

fn hmac_signature(secret: &str, payload: &str) -> Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| anyhow::anyhow!("invalid HMAC key"))?;
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();
    Ok(hex::encode(bytes))
}

#[derive(serde::Serialize)]
struct DlqRecord {
    idempotency_key: String,
    fhir_id: String,
    error: String,
}

fn write_dlq(
    path: &std::path::Path,
    notification: &DispatchNotification,
    err: &DeliveryError,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("opening dispatch DLQ {path:?}"))?;

    let record = DlqRecord {
        idempotency_key: notification.idempotency_key.clone(),
        fhir_id: notification.fhir_id.clone(),
        error: err.to_string(),
    };

    writeln!(file, "{}", serde_json::to_string(&record)?)
        .with_context(|| format!("writing dispatch DLQ {path:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DispatchConsumer;
    use crate::event::{Op, Source};

    fn sample_notification() -> DispatchNotification {
        DispatchNotification {
            resource_type: "Patient".to_string(),
            fhir_id: "123".to_string(),
            fhir_version_id: Some("4".to_string()),
            op: Op::Upsert,
            source: Source::OscarBinlog,
            idempotency_key: "oscar:demographic:42:0000012345".to_string(),
            occurred_at: Utc::now(),
            fhir_base_url: "https://fhir.example.invalid/fhir".to_string(),
        }
    }

    #[test]
    fn hmac_signature_vector() {
        // Fixed key/timestamp/body -> deterministic hex.
        let key = "test-secret";
        let payload = "1234567890.{\"spec_version\":1}";
        let sig = hmac_signature(key, payload).unwrap();
        assert_eq!(sig.len(), 64); // 256 bits in hex
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn body_is_reference_only() {
        let n = sample_notification();
        let body = build_body(&n);
        assert!(body.contains("\"fhir_id\":\"123\""));
        assert!(body.contains("\"fhir_version_id\":\"4\""));
        assert!(body.contains("\"idempotency_key\":\"oscar:demographic:42:0000012345\""));
        assert!(!body.contains("first_name"));
        assert!(!body.contains("phone"));
    }

    #[test]
    fn dlq_record_never_contains_phi() {
        let n = sample_notification();
        let err = DeliveryError::Permanent(anyhow::anyhow!("test"));
        let dir = std::env::temp_dir().join(format!("dispatch-dlq-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("consumer-a.jsonl");
        write_dlq(&path, &n, &err).unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains("idempotency_key"));
        assert!(contents.contains("fhir_id"));
        assert!(!contents.contains("fhir_version_id")); // PHI-free DLQ
        assert!(!contents.contains("fhir_base_url"));
        std::fs::remove_dir_all(&dir).ok();
    }
}

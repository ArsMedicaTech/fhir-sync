//! HAPI → HAPI write path: idempotent mirror/federate upsert, delete,
//! and dead-letter classification.

use anyhow::{Context, Result};
use reqwest::StatusCode;
use serde_json::Value;
use tracing::{info, warn};

use crate::config::{ReplicationLink, ReplicationMode, ReplicationNode};

use super::provenance;
use super::util::{adjust_meta, fabric_tag, find_identifier_value, parse_etag, parse_location_id, write_dead_letter, DeadLetterRecord};

#[derive(Debug)]
pub enum ReplicateError {
    Retryable(anyhow::Error),
    Permanent(anyhow::Error),
}

impl From<anyhow::Error> for ReplicateError {
    fn from(e: anyhow::Error) -> Self {
        ReplicateError::Retryable(e)
    }
}

pub struct WriteResult {
    pub target_id: String,
    pub target_version: String,
    pub target_meta_source: Option<String>,
}

/// Upsert a resource from `source` to `target`.
/// `Ok(None)` means the entry was handled (e.g. dead-lettered) and the
/// stream can advance.
pub async fn upsert(
    client: &reqwest::Client,
    link: &ReplicationLink,
    source_node: &ReplicationNode,
    target_node: &ReplicationNode,
    dead_letter_path: &str,
    token: Option<&str>,
    source_resource: &Value,
    version_id: &str,
) -> Result<Option<WriteResult>, ReplicateError> {
    let resource_type = source_resource
        .get("resourceType")
        .and_then(Value::as_str)
        .context("history entry missing resourceType")?;
    let source_id = source_resource
        .get("id")
        .and_then(Value::as_str)
        .context("history entry missing id")?;

    let mut body = source_resource.clone();
    adjust_meta(&mut body, &link.source);

    let result = match link.mode {
        ReplicationMode::Mirror => {
            let url = format!("{}/{}/{}", target_node.base_url.trim_end_matches('/'), resource_type, source_id);
            let mut req = client
                .put(&url)
                .header("Content-Type", "application/fhir+json")
                .header("Accept", "application/fhir+json")
                .body(body.to_string());
            if let Some(t) = token {
                req = req.bearer_auth(t);
            }

            let resp = req.send().await.with_context(|| format!("PUT {url}"))?;
            let status = resp.status();
            if status == StatusCode::OK || status == StatusCode::CREATED {
                let target_version = extract_version(&resp);
                let target_id = source_id.to_string();
                info!("replication link {}: upserted {}/{}", link.name, resource_type, target_id);
                WriteResult {
                    target_id: target_id.clone(),
                    target_version,
                    target_meta_source: Some(fabric_tag(&link.source)),
                }
            } else {
                return classify_write_error(link, resource_type, source_id, version_id, resp, dead_letter_path).await;
            }
        }
        ReplicationMode::Federate => {
            let system = link
                .federate_identifier_system
                .as_ref()
                .expect("federate mode validated");
            let value = match find_identifier_value(&body, system) {
                Some(v) => v,
                None => {
                    write_dead_letter(
                        dead_letter_path,
                        &DeadLetterRecord {
                            timestamp: chrono::Utc::now().to_rfc3339(),
                            link: &link.name,
                            resource_type,
                            id: source_id,
                            reason: "no_federation_identifier",
                            version_id: Some(version_id),
                            error: None,
                        },
                    )?;
                    return Ok(None);
                }
            };

            if let Some(obj) = body.as_object_mut() {
                obj.remove("id");
            }

            let base = format!("{}/{}", target_node.base_url.trim_end_matches('/'), resource_type);
            let ident = format!("{}|{}", system, value);
            let mut req = client
                .put(&base)
                .query(&[("identifier", ident.as_str())])
                .header("Content-Type", "application/fhir+json")
                .header("Accept", "application/fhir+json")
                .body(body.to_string());
            if let Some(t) = token {
                req = req.bearer_auth(t);
            }

            let resp = req.send().await.with_context(|| format!("conditional PUT {base}?{ident}"))?;
            let status = resp.status();
            if status == StatusCode::OK || status == StatusCode::CREATED {
                let target_version = extract_version(&resp);
                let target_id = resp
                    .headers()
                    .get("Location")
                    .and_then(|h| h.to_str().ok())
                    .and_then(parse_location_id)
                    .unwrap_or_default();
                info!("replication link {}: federated {}/{} -> {}/{}", link.name, resource_type, source_id, resource_type, target_id);
                WriteResult { target_id, target_version, target_meta_source: Some(fabric_tag(&link.source)) }
            } else {
                return classify_write_error(link, resource_type, source_id, version_id, resp, dead_letter_path).await;
            }
        }
    };

    if link.provenance {
        if let Err(e) = provenance::stamp(
            client,
            &target_node.base_url,
            target_node.token_env.as_deref(),
            resource_type,
            &result.target_id,
            &source_node.base_url,
            resource_type,
            source_id,
            version_id,
            &link.name,
            &source_node.name,
        )
        .await
        {
            warn!("replication link {}: provenance stamp failed for {}/{}: {e:?}", link.name, resource_type, result.target_id);
        }
    }

    Ok(Some(result))
}

fn extract_version(resp: &reqwest::Response) -> String {
    resp
        .headers()
        .get("ETag")
        .and_then(|h| h.to_str().ok())
        .and_then(parse_etag)
        .unwrap_or_default()
}

async fn classify_write_error(
    link: &ReplicationLink,
    resource_type: &str,
    id: &str,
    version_id: &str,
    resp: reqwest::Response,
    dead_letter_path: &str,
) -> Result<Option<WriteResult>, ReplicateError> {
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    let err = anyhow::anyhow!("HAPI write failed ({status}): {text}");

    if status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS || status.as_u16() == 423 {
        Err(ReplicateError::Retryable(err))
    } else if status.is_client_error() {
        write_dead_letter(
            dead_letter_path,
            &DeadLetterRecord {
                timestamp: chrono::Utc::now().to_rfc3339(),
                link: &link.name,
                resource_type,
                id,
                reason: "permanent_write_failure",
                version_id: Some(version_id),
                error: Some(err.to_string()),
            },
        )?;
        Ok(None)
    } else {
        Err(ReplicateError::Retryable(err))
    }
}

/// Delete a resource on the target.
pub async fn delete(
    client: &reqwest::Client,
    link: &ReplicationLink,
    _source_node: &ReplicationNode,
    target_node: &ReplicationNode,
    dead_letter_path: &str,
    token: Option<&str>,
    resource_type: &str,
    id: &str,
    resource_stub: Option<&Value>,
) -> Result<Option<()>, ReplicateError> {
    match link.mode {
        ReplicationMode::Mirror => {
            let url = format!("{}/{}/{}", target_node.base_url.trim_end_matches('/'), resource_type, id);
            let mut req = client.delete(&url);
            if let Some(t) = token {
                req = req.bearer_auth(t);
            }
            let resp = req.send().await.with_context(|| format!("DELETE {url}"))?;
            let status = resp.status();
            if status.is_success() || status.as_u16() == 404 || status.as_u16() == 410 {
                info!("replication link {}: deleted {}/{} ({})", link.name, resource_type, id, status);
                Ok(Some(()))
            } else if status.is_client_error() {
                let text = resp.text().await.unwrap_or_default();
                write_dead_letter(
                    dead_letter_path,
                    &DeadLetterRecord {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        link: &link.name,
                        resource_type,
                        id,
                        reason: "permanent_delete_failure",
                        version_id: None,
                        error: Some(format!("{status}: {text}")),
                    },
                )?;
                Ok(None)
            } else {
                Err(ReplicateError::Retryable(anyhow::anyhow!("DELETE {url} failed ({status})")))
            }
        }
        ReplicationMode::Federate => {
            if resource_type != "Patient" {
                write_dead_letter(
                    dead_letter_path,
                    &DeadLetterRecord {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        link: &link.name,
                        resource_type,
                        id,
                        reason: "unsupported_federate_delete",
                        version_id: None,
                        error: None,
                    },
                )?;
                return Ok(None);
            }
            federate_soft_delete(client, link, target_node, dead_letter_path, token, id, resource_stub).await
        }
    }
}

async fn federate_soft_delete(
    client: &reqwest::Client,
    link: &ReplicationLink,
    target_node: &ReplicationNode,
    dead_letter_path: &str,
    token: Option<&str>,
    source_id: &str,
    resource_stub: Option<&Value>,
) -> Result<Option<()>, ReplicateError> {
    let system = link
        .federate_identifier_system
        .as_ref()
        .expect("federate mode validated");
    let value = match resource_stub.and_then(|r| find_identifier_value(r, system)) {
        Some(v) => v,
        None => {
            write_dead_letter(
                dead_letter_path,
                &DeadLetterRecord {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    link: &link.name,
                    resource_type: "Patient",
                    id: source_id,
                    reason: "no_federation_identifier",
                    version_id: None,
                    error: None,
                },
            )?;
            return Ok(None);
        }
    };

    let base = format!("{}/Patient", target_node.base_url.trim_end_matches('/'));
    let ident = format!("{}|{}", system, value);
    let mut sreq = client
        .get(&base)
        .query(&[("identifier", ident.as_str()), ("_summary", "true"), ("_count", "1")])
        .header("Accept", "application/fhir+json");
    if let Some(t) = token {
        sreq = sreq.bearer_auth(t);
    }
    let sresp = sreq.send().await.with_context(|| "federate delete search")?;
    if !sresp.status().is_success() {
        return Err(ReplicateError::Retryable(anyhow::anyhow!("federate delete search failed ({})", sresp.status())));
    }
    let bundle: Value = sresp.json().await.context("parsing federate delete search bundle")?;
    let target_id = bundle
        .get("entry")
        .and_then(Value::as_array)
        .and_then(|e| e.first())
        .and_then(|e| e.get("resource"))
        .and_then(|r| r.get("id"))
        .and_then(Value::as_str)
        .map(String::from);

    let Some(target_id) = target_id else {
        return Ok(None);
    };

    let get_url = format!("{}/Patient/{}", target_node.base_url.trim_end_matches('/'), target_id);
    let mut greq = client.get(&get_url).header("Accept", "application/fhir+json");
    if let Some(t) = token {
        greq = greq.bearer_auth(t);
    }
    let gresp = greq.send().await.with_context(|| "federate delete GET")?;
    if !gresp.status().is_success() {
        if gresp.status().as_u16() == 404 || gresp.status().as_u16() == 410 {
            return Ok(None);
        }
        return Err(ReplicateError::Retryable(anyhow::anyhow!("federate delete GET failed ({})", gresp.status())));
    }
    let mut patient: Value = gresp.json().await.context("parsing federate delete GET body")?;
    if let Some(obj) = patient.as_object_mut() {
        obj.insert("active".to_string(), Value::Bool(false));
    }
    adjust_meta(&mut patient, &link.source);

    let put_url = format!("{}/Patient/{}", target_node.base_url.trim_end_matches('/'), target_id);
    let mut preq = client
        .put(&put_url)
        .header("Content-Type", "application/fhir+json")
        .body(patient.to_string());
    if let Some(t) = token {
        preq = preq.bearer_auth(t);
    }
    let presp = preq.send().await.with_context(|| "federate delete PUT")?;
    if !presp.status().is_success() {
        return Err(ReplicateError::Retryable(anyhow::anyhow!("federate delete PUT failed ({})", presp.status())));
    }
    info!("replication link {}: federate-soft-deleted Patient/{}", link.name, target_id);
    Ok(Some(()))
}

//! Provenance stamping for successful upsert replications.

use anyhow::Result;
use serde_json::Value;
use tracing::warn;

/// Posts a `Provenance` resource to `target_base` recording the source
/// of a replicated resource. Non-fatal on failure.
pub async fn stamp(
    client: &reqwest::Client,
    target_base: &str,
    token_env: Option<&str>,
    resource_type: &str,
    target_id: &str,
    source_base: &str,
    source_resource_type: &str,
    source_id: &str,
    source_version_id: &str,
    link_name: &str,
    source_node_name: &str,
) -> Result<()> {
    if target_id.is_empty() {
        return Ok(());
    }

    let token = token_env.and_then(|key| std::env::var(key).ok());
    let url = format!("{}/Provenance", target_base.trim_end_matches('/'));
    let body = serde_json::json!({
        "resourceType": "Provenance",
        "target": [{ "reference": format!("{resource_type}/{target_id}") }],
        "recorded": chrono::Utc::now().to_rfc3339(),
        "agent": [{
            "type": {
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/provenance-participant-type",
                    "code": "assembler"
                }]
            },
            "who": { "display": format!("fhir-sync replication ({link_name})") }
        }],
        "entity": [{
            "role": "source",
            "what": {
                "reference": format!("{}/{source_resource_type}/{source_id}/_history/{source_version_id}",
                    source_base.trim_end_matches('/')),
                "display": format!("Replicated from {source_node_name}")
            }
        }]
    });

    let mut req = client
        .post(&url)
        .header("Content-Type", "application/fhir+json")
        .header("Accept", "application/fhir+json")
        .body(body.to_string());
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    match req.send().await {
        Ok(resp) if resp.status().is_success() => Ok(()),
        Ok(resp) => {
            warn!("Provenance POST returned {}", resp.status());
            Ok(())
        }
        Err(e) => {
            warn!("Provenance POST failed: {e:?}");
            Ok(())
        }
    }
}

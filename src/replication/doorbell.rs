//! Optional Subscription "doorbell" accelerator.
//!
//! A minimal axum listener on `replication.doorbell_port` with a single
//! `POST /replication/notify/{link_name}` route. The payload is ignored; the
//! only effect is `poll_now.notify_one()` for that link.

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::post;
use axum::Router;
use serde_json::Value;
use tokio::sync::Notify;
use tracing::{info, warn};

use crate::config::{Config, ReplicationLink};

/// Starts the doorbell server and idempotently creates rest-hook Subscriptions
/// on each source node that has `subscription_doorbell = true`.
pub async fn run(
    client: reqwest::Client,
    cfg: Config,
    notifiers: Arc<HashMap<String, Arc<Notify>>>,
) -> anyhow::Result<()> {
    for link in &cfg.replication.links {
        if !link.subscription_doorbell {
            continue;
        }
        if let Err(e) = ensure_subscription(&client, &cfg, link).await {
            warn!("replication link {}: failed to ensure doorbell Subscription: {e:?}", link.name);
        } else {
            info!("replication link {}: doorbell Subscription ensured", link.name);
        }
    }

    let addr = format!("0.0.0.0:{}", cfg.replication.doorbell_port);
    let app = Router::new()
        .route("/replication/notify/{link_name}", post(notify_handler))
        .with_state(notifiers);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("replication doorbell listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn notify_handler(
    Path(link_name): Path<String>,
    State(notifiers): State<Arc<HashMap<String, Arc<Notify>>>>,
) -> &'static str {
    if let Some(n) = notifiers.get(&link_name) {
        n.notify_one();
        info!("replication doorbell: notified {}", link_name);
    } else {
        warn!("replication doorbell: no link named {}", link_name);
    }
    "ok"
}

const DOORBELL_ID_SYSTEM: &str = "https://arsmedicatech.com/fhir/sid/fhir-sync-doorbell";

async fn ensure_subscription(
    client: &reqwest::Client,
    cfg: &Config,
    link: &ReplicationLink,
) -> anyhow::Result<()> {
    let source = cfg
        .replication
        .nodes
        .iter()
        .find(|n| n.name == link.source)
        .ok_or_else(|| anyhow::anyhow!("source node {} not found", link.source))?;
    // TODO: migrate to TokenProvider once replication runs against an authenticated HAPI
    let token = source
        .token_env
        .as_ref()
        .and_then(|key| std::env::var(key).ok());

    // Idempotent: search by identifier first.
    let search_url = format!("{}/Subscription", source.base_url.trim_end_matches('/'));
    let sub_ident = format!("{}|{}", DOORBELL_ID_SYSTEM, link.name);
    let mut sreq = client
        .get(&search_url)
        .query(&[("identifier", sub_ident.as_str())])
        .header("Accept", "application/fhir+json");
    if let Some(t) = &token {
        sreq = sreq.bearer_auth(t);
    }
    let sresp = sreq.send().await?;
    if sresp.status().is_success() {
        if let Ok(bundle) = sresp.json::<Value>().await {
            if let Some(entries) = bundle.get("entry").and_then(Value::as_array) {
                if !entries.is_empty() {
                    return Ok(());
                }
            }
        }
    }

    let endpoint = format!(
        "http://fhir-sync:{}/replication/notify/{}",
        cfg.replication.doorbell_port, link.name
    );
    // R4 Subscription only supports one resource type per Subscription, so we
    // use Patient? as the demo criteria. For multi-resource, a
    // Subscription per resource type would be needed.
    let subscription = serde_json::json!({
        "resourceType": "Subscription",
        "status": "requested",
        "reason": format!("fhir-sync replication doorbell for {}", link.name),
        "criteria": "Patient?",
        "channel": {
            "type": "rest-hook",
            "endpoint": endpoint,
            "payload": "application/fhir+json",
            "header": ["Accept: application/fhir+json"]
        },
        "identifier": [{"system": DOORBELL_ID_SYSTEM, "value": link.name}]
    });

    let mut preq = client
        .post(&search_url)
        .header("Content-Type", "application/fhir+json")
        .body(subscription.to_string());
    if let Some(t) = &token {
        preq = preq.bearer_auth(t);
    }
    let presp = preq.send().await?;
    if !presp.status().is_success() {
        let status = presp.status();
        let text = presp.text().await.unwrap_or_default();
        anyhow::bail!("Subscription POST failed ({status}): {text}");
    }
    Ok(())
}

//! Multi-FHIR replication fabric: HAPI → HAPI change-feed polling and
//! generic JSON pass-through.

pub mod conflict;
pub mod counters;
pub mod doorbell;
pub mod poller;
pub mod provenance;
pub mod util;
pub mod writer;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::Notify;
use tracing::{error, info, warn};

use crate::config::{Config, ReplicationLink, ReplicationNode};

/// Shared echo-suppression state: key = `{node_name}/{resourceType}/{id}`
/// mapping to the version known to have been written by this fabric.
#[derive(Clone, Debug)]
pub struct SharedState {
    pub echo: Arc<Mutex<HashMap<String, String>>>,
}

/// Entry point for the replication task set. Spawns one poller per link.
pub async fn run(
    cfg: Config,
    dispatch_tx: Option<mpsc::Sender<crate::dispatch::DispatchNotification>>,
) -> anyhow::Result<()> {
    if !cfg.replication.enabled {
        return Ok(());
    }

    info!("replication: starting {} link(s)", cfg.replication.links.len());

    let state = SharedState {
        echo: Arc::new(Mutex::new(HashMap::new())),
    };

    // Warm the echo map from persisted checkpoints.
    for link in &cfg.replication.links {
        let Some(target_name) = link.target.as_deref() else { continue };
        let cp = util::load_checkpoint(&checkpoint_path(&cfg, &link.name));
        let source_node = find_node(&cfg, &link.source)?;
        let target_node = find_node(&cfg, target_name)?;
        let mut guard = state.echo.lock().unwrap();
        for (resource_key, version) in &cp.last_versionids_seen {
            // Best-effort: for mirror the id is the same on both nodes.
            let echo_key = format!("{}/{}", target_node.name, resource_key);
            guard.insert(echo_key, version.clone());
        }
    }

    let mut notifiers: HashMap<String, Arc<Notify>> = HashMap::new();
    let mut handles = Vec::new();
    let mut doorbell_needed = false;

    for link in &cfg.replication.links {
        let source = match find_node(&cfg, &link.source) {
            Ok(n) => n,
            Err(e) => {
                warn!("replication: skipping link {}: {}", link.name, e);
                continue;
            }
        };
        let target = match link.target.as_deref() {
            None => None,
            Some(t) => match find_node(&cfg, t) {
                Ok(n) => Some(n),
                Err(e) => {
                    warn!("replication: skipping link {}: {}", link.name, e);
                    continue;
                }
            },
        };

        doorbell_needed = doorbell_needed || link.subscription_doorbell;
        let notify = Arc::new(Notify::new());
        notifiers.insert(link.name.clone(), notify.clone());

        let client = reqwest::Client::new();
        let state = state.clone();
        let cfg = cfg.clone();
        let link = link.clone();
        let source = source.clone();
        let target = target.clone();
        let dispatch_tx = dispatch_tx.clone();

        handles.push(tokio::spawn(async move {
            poller::run(client, cfg, link, source, target, state, notify, dispatch_tx).await
        }));
    }

    if doorbell_needed {
        let client = reqwest::Client::new();
        let notifiers = Arc::new(notifiers);
        let cfg = cfg.clone();
        handles.push(tokio::spawn(async move {
            doorbell::run(client, cfg, notifiers).await
        }));
    }

    for h in handles {
        match h.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => error!("replication: task exited with error: {e:?}"),
            Err(e) => error!("replication: task join error: {e:?}"),
        }
    }

    Ok(())
}

fn checkpoint_path(cfg: &Config, link_name: &str) -> String {
    format!("{}/replication/{}/checkpoint.json", cfg.replication.state_dir, link_name)
}

fn find_node(cfg: &Config, name: &str) -> anyhow::Result<ReplicationNode> {
    cfg.replication
        .nodes
        .iter()
        .find(|n| n.name == name)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("replication node '{name}' not found"))
}

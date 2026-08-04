//! Post-commit dispatch fan-out.
//!
//! After the FHIR sink successfully commits a resource to HAPI, it emits a
//! `DispatchNotification` into a dedicated mpsc channel. The dispatch task
//! clones each notification to every configured, enabled consumer and delivers
//! it as a signed HTTPS POST.
//!
//! The outbound body is reference-only and contains zero PHI. Consumers must
//! fetch authoritative data from the FHIR store using `fhir_id` and
//! `fhir_version_id`.
//!
//! Design notes:
//! - Ordering is best-effort per-consumer; global ordering is NOT guaranteed.
//! - Consumers must be idempotent on `idempotency_key` and tolerate reordering.
//! - Security is HMAC + FHIR-layer auth, not URL or resource filter secrecy.
//! - Dispatch must never backpressure the sink; a full consumer channel drops
//!   the notification and increments a counter.

use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::config::DispatchConfig;
use crate::event::{Op, Source};
use crate::metrics::SharedMetrics;

pub mod delivery;

/// Reference-only notification emitted after a successful HAPI commit.
#[derive(Debug, Clone)]
pub struct DispatchNotification {
    pub resource_type: String,
    pub fhir_id: String,
    pub fhir_version_id: Option<String>,
    pub op: Op,
    pub source: Source,
    pub idempotency_key: String,
    pub occurred_at: DateTime<Utc>,
    pub fhir_base_url: String,
}

/// Runs the dispatch fan-out loop.
///
/// Receives notifications from the sink, clones them to each enabled consumer's
/// channel, and waits for the consumer tasks to finish when the input closes.
pub async fn run(
    cfg: DispatchConfig,
    mut rx: mpsc::Receiver<DispatchNotification>,
    metrics: SharedMetrics,
) -> anyhow::Result<()> {
    let consumers: Vec<_> = cfg.consumers.into_iter().filter(|c| c.enabled).collect();
    if consumers.is_empty() {
        while rx.recv().await.is_some() {}
        return Ok(());
    }

    let mut handles = Vec::new();
    let mut consumer_txs = Vec::new();
    for consumer in consumers {
        let (tx, crx) = mpsc::channel::<DispatchNotification>(1024);
        consumer_txs.push(tx);
        handles.push(tokio::spawn(delivery::consume(
            consumer,
            crx,
            cfg.dead_letter_dir.clone(),
            cfg.timeout_ms,
            cfg.retry_max_attempts,
            cfg.retry_base_ms,
        )));
    }

    while let Some(notification) = rx.recv().await {
        for tx in &consumer_txs {
            if tx.try_send(notification.clone()).is_err() {
                warn!("dispatch: dropping notification to consumer channel");
                metrics.inc_dispatch_dropped();
            }
        }
    }

    drop(consumer_txs);
    for h in handles {
        if let Err(e) = h.await {
            error!("dispatch: consumer task join error: {e:?}");
        }
    }

    info!("dispatch: run loop ended");
    Ok(())
}

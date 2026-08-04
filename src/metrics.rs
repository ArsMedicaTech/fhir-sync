//! Shared counters for the 60s status log (received / synced /
//! retried / dead-lettered / current position). Deliberately minimal —
//! no metrics backend, just atomics logged on a timer. If a real metrics
//! sink (Prometheus, etc.) is ever needed, this is the seam to replace.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tracing::info;

#[derive(Debug, Default)]
pub struct Metrics {
    received: AtomicU64,
    synced: AtomicU64,
    retried: AtomicU64,
    dead_lettered: AtomicU64,
    dispatch_dropped: AtomicU64,
    position: Mutex<String>,
}

pub type SharedMetrics = Arc<Metrics>;

impl Metrics {
    pub fn new() -> SharedMetrics {
        Arc::new(Metrics::default())
    }

    pub fn inc_received(&self) {
        self.received.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_synced(&self) {
        self.synced.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_retried(&self) {
        self.retried.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_dead_lettered(&self) {
        self.dead_lettered.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_dispatch_dropped(&self) {
        self.dispatch_dropped.fetch_add(1, Ordering::Relaxed);
    }

    /// Records the current source position, e.g. `"mysql-bin.000002:4"`.
    pub fn set_position(&self, position: impl Into<String>) {
        if let Ok(mut p) = self.position.lock() {
            *p = position.into();
        }
    }

    fn snapshot(&self) -> (u64, u64, u64, u64, u64, String) {
        (
            self.received.load(Ordering::Relaxed),
            self.synced.load(Ordering::Relaxed),
            self.retried.load(Ordering::Relaxed),
            self.dead_lettered.load(Ordering::Relaxed),
            self.dispatch_dropped.load(Ordering::Relaxed),
            self.position.lock().map(|p| p.clone()).unwrap_or_default(),
        )
    }
}

/// Spawns a task that logs a counters snapshot every 60s (Phase 3 item 4).
pub fn spawn_reporter(metrics: SharedMetrics) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let (received, synced, retried, dead_lettered, dispatch_dropped, position) = metrics.snapshot();
            info!(
                "metrics: received={received} synced={synced} retried={retried} \
                 dead_lettered={dead_lettered} dispatch_dropped={dispatch_dropped} position={position}"
            );
        }
    })
}

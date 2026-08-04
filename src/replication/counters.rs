//! Per-link replication counters, logged every 60 s for observability.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tracing::info;

#[derive(Debug, Default)]
pub struct Counters {
    seen: AtomicU64,
    replicated: AtomicU64,
    skipped_by_filter: AtomicU64,
    suppressed: AtomicU64,
    retried: AtomicU64,
    dead_lettered: AtomicU64,
    dispatch_dropped: AtomicU64,
}

impl Counters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_seen(&self) {
        self.seen.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_replicated(&self) {
        self.replicated.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_skipped_by_filter(&self) {
        self.skipped_by_filter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_suppressed(&self) {
        self.suppressed.fetch_add(1, Ordering::Relaxed);
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

    pub fn snapshot(&self) -> (u64, u64, u64, u64, u64, u64) {
        (
            self.seen.load(Ordering::Relaxed),
            self.replicated.load(Ordering::Relaxed),
            self.skipped_by_filter.load(Ordering::Relaxed),
            self.suppressed.load(Ordering::Relaxed),
            self.retried.load(Ordering::Relaxed),
            self.dead_lettered.load(Ordering::Relaxed),
        )
    }
}

/// Spawns a background task that logs the counters every 60 s.
pub fn spawn_reporter(link: String, counters: Arc<Counters>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let (seen, replicated, skipped, suppressed, retried, dl) = counters.snapshot();
            info!(
                "replication link {}: seen={} replicated={} skipped_by_filter={} suppressed={} retried={} dead_lettered={}",
                link, seen, replicated, skipped, suppressed, retried, dl
            );
        }
    })
}

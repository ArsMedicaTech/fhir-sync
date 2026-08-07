//! Oscar write-back (AMT → Oscar).
//!
//! See `TASK_FEATURES_SPEC_OSCAR_WRITEBACK.md` for scope and safety rules.

pub mod authorship;
pub mod deadletter;
pub mod mappers;
pub mod oscar_sink;
pub mod poller;

pub async fn run(cfg: crate::config::Config) -> anyhow::Result<()> {
    if !cfg.writeback.enabled {
        return Ok(());
    }
    poller::run(cfg).await
}

use tokio::{select, signal, sync::mpsc};
use tracing::{info, error};

mod webhook;
mod api;

pub mod config;
pub mod domain;
pub mod adapters;
pub mod service;
pub mod event;
pub mod sources;
pub mod mapping;
pub mod sink;
pub mod checkpoint;
pub mod backfill;
pub mod metrics;

pub mod proto;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = config::load_config()?;
    let metrics = metrics::Metrics::new();
    metrics::spawn_reporter(metrics.clone());

    // Shared channel: producers (source, webhook) push events; the fhir
    // sink is the single consumer (D4). No tokio::broadcast in this phase.
    let (tx, rx) = mpsc::channel::<event::SyncEvent>(1024);

    // Sink must be draining before backfill sends anything — otherwise a
    // backfill larger than the channel capacity would block forever with
    // no consumer yet running.
    let sink_task = tokio::spawn(sink::fhir::run(cfg.clone(), rx, metrics.clone()));

    // `--backfill` snapshot mode. Captures the pre-scan
    // binlog position, persists it as the checkpoint, then batch-sends the
    // whole `demographic` table through the sink. Runs to completion
    // *before* the source task starts, so the source's `resolve_start_position`
    // reads this exact pre-snapshot checkpoint instead of racing its own
    // cold-start `SHOW MASTER STATUS` call.
    if std::env::args().any(|a| a == "--backfill") {
        let columns = sources::mariadb_binlog::resolve_column_map(&cfg.database).await?;
        let total = backfill::run(&cfg, &columns, &tx, &metrics).await?;
        info!("backfill: sent {total} patients");
    }

    let source_task = tokio::spawn(sources::mariadb_binlog::run(cfg.clone(), tx.clone(), metrics.clone()));
    let webhook_task = tokio::spawn(webhook::run_webhook_server(tx.clone(), cfg.server.webhook_port));
    let api_task      = tokio::spawn(api::run_grpc_server(cfg.server.health_port, cfg.server.grpc_port));

    // graceful shutdown on Ctrl-C
    select! {
        res = source_task   => handle_exit("source",   res),
        res = sink_task     => handle_exit("sink",     res),
        res = webhook_task  => handle_exit("webhook",  res),
        res = api_task      => handle_exit("api",      res),
        _  = signal::ctrl_c() => info!("Ctrl-C received, shutting down"),
    };

    Ok(())
}

fn handle_exit(name: &str, res: Result<anyhow::Result<()>, tokio::task::JoinError>) {
    match res {
        Ok(Ok(())) => info!("{name} task exited cleanly"),
        Ok(Err(e)) => error!("{name} task exited with error: {e:?}"),
        Err(e)     => error!("{name} task exited with join error: {e:?}"),
    }
}

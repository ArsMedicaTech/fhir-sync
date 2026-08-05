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
pub mod replication;
pub mod dispatch;

mod auth;

pub mod proto;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = config::load_config()?;

    if cfg.oscar_enabled && cfg.database.host.is_empty() {
        anyhow::bail!("oscar_enabled = true but [database] is missing or has no host");
    }

    if cfg.oscar_enabled {
        mapping::dxresearch::load_diagnostic_codes(&cfg.database, &cfg.oscar).await?;
    }

    let metrics = metrics::Metrics::new();
    metrics::spawn_reporter(metrics.clone());

    // Shared channel: producers (source, webhook) push events; the fhir
    // sink is the single consumer (D4). No tokio::broadcast in this phase.
    let (tx, rx) = mpsc::channel::<event::SyncEvent>(1024);

    let never = || tokio::spawn(std::future::pending::<anyhow::Result<()>>());

    // Dispatch fan-out channel. Created only when dispatch is enabled so the
    // sink's optional sender is never attached to a dangling receiver.
    let (dispatch_tx, dispatch_task) = if cfg.dispatch.enabled {
        let (dtx, drx) = mpsc::channel::<dispatch::DispatchNotification>(1024);
        let task = tokio::spawn(dispatch::run(cfg.dispatch.clone(), drx, metrics.clone()));
        (Some(dtx), task)
    } else {
        (None, never())
    };

    // Sink must be draining before backfill sends anything — otherwise a
    // backfill larger than the channel capacity would block forever with
    // no consumer yet running.

    let sink_task = if cfg.oscar_enabled {
        tokio::spawn(sink::fhir::run(cfg.clone(), rx, metrics.clone(), dispatch_tx.clone()))
    } else { never() };

    // `--backfill` snapshot mode. Captures the pre-scan
    // binlog position, persists it as the checkpoint, then batch-sends the
    // `provider`, `demographic`, and `appointment` tables through the sink. Runs to completion
    // *before* the source task starts, so the source's `resolve_start_position`
    // reads this exact pre-snapshot checkpoint instead of racing its own
    // cold-start `SHOW MASTER STATUS` call.
    if cfg.oscar_enabled && std::env::args().any(|a| a == "--backfill") {
        let total = backfill::run(&cfg, &tx, &metrics).await?;
        info!("backfill: sent {total} resources");
        drop(tx);
        let _ = sink_task.await;
        return Ok(());
    }

    let source_task = if cfg.oscar_enabled {
        tokio::spawn(sources::mariadb_binlog::run(cfg.clone(), tx.clone(), metrics.clone()))
    } else { never() };
    
    let webhook_task = tokio::spawn(webhook::run_webhook_server(tx.clone(), cfg.server.webhook_port));
    
    let api_task      = tokio::spawn(api::run_grpc_server(cfg.server.health_port, cfg.server.grpc_port));

    let replication_task = if cfg.replication.enabled {
        tokio::spawn(replication::run(cfg.clone(), dispatch_tx.clone()))
    } else { never() };

    // graceful shutdown on Ctrl-C
    select! {
        res = source_task   => handle_exit("source",   res),
        res = sink_task     => handle_exit("sink",     res),
        res = webhook_task  => handle_exit("webhook",  res),
        res = api_task      => handle_exit("api",      res),
        res = replication_task => handle_exit("replication", res),
        res = dispatch_task => handle_exit("dispatch", res),
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

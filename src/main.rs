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

pub mod proto;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = config::load_config()?;

    // Shared channel: producers (source, webhook) push events; the fhir
    // sink is the single consumer (D4). No tokio::broadcast in this phase.
    let (tx, rx) = mpsc::channel::<event::SyncEvent>(1024);

    let source_task = tokio::spawn(sources::mariadb_binlog::run(cfg.clone(), tx.clone()));
    let sink_task   = tokio::spawn(sink::fhir::run(cfg.clone(), rx));
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

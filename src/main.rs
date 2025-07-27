use tokio::{select, signal, sync::mpsc};
use tracing::{info, error};

use crate::domain::patient::DomainPatient;

mod binlog;
mod webhook;
mod api;

pub mod config;
pub mod domain;
pub mod adapters;
pub mod ext;

pub mod proto;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Shared channel: listeners push events; API consumes & forwards
    let (tx, rx) = mpsc::channel::<Event>(1024);

    let binlog_task   = tokio::spawn(binlog::run_binlog_listener(tx.clone()));
    let webhook_task  = tokio::spawn(webhook::run_webhook_server(tx.clone()));
    let api_task      = tokio::spawn(api::run_grpc_server(rx));

    // graceful shutdown on Ctrl-C
    select! {
        res = binlog_task   => handle_exit("binlog",   res),
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

/// Central event enum; extend ad-hoc
#[derive(Debug)]
pub enum Event {
    PatientUpsert(DomainPatient),
    //EncounterUpdate(Encounter),
    // ...
}

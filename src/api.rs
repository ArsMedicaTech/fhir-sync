use axum::routing::get;
use axum::Router;
use tonic::{transport::Server as TonicServer};
use crate::Event;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use tracing_subscriber::{fmt, EnvFilter};

use crate::service::fhir_sync::MyFhirSyncService as ServiceImpl;

use crate::proto::fhir_sync::fhir_sync_server::FhirSyncServer;


pub async fn run_grpc_server(
    mut rx: tokio::sync::mpsc::Receiver<Event>,
) -> anyhow::Result<()> {
    // -------- Setup (no changes here) -----------------
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<FhirSyncServer<ServiceImpl>>()
        .await;

    let fhir_service = FhirSyncServer::new(ServiceImpl::default());

    tokio::spawn(async move {
        while let Some(_ev) = rx.recv().await {
            // convert DTO → google.fhir.r5.core.Patient → send to peer
        }
    });

    async fn health_check() -> &'static str {
        "ok"
    }

    let http_router = Router::new()
        .route("/grpc_health_probe", get(health_check));

    // -------- Server Startup (Updated Logic) -----------------

    // 1. Create the HTTP server task using axum::serve
    let http_addr = "0.0.0.0:8080".parse::<SocketAddr>()?;
    let http_server = tokio::spawn(async move {
        println!("HTTP health check server listening on {}", http_addr);
        let listener = TcpListener::bind(http_addr).await.unwrap();
        axum::serve(listener, http_router.into_make_service()).await.unwrap();
    });

    // 2. Create the gRPC server task using tonic's standard .serve()
    let grpc_addr = "0.0.0.0:50051".parse::<SocketAddr>()?;
    let grpc_server = tokio::spawn(async move {
        println!("gRPC server listening on {}", grpc_addr);
        TonicServer::builder()
            .add_service(health_service)
            .add_service(fhir_service)
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // 3. Wait for either server to exit
    tokio::select! {
        _ = http_server => eprintln!("HTTP server exited."),
        _ = grpc_server => eprintln!("gRPC server exited."),
    };

    Ok(())
}

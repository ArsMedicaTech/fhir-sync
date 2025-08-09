use axum::routing::get;
use axum::Router;
use tonic::{transport::Server as TonicServer};
use crate::Event;

use tracing_subscriber::{fmt, EnvFilter};

use crate::service::fhir_sync::MyFhirSyncService as ServiceImpl;

use crate::proto::fhir_sync::fhir_sync_server::FhirSyncServer;

pub async fn run_grpc_server(
    mut rx: tokio::sync::mpsc::Receiver<Event>,
) -> anyhow::Result<()> {
    // -------- gRPC health service -----------------
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<FhirSyncServer<ServiceImpl>>()
        .await;

    // start a task that forwards rx → FHIR sync pipeline
    tokio::spawn(async move {
        while let Some(_ev) = rx.recv().await {
            // convert DTO → google.fhir.r5.core.Patient → send to peer
        }
    });

    // Simple health check handler for HTTP health probe
    async fn health_check() -> &'static str {
        "ok"
    }

    // Start separate HTTP server for health checks
    let http_router: Router = Router::new()
        .route("/grpc_health_probe", get(health_check));

    // 1. Start the gRPC server on its own port.
    let grpc_addr = "[::]:50051".parse()?;
    let http_addr = "[::]:8080".parse()?;
    let grpc_server = tokio::spawn(async move {
        println!("gRPC server listening on {}", grpc_addr);
        TonicServer::builder()
            .add_service(health_service)
            .add_service(FhirSyncServer::new(ServiceImpl::default()))
            .serve(grpc_addr)
            .await
    });

    // 2. Start the HTTP health check server on a different port.
    let http_server = tokio::spawn(async move {
        println!("HTTP health check server listening on {}", http_addr);
        axum_server::bind(http_addr)
            .serve(http_router.into_make_service())
            .await
    });

    // 3. Wait for both servers to complete (or error).
    let (grpc_res, http_res) = tokio::join!(grpc_server, http_server);
    grpc_res??;
    http_res??;

    Ok(())
}

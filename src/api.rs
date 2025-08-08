use tonic::{transport::Server};
use crate::Event;

use tracing_subscriber::{fmt, EnvFilter};

use crate::service::fhir_sync::MyFhirSyncService as ServiceImpl;

use crate::proto::fhir_sync::fhir_sync_server::FhirSyncServer;

pub async fn run_grpc_server(
    mut rx: tokio::sync::mpsc::Receiver<Event>,
) -> anyhow::Result<()> {
    // -------- logging -----------------------------
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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

    // Start separate HTTP server for health checks
    let health_app = Router::new()
        .route("/grpc_health_probe", get(health_check));

    // expose external API (optional)
    Server::builder()
        //.add_service(MyApiServer::new(MySvc))
        .add_service(health_service)
        .add_service(FhirSyncServer::new(
            ServiceImpl::default(),
        ))
        .serve("[::]:50051".parse()?)
        .await?;
    Ok(())
}

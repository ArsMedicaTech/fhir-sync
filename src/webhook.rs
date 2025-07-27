use axum::{Json, Router, routing::post};
use crate::{Event};
use crate::domain::patient::DomainPatient;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Extension;
use std::net::SocketAddr;

#[axum::debug_handler]
pub async fn handle_upsert(
    Extension(_tx): Extension<tokio::sync::mpsc::Sender<Event>>,
    Json(dto): Json<DomainPatient>,
) -> impl IntoResponse + Send {
    println!("Got patient: {:?}", dto);
    // You can use tx.send(...) here if needed
    StatusCode::OK
}

pub async fn run_webhook_server(tx: tokio::sync::mpsc::Sender<Event>) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/patient", post(handle_upsert))
        .layer(Extension(tx));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080".parse::<SocketAddr>()?)
        .await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

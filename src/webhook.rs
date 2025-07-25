use axum::{Json, Router, routing::post};
use crate::{Event};
use crate::domain::patient::DomainPatient;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Extension;

#[axum::debug_handler]
pub async fn handle_upsert(
    Json(dto): Json<DomainPatient>,
    Extension(_tx): Extension<tokio::sync::mpsc::Sender<Event>>,
) -> impl IntoResponse + Send {
    println!("Got patient: {:?}", dto);
    // You can use tx.send(...) here if needed
    StatusCode::OK
}

pub async fn run_webhook_server(tx: tokio::sync::mpsc::Sender<Event>) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/patient", post(handle_upsert))
        .layer(Extension(tx));
    
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080".parse()?)
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

use axum::{Json, Router, routing::post};
use crate::{Event};
use crate::domain::patient::DomainPatient;
use axum::http::StatusCode;

pub async fn handle_upsert(Json(dto): Json<DomainPatient>) -> StatusCode {
    println!("Got patient: {:?}", dto);
    StatusCode::OK
}

pub async fn run_webhook_server(tx: tokio::sync::mpsc::Sender<Event>)
        -> anyhow::Result<()> {
        let app = Router::new().route("/patient", post(handle_upsert));

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

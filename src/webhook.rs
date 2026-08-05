use axum::{Json, Router, routing::post};
use crate::domain::patient::{AddressKind, AddressUse, DomainAddress, DomainPatient};
use crate::domain::resource::DomainResource;
use crate::event::{Op, Source, SyncEvent};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Extension;
use std::net::SocketAddr;

#[axum::debug_handler]
pub async fn handle_upsert(
    Extension(tx): Extension<tokio::sync::mpsc::Sender<SyncEvent>>,
    Json(dto): Json<DomainPatient>,
) -> impl IntoResponse + Send {
    handle_upsert_internal(tx, dto).await
}

async fn handle_upsert_internal(
    tx: tokio::sync::mpsc::Sender<SyncEvent>,
    patient: DomainPatient,
) -> StatusCode {
    let event = SyncEvent::new(
        Source::Webhook,
        Op::Upsert,
        DomainResource::Patient(patient),
        chrono::Utc::now(),
    );

    match tx.send(event).await {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

pub async fn run_webhook_server(tx: tokio::sync::mpsc::Sender<SyncEvent>, port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/patient", post(handle_upsert))
        .layer(Extension(tx));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind webhook server");

    axum::serve(listener, app).await.expect("run webhook server");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test patient
    fn create_test_patient() -> DomainPatient {
        DomainPatient {
            demographic_no: "12345".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            date_of_birth: Some("1990-01-01".to_string()),
            addresses: vec![DomainAddress {
                line: Some("123 Main St".to_string()),
                city: Some("Toronto".to_string()),
                province: Some("ON".to_string()),
                postal: Some("M5V1A1".to_string()),
                use_: AddressUse::Home,
                kind: AddressKind::Postal,
            }],
            patient_status: None,
            merged_to: None,
            sex: Some("male".to_string()),
            phone: Some("+1-555-123-4567".to_string()),
            email: Some("john.doe@example.com".to_string()),
            hin: Some("1234567890".to_string()),
        }
    }

    #[tokio::test]
    async fn test_handle_upsert_function_logic() {
        // Test the actual function logic, not the HTTP layer
        let (tx, mut rx) = tokio::sync::mpsc::channel::<SyncEvent>(1);
        let test_patient = create_test_patient();
        
        // Call the function directly (simulating what the HTTP handler does)
        let result = handle_upsert_internal(tx, test_patient).await;
        
        // Verify the function returns success
        assert_eq!(result, StatusCode::OK);
        
        // Verify a SyncEvent was actually sent
        let event = rx.try_recv().unwrap();
        assert_eq!(event.source, Source::Webhook);
        assert_eq!(event.op, Op::Upsert);
    }

    #[tokio::test]
    async fn test_handle_upsert_with_minimal_patient() {
        let (tx, _rx) = tokio::sync::mpsc::channel::<SyncEvent>(1);
        let minimal_patient = DomainPatient {
            demographic_no: "67890".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
            addresses: Vec::new(),
            patient_status: None,
            merged_to: None,
            sex: None,
            phone: None,
            email: None,
            hin: None,
        };
        
        let result = handle_upsert_internal(tx, minimal_patient).await;
        assert_eq!(result, StatusCode::OK);
    }

    #[test]
    fn test_domain_patient_serialization() {
        let patient = create_test_patient();
        
        // Test serialization
        let json = serde_json::to_string(&patient).unwrap();
        assert!(json.contains("12345"));
        assert!(json.contains("John"));
        assert!(json.contains("Doe"));
        
        // Test deserialization
        let deserialized: DomainPatient = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.demographic_no, patient.demographic_no);
        assert_eq!(deserialized.first_name, patient.first_name);
        assert_eq!(deserialized.last_name, patient.last_name);
    }

    #[test]
    fn test_domain_patient_with_optional_fields() {
        let patient = DomainPatient {
            demographic_no: "67890".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
            addresses: Vec::new(),
            patient_status: None,
            merged_to: None,
            sex: None,
            phone: None,
            email: None,
            hin: None,
        };
        
        let json = serde_json::to_string(&patient).unwrap();
        let deserialized: DomainPatient = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.demographic_no, "67890");
        assert_eq!(deserialized.first_name, None);
        assert_eq!(deserialized.last_name, None);
    }

}

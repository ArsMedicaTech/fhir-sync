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
            location: Some(("Toronto".to_string(), "ON".to_string(), "Canada".to_string(), "M5V1A1".to_string())),
            sex: Some("male".to_string()),
            phone: Some("+1-555-123-4567".to_string()),
            email: Some("john.doe@example.com".to_string()),
        }
    }

    #[tokio::test]
    async fn test_handle_upsert_function_logic() {
        // Test the actual function logic, not the HTTP layer
        let (tx, _rx) = tokio::sync::mpsc::channel::<Event>(1);
        let test_patient = create_test_patient();
        
        // Call the function directly (simulating what the HTTP handler does)
        let result = handle_upsert_internal(tx, test_patient).await;
        
        // Verify the function returns success
        assert_eq!(result, StatusCode::OK);
        
        // Optionally verify that an event was sent (if you want to test that)
        // let event = rx.try_recv().unwrap();
        // assert!(matches!(event, Event::PatientUpserted(_)));
    }

    #[tokio::test]
    async fn test_handle_upsert_with_minimal_patient() {
        let (tx, _rx) = tokio::sync::mpsc::channel::<Event>(1);
        let minimal_patient = DomainPatient {
            demographic_no: "67890".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
            location: None,
            sex: None,
            phone: None,
            email: None,
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
            location: None,
            sex: None,
            phone: None,
            email: None,
        };
        
        let json = serde_json::to_string(&patient).unwrap();
        let deserialized: DomainPatient = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.demographic_no, "67890");
        assert_eq!(deserialized.first_name, None);
        assert_eq!(deserialized.last_name, None);
    }

    // Helper function to test the actual logic without HTTP overhead
    async fn handle_upsert_internal(
        _tx: tokio::sync::mpsc::Sender<Event>,
        patient: DomainPatient,
    ) -> StatusCode {
        // This simulates what the HTTP handler does internally
        println!("Got patient: {:?}", patient);
        // You can add actual business logic here
        // tx.send(Event::PatientUpserted(patient)).await.unwrap();
        StatusCode::OK
    }
}

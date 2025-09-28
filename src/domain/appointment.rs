use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainAppointment {
    pub appointment_id: String,
    pub patient_demographic_no: String,
    pub practitioner_id: Option<String>,
    pub location_id: Option<String>,
    pub status: Option<String>, // "proposed" | "pending" | "booked" | "arrived" | "fulfilled" | "cancelled" | "noshow" | "entered-in-error" | "checked-in" | "waitlist"
    pub service_type: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>, // ISO datetime string
    pub end_time: Option<String>, // ISO datetime string
    pub duration_minutes: Option<u32>,
    pub reason: Option<String>,
    pub priority: Option<String>,
    pub comments: Option<String>,
    pub created_date: Option<String>, // ISO datetime string
    pub cancellation_reason: Option<String>,
    pub cancellation_date: Option<String>, // ISO datetime string
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_appointment_deserialization() {
        let json = r#"{
            "appointment_id": "apt_12345",
            "patient_demographic_no": "12345",
            "practitioner_id": "prac_001",
            "location_id": "loc_001",
            "status": "booked",
            "service_type": "consultation",
            "description": "Follow-up appointment",
            "start_time": "2024-01-15T10:00:00Z",
            "end_time": "2024-01-15T10:30:00Z",
            "duration_minutes": 30,
            "reason": "Follow-up care",
            "priority": "routine",
            "comments": "Patient prefers morning appointments",
            "created_date": "2024-01-01T09:00:00Z",
            "cancellation_reason": null,
            "cancellation_date": null
        }"#;

        let appointment: DomainAppointment = serde_json::from_str(json).unwrap();
        
        assert_eq!(appointment.appointment_id, "apt_12345");
        assert_eq!(appointment.patient_demographic_no, "12345");
        assert_eq!(appointment.practitioner_id, Some("prac_001".to_string()));
        assert_eq!(appointment.location_id, Some("loc_001".to_string()));
        assert_eq!(appointment.status, Some("booked".to_string()));
        assert_eq!(appointment.service_type, Some("consultation".to_string()));
        assert_eq!(appointment.description, Some("Follow-up appointment".to_string()));
        assert_eq!(appointment.start_time, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(appointment.end_time, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(appointment.duration_minutes, Some(30));
        assert_eq!(appointment.reason, Some("Follow-up care".to_string()));
        assert_eq!(appointment.priority, Some("routine".to_string()));
        assert_eq!(appointment.comments, Some("Patient prefers morning appointments".to_string()));
        assert_eq!(appointment.created_date, Some("2024-01-01T09:00:00Z".to_string()));
        assert_eq!(appointment.cancellation_reason, None);
        assert_eq!(appointment.cancellation_date, None);
    }

    #[test]
    fn test_domain_appointment_minimal_deserialization() {
        let json = r#"{
            "appointment_id": "apt_67890",
            "patient_demographic_no": "67890"
        }"#;

        let appointment: DomainAppointment = serde_json::from_str(json).unwrap();
        
        assert_eq!(appointment.appointment_id, "apt_67890");
        assert_eq!(appointment.patient_demographic_no, "67890");
        assert_eq!(appointment.practitioner_id, None);
        assert_eq!(appointment.location_id, None);
        assert_eq!(appointment.status, None);
        assert_eq!(appointment.service_type, None);
        assert_eq!(appointment.description, None);
        assert_eq!(appointment.start_time, None);
        assert_eq!(appointment.end_time, None);
        assert_eq!(appointment.duration_minutes, None);
        assert_eq!(appointment.reason, None);
        assert_eq!(appointment.priority, None);
        assert_eq!(appointment.comments, None);
        assert_eq!(appointment.created_date, None);
        assert_eq!(appointment.cancellation_reason, None);
        assert_eq!(appointment.cancellation_date, None);
    }

    #[test]
    fn test_domain_appointment_cancelled_appointment() {
        let json = r#"{
            "appointment_id": "apt_99999",
            "patient_demographic_no": "99999",
            "status": "cancelled",
            "start_time": "2024-01-20T14:00:00Z",
            "end_time": "2024-01-20T14:30:00Z",
            "cancellation_reason": "Patient requested",
            "cancellation_date": "2024-01-19T16:30:00Z"
        }"#;

        let appointment: DomainAppointment = serde_json::from_str(json).unwrap();
        
        assert_eq!(appointment.appointment_id, "apt_99999");
        assert_eq!(appointment.patient_demographic_no, "99999");
        assert_eq!(appointment.status, Some("cancelled".to_string()));
        assert_eq!(appointment.start_time, Some("2024-01-20T14:00:00Z".to_string()));
        assert_eq!(appointment.end_time, Some("2024-01-20T14:30:00Z".to_string()));
        assert_eq!(appointment.cancellation_reason, Some("Patient requested".to_string()));
        assert_eq!(appointment.cancellation_date, Some("2024-01-19T16:30:00Z".to_string()));
    }

    #[test]
    fn test_domain_appointment_missing_required_field() {
        let json = r#"{
            "practitioner_id": "prac_001"
        }"#;

        // This should fail because appointment_id and patient_demographic_no are required
        let result: Result<DomainAppointment, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}

use chrono::{DateTime, Utc};
use crate::domain::appointment::DomainAppointment;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    Appointment, // proto message we're producing
    Id,
    Identifier,
    Uri,
    String,
    CodeableConcept,
    Coding,
    Code,
    Reference,
    Instant,
    DateTime as FhirDateTime,
    PositiveInt,
    Annotation,
    CodeableReference,
};

use crate::proto::google::fhir::proto::r5::core::appointment_status_code::Value as AppointmentStatusCode;
use crate::proto::google::fhir::proto::r5::core::appointment;

// Shorthand for nested message that lives *inside* Appointment.
type StatusCode = appointment::StatusCode;
type Participant = appointment::Participant;

impl From<DomainAppointment> for Appointment {
    fn from(src: DomainAppointment) -> Self {
        // Start with a completely empty message
        let mut dest = Appointment::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.appointment_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:appointment_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.appointment_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status_raw) = src.status {
            let code_val = match status_raw.to_lowercase().as_str() {
                "proposed"        => AppointmentStatusCode::Proposed,
                "pending"         => AppointmentStatusCode::Pending,
                "booked"          => AppointmentStatusCode::Booked,
                "arrived"         => AppointmentStatusCode::Arrived,
                "fulfilled"       => AppointmentStatusCode::Fulfilled,
                "cancelled"       => AppointmentStatusCode::Cancelled,
                "noshow"          => AppointmentStatusCode::Noshow,
                "entered-in-error" => AppointmentStatusCode::EnteredInError,
                "checked-in"      => AppointmentStatusCode::CheckedIn,
                "waitlist"        => AppointmentStatusCode::Waitlist,
                _                 => AppointmentStatusCode::InvalidUninitialized,
            };

            dest.status = Some(StatusCode {
                value: code_val as i32,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 4. Service Type  --------------------------------------------------
        if let Some(service_type) = src.service_type {
            dest.service_type.push(CodeableReference {
                concept: Some(CodeableConcept {
                    coding: vec![Coding {
                        system: Some(Uri {
                            value: "urn:arsmedicatech:service_type".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: service_type,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 5. Description  ---------------------------------------------------
        if let Some(description) = src.description {
            dest.description = Some(String {
                value: description,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Start and End Times  -------------------------------------------
        if let Some(start_time) = src.start_time {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&start_time) {
                let instant = dt.with_timezone(&Utc);
                dest.start = Some(Instant {
                    value_us: instant.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        if let Some(end_time) = src.end_time {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&end_time) {
                let instant = dt.with_timezone(&Utc);
                dest.end = Some(Instant {
                    value_us: instant.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 7. Duration  ------------------------------------------------------
        if let Some(duration) = src.duration_minutes {
            dest.minutes_duration = Some(PositiveInt {
                value: duration as u32,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 8. Reason  --------------------------------------------------------
        if let Some(reason) = src.reason {
            dest.reason.push(CodeableReference {
                concept: Some(CodeableConcept {
                    text: Some(String {
                        value: reason,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 9. Priority  ------------------------------------------------------
        if let Some(priority) = src.priority {
            dest.priority = Some(CodeableConcept {
                text: Some(String {
                    value: priority,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Comments/Notes  -----------------------------------------------
        if let Some(comments) = src.comments {
            dest.note.push(Annotation {
                text: Some(String {
                    value: comments,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 11. Created Date  -------------------------------------------------
        if let Some(created_date) = src.created_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&created_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.created = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 12. Cancellation Reason  ------------------------------------------
        if let Some(cancellation_reason) = src.cancellation_reason {
            dest.cancellation_reason = Some(CodeableConcept {
                text: Some(String {
                    value: cancellation_reason,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 13. Cancellation Date  --------------------------------------------
        if let Some(cancellation_date) = src.cancellation_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&cancellation_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.cancellation_date = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Subject (Patient)  --------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 15. Participants  -------------------------------------------------
        // Add patient as participant
        dest.participant.push(Participant {
            actor: Some(Reference {
                reference: Some(String {
                    value: format!("Patient/{}", src.patient_demographic_no),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            required: Some(crate::proto::google::fhir::proto::r5::core::Boolean {
                value: true,
                ..Default::default()
            }),
            ..Default::default()
        });

        // Add practitioner as participant if available
        if let Some(practitioner_id) = src.practitioner_id {
            dest.participant.push(Participant {
                actor: Some(Reference {
                    reference: Some(String {
                        value: format!("Practitioner/{}", practitioner_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                required: Some(crate::proto::google::fhir::proto::r5::core::Boolean {
                    value: true,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // Add location as participant if available
        if let Some(location_id) = src.location_id {
            dest.participant.push(Participant {
                actor: Some(Reference {
                    reference: Some(String {
                        value: format!("Location/{}", location_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                required: Some(crate::proto::google::fhir::proto::r5::core::Boolean {
                    value: false,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        dest
    }
}

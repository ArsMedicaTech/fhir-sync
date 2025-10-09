use chrono::{DateTime, Utc};
use crate::domain::encounter::DomainEncounter;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    Encounter, // proto message we're producing
    Id,
    Identifier,
    Uri,
    String,
    CodeableConcept,
    Coding,
    Code,
    Reference,
    DateTime as FhirDateTime,
    Period,
    Duration,
    Decimal,
    CodeableReference,
};

use crate::proto::google::fhir::proto::r5::core::encounter_status_code::Value as EncounterStatusCode;
use crate::proto::google::fhir::proto::r5::core::encounter;

// Shorthand for nested message that lives *inside* Encounter.
type StatusCode = encounter::StatusCode;
type Participant = encounter::Participant;
type Reason = encounter::Reason;
type Diagnosis = encounter::Diagnosis;
type Location = encounter::Location;
type Admission = encounter::Admission;

impl From<DomainEncounter> for Encounter {
    fn from(src: DomainEncounter) -> Self {
        // Start with a completely empty message
        let mut dest = Encounter::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.encounter_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:encounter_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.encounter_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let code_val = match status.to_lowercase().as_str() {
                "planned"         => EncounterStatusCode::Planned,
                "in-progress"     => EncounterStatusCode::InProgress,
                "on-hold"         => EncounterStatusCode::OnHold,
                "discharged"      => EncounterStatusCode::Discharged,
                "completed"       => EncounterStatusCode::Completed,
                "cancelled"       => EncounterStatusCode::Cancelled,
                "discontinued"    => EncounterStatusCode::Discontinued,
                "entered-in-error" => EncounterStatusCode::EnteredInError,
                "unknown"         => EncounterStatusCode::Unknown,
                _                 => EncounterStatusCode::InvalidUninitialized,
            };

            dest.status = Some(StatusCode {
                value: code_val as i32,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 4. Class (Encounter Class)  ---------------------------------------
        if let Some(class_code) = src.class_code {
            dest.class_value.push(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: class_code.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match class_code.as_str() {
                            "inpatient" => "Inpatient",
                            "outpatient" => "Outpatient",
                            "emergency" => "Emergency",
                            "ambulatory" => "Ambulatory",
                            "wellness" => "Wellness",
                            "urgentcare" => "Urgent Care",
                            "virtual" => "Virtual",
                            _ => &class_code,
                        }.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: class_code,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 5. Priority  ------------------------------------------------------
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
        // 6. Type (Encounter Type)  -----------------------------------------
        if let Some(encounter_type) = src.encounter_type {
            dest.r#type.push(CodeableConcept {
                text: Some(String {
                    value: encounter_type,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Service Type  --------------------------------------------------
        if let Some(service_type) = src.service_type {
            dest.service_type.push(CodeableReference {
                concept: Some(CodeableConcept {
                    text: Some(String {
                        value: service_type,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 8. Subject (Patient)  --------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 9. Subject Status  ------------------------------------------------
        if let Some(subject_status) = src.subject_status {
            dest.subject_status = Some(CodeableConcept {
                text: Some(String {
                    value: subject_status,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Planned Dates  ------------------------------------------------
        if let Some(planned_start) = src.planned_start_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&planned_start) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.planned_start_date = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        if let Some(planned_end) = src.planned_end_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&planned_end) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.planned_end_date = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 11. Actual Period  ------------------------------------------------
        if src.actual_start_date.is_some() || src.actual_end_date.is_some() {
            let mut period = Period {
                ..Default::default()
            };

            if let Some(actual_start) = src.actual_start_date {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&actual_start) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.start = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            if let Some(actual_end) = src.actual_end_date {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&actual_end) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.end = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.actual_period = Some(period);
        }

        // ------------------------------------------------------------------
        // 12. Length (Duration)  --------------------------------------------
        if let Some(length_minutes) = src.length_minutes {
            dest.length = Some(Duration {
                value: Some(Decimal {
                    value: length_minutes as f64,
                    ..Default::default()
                }),
                unit: Some(String {
                    value: "min".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 13. Participants  -------------------------------------------------
        if let Some(practitioner_id) = src.practitioner_id {
            dest.participant.push(Participant {
                r#type: vec![CodeableConcept {
                    text: Some(String {
                        value: "Primary Practitioner".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                actor: Some(Reference {
                    reference: Some(String {
                        value: format!("Practitioner/{}", practitioner_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        if let Some(location_id) = src.location_id {
            dest.participant.push(Participant {
                r#type: vec![CodeableConcept {
                    text: Some(String {
                        value: "Location".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                actor: Some(Reference {
                    reference: Some(String {
                        value: format!("Location/{}", location_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 14. Service Provider  ---------------------------------------------
        if let Some(service_provider_id) = src.service_provider_id {
            dest.service_provider = Some(Reference {
                reference: Some(String {
                    value: format!("Organization/{}", service_provider_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 15. Appointment Reference  ----------------------------------------
        if let Some(appointment_id) = src.appointment_id {
            dest.appointment.push(Reference {
                reference: Some(String {
                    value: format!("Appointment/{}", appointment_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 16. Part Of Encounter  --------------------------------------------
        if let Some(part_of_encounter_id) = src.part_of_encounter_id {
            dest.part_of = Some(Reference {
                reference: Some(String {
                    value: format!("Encounter/{}", part_of_encounter_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 17. Episode of Care  ----------------------------------------------
        if let Some(episode_of_care_id) = src.episode_of_care_id {
            dest.episode_of_care.push(Reference {
                reference: Some(String {
                    value: format!("EpisodeOfCare/{}", episode_of_care_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 18. Care Team  ----------------------------------------------------
        if let Some(care_team_ids) = src.care_team_ids {
            for care_team_id in care_team_ids {
                dest.care_team.push(Reference {
                    reference: Some(String {
                        value: format!("CareTeam/{}", care_team_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 19. Reasons  ------------------------------------------------------
        if let Some(reason_codes) = src.reason_codes {
            for (i, reason_code) in reason_codes.iter().enumerate() {
                let mut reason = Reason {
                    ..Default::default()
                };

                reason.value.push(CodeableReference {
                    concept: Some(CodeableConcept {
                        coding: vec![Coding {
                            system: Some(Uri {
                                value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                                ..Default::default()
                            }),
                            code: Some(Code {
                                value: reason_code.clone(),
                                ..Default::default()
                            }),
                            display: src.reason_descriptions.as_ref()
                                .and_then(|descriptions| descriptions.get(i))
                                .map(|description| String {
                                    value: description.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                dest.reason.push(reason);
            }
        }

        // ------------------------------------------------------------------
        // 20. Diagnoses  ----------------------------------------------------
        if let Some(diagnosis_codes) = src.diagnosis_codes {
            for (i, diagnosis_code) in diagnosis_codes.iter().enumerate() {
                let mut diagnosis = Diagnosis {
                    ..Default::default()
                };

                diagnosis.condition = Some(Reference {
                    reference: Some(String {
                        value: format!("Condition/{}", diagnosis_code),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                diagnosis.r#use = Some(CodeableConcept {
                    text: Some(String {
                        value: "Primary".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                if let Some(ranks) = &src.diagnosis_ranks {
                    if let Some(rank) = ranks.get(i) {
                        diagnosis.rank = Some(crate::proto::google::fhir::proto::r5::core::PositiveInt {
                            value: *rank as u32,
                            ..Default::default()
                        });
                    }
                }

                dest.diagnosis.push(diagnosis);
            }
        }

        // ------------------------------------------------------------------
        // 21. Account References  -------------------------------------------
        if let Some(account_ids) = src.account_ids {
            for account_id in account_ids {
                dest.account.push(Reference {
                    reference: Some(String {
                        value: format!("Account/{}", account_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 22. Diet Preferences  ---------------------------------------------
        if let Some(diet_preferences) = src.diet_preferences {
            for diet_pref in diet_preferences {
                dest.diet_preference.push(CodeableConcept {
                    text: Some(String {
                        value: diet_pref,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 23. Special Arrangements  -----------------------------------------
        if let Some(special_arrangements) = src.special_arrangements {
            for arrangement in special_arrangements {
                dest.special_arrangement.push(CodeableConcept {
                    text: Some(String {
                        value: arrangement,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 24. Special Courtesies  -------------------------------------------
        if let Some(special_courtesies) = src.special_courtesies {
            for courtesy in special_courtesies {
                dest.special_courtesy.push(CodeableConcept {
                    text: Some(String {
                        value: courtesy,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 25. Admission Information  ----------------------------------------
        if src.admission_source.is_some() || src.admission_diagnosis.is_some() || 
           src.discharge_disposition.is_some() || src.discharge_diagnosis.is_some() {
            let mut admission = Admission {
                ..Default::default()
            };

            if let Some(admission_source) = src.admission_source {
                admission.source = Some(CodeableConcept {
                    text: Some(String {
                        value: admission_source,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            if let Some(admission_diagnosis) = src.admission_diagnosis {
                admission.diagnosis = Some(Reference {
                    reference: Some(String {
                        value: format!("Condition/{}", admission_diagnosis),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            if let Some(discharge_disposition) = src.discharge_disposition {
                admission.destination = Some(Reference {
                    reference: Some(String {
                        value: format!("Location/{}", discharge_disposition),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.admission = Some(admission);
        }

        // ------------------------------------------------------------------
        // 26. Location  -----------------------------------------------------
        if let Some(location_id) = src.location_id {
            dest.location.push(Location {
                location: Some(Reference {
                    reference: Some(String {
                        value: format!("Location/{}", location_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        dest
    }
}

use chrono::{DateTime, Utc};
use crate::domain::condition::DomainCondition;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    Condition, // proto message we're producing
    Id,
    Identifier,
    Uri,
    String,
    CodeableConcept,
    Coding,
    Code,
    Reference,
    DateTime as FhirDateTime,
    Annotation,
    CodeableReference,
};

use crate::proto::google::fhir::proto::r5::core::condition;

// Shorthand for nested message that lives *inside* Condition.
type OnsetX = condition::OnsetX;
type AbatementX = condition::AbatementX;
type Participant = condition::Participant;
type Stage = condition::Stage;

impl From<DomainCondition> for Condition {
    fn from(src: DomainCondition) -> Self {
        // Start with a completely empty message
        let mut dest = Condition::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.condition_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:condition_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.condition_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Clinical Status  -----------------------------------------------
        if let Some(status) = src.clinical_status {
            dest.clinical_status = Some(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/condition-clinical".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: status.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match status.as_str() {
                            "active" => "Active",
                            "recurrence" => "Recurrence",
                            "relapse" => "Relapse",
                            "inactive" => "Inactive",
                            "remission" => "Remission",
                            "resolved" => "Resolved",
                            "unknown" => "Unknown",
                            _ => &status,
                        }.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: status,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 4. Verification Status  -------------------------------------------
        if let Some(verification_status) = src.verification_status {
            dest.verification_status = Some(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/condition-ver-status".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: verification_status.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match verification_status.as_str() {
                            "unconfirmed" => "Unconfirmed",
                            "provisional" => "Provisional",
                            "differential" => "Differential",
                            "confirmed" => "Confirmed",
                            "refuted" => "Refuted",
                            "entered-in-error" => "Entered in Error",
                            _ => &verification_status,
                        }.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: verification_status,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 5. Category  ------------------------------------------------------
        if let Some(category) = src.category {
            dest.category.push(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/condition-category".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: category.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match category.as_str() {
                            "problem-list-item" => "Problem List Item",
                            "encounter-diagnosis" => "Encounter Diagnosis",
                            _ => &category,
                        }.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: category,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Severity  ------------------------------------------------------
        if let Some(severity) = src.severity {
            dest.severity = Some(CodeableConcept {
                text: Some(String {
                    value: severity,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Code (Condition Code)  -----------------------------------------
        if let Some(code) = src.code {
            let mut codeable_concept = CodeableConcept {
                ..Default::default()
            };

            // Add coding if we have a code
            codeable_concept.coding.push(Coding {
                system: Some(Uri {
                    value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                    ..Default::default()
                }),
                code: Some(Code {
                    value: code,
                    ..Default::default()
                }),
                display: src.code_display.map(|display| String {
                    value: display,
                    ..Default::default()
                }),
                ..Default::default()
            });

            dest.code = Some(codeable_concept);
        }

        // ------------------------------------------------------------------
        // 8. Body Site  -----------------------------------------------------
        if let Some(body_site) = src.body_site {
            dest.body_site.push(CodeableConcept {
                text: Some(String {
                    value: body_site,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 9. Subject (Patient)  --------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 10. Encounter  ----------------------------------------------------
        if let Some(encounter_id) = src.encounter_id {
            dest.encounter = Some(Reference {
                reference: Some(String {
                    value: format!("Encounter/{}", encounter_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 11. Onset  --------------------------------------------------------
        if let Some(onset_date) = src.onset_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&onset_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.onset = Some(OnsetX {
                    choice: Some(condition::onset_x::Choice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    })),
                });
            }
        } else if let Some(onset_age) = src.onset_age {
            dest.onset = Some(OnsetX {
                choice: Some(condition::onset_x::Choice::StringValue(String {
                    value: onset_age,
                    ..Default::default()
                })),
            });
        } else if let Some(onset_description) = src.onset_description {
            dest.onset = Some(OnsetX {
                choice: Some(condition::onset_x::Choice::StringValue(String {
                    value: onset_description,
                    ..Default::default()
                })),
            });
        }

        // ------------------------------------------------------------------
        // 12. Abatement  ----------------------------------------------------
        if let Some(abatement_date) = src.abatement_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&abatement_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.abatement = Some(AbatementX {
                    choice: Some(condition::abatement_x::Choice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    })),
                });
            }
        } else if let Some(abatement_age) = src.abatement_age {
            dest.abatement = Some(AbatementX {
                choice: Some(condition::abatement_x::Choice::StringValue(String {
                    value: abatement_age,
                    ..Default::default()
                })),
            });
        } else if let Some(abatement_description) = src.abatement_description {
            dest.abatement = Some(AbatementX {
                choice: Some(condition::abatement_x::Choice::StringValue(String {
                    value: abatement_description,
                    ..Default::default()
                })),
            });
        }

        // ------------------------------------------------------------------
        // 13. Recorded Date  ------------------------------------------------
        if let Some(recorded_date) = src.recorded_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&recorded_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.recorded_date = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Participants  -------------------------------------------------
        if let Some(practitioner_id) = src.practitioner_id {
            dest.participant.push(Participant {
                function: Some(CodeableConcept {
                    text: Some(String {
                        value: "Diagnosing Practitioner".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
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

        // ------------------------------------------------------------------
        // 15. Stage  --------------------------------------------------------
        if src.stage_summary.is_some() || src.stage_type.is_some() || src.stage_assessment_ids.is_some() {
            let mut stage = Stage {
                ..Default::default()
            };

            if let Some(stage_summary) = src.stage_summary {
                stage.summary = Some(CodeableConcept {
                    text: Some(String {
                        value: stage_summary,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            if let Some(stage_type) = src.stage_type {
                stage.r#type = Some(CodeableConcept {
                    text: Some(String {
                        value: stage_type,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            if let Some(assessment_ids) = src.stage_assessment_ids {
                for assessment_id in assessment_ids {
                    stage.assessment.push(Reference {
                        reference: Some(String {
                            value: format!("Observation/{}", assessment_id),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }
            }

            dest.stage.push(stage);
        }

        // ------------------------------------------------------------------
        // 16. Evidence  -----------------------------------------------------
        if let Some(evidence_codes) = src.evidence_codes {
            for (i, evidence_code) in evidence_codes.iter().enumerate() {
                let mut evidence = CodeableReference {
                    ..Default::default()
                };

                evidence.concept = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: Some(Uri {
                            value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: evidence_code.clone(),
                            ..Default::default()
                        }),
                        display: src.evidence_descriptions.as_ref()
                            .and_then(|descriptions| descriptions.get(i))
                            .map(|description| String {
                                value: description.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    ..Default::default()
                });

                dest.evidence.push(evidence);
            }
        }

        // ------------------------------------------------------------------
        // 17. Notes  --------------------------------------------------------
        if let Some(notes) = src.notes {
            dest.note.push(Annotation {
                text: Some(String {
                    value: notes,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        dest
    }
}

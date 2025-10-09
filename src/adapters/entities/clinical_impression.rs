use chrono::{DateTime, Utc};
use crate::domain::clinical_impression::DomainClinicalImpression;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: ClinicalImpression struct may not be generated yet
    // We'll use a placeholder for now and update when the struct is available
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
    Annotation,
    CodeableReference,
};

// Placeholder for ClinicalImpression until it's generated
// This will be replaced with the actual struct when available
pub struct ClinicalImpression {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub status: Option<StatusCode>,
    pub status_reason: Option<CodeableConcept>,
    pub description: Option<String>,
    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub effective: Option<EffectiveX>,
    pub date: Option<FhirDateTime>,
    pub performer: Option<Reference>,
    pub previous: Option<Reference>,
    pub problem: Vec<Reference>,
    pub change_pattern: Option<CodeableConcept>,
    pub protocol: Vec<Uri>,
    pub summary: Option<String>,
    pub finding: Vec<Finding>,
    pub prognosis_codeable_concept: Vec<CodeableConcept>,
    pub prognosis_reference: Vec<Reference>,
    pub supporting_info: Vec<Reference>,
    pub note: Vec<Annotation>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct EffectiveX {
    pub choice: Option<EffectiveChoice>,
}

pub enum EffectiveChoice {
    DateTime(FhirDateTime),
    Period(Period),
}

pub struct Finding {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub item: Option<CodeableReference>,
    pub basis: Option<String>,
}

pub struct Extension {
    pub id: Option<String>,
    pub url: String,
    pub value: Option<ExtensionValue>,
}

pub enum ExtensionValue {
    String(String),
    CodeableConcept(CodeableConcept),
    // Add other types as needed
}

impl Default for ClinicalImpression {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            status: None,
            status_reason: None,
            description: None,
            subject: None,
            encounter: None,
            effective: None,
            date: None,
            performer: None,
            previous: None,
            problem: Vec::new(),
            change_pattern: None,
            protocol: Vec::new(),
            summary: None,
            finding: Vec::new(),
            prognosis_codeable_concept: Vec::new(),
            prognosis_reference: Vec::new(),
            supporting_info: Vec::new(),
            note: Vec::new(),
        }
    }
}

impl From<DomainClinicalImpression> for ClinicalImpression {
    fn from(src: DomainClinicalImpression) -> Self {
        // Start with a completely empty message
        let mut dest = ClinicalImpression::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.clinical_impression_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:clinical_impression_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.clinical_impression_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "preparation" => 1,
                "in-progress" => 2,
                "not-done" => 3,
                "on-hold" => 4,
                "stopped" => 5,
                "completed" => 6,
                "entered-in-error" => 7,
                "unknown" => 8,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 4. Status Reason  -------------------------------------------------
        if let Some(status_reason) = src.status_reason {
            dest.status_reason = Some(CodeableConcept {
                text: Some(String {
                    value: status_reason,
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
        // 6. Subject (Patient)  --------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 7. Encounter  -----------------------------------------------------
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
        // 8. Effective (Assessment Time)  -----------------------------------
        if let Some(effective_date) = src.effective_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&effective_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.effective = Some(EffectiveX {
                    choice: Some(EffectiveChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    })),
                });
            }
        } else if src.effective_period_start.is_some() || src.effective_period_end.is_some() {
            let mut period = Period {
                ..Default::default()
            };

            if let Some(period_start) = src.effective_period_start {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.start = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            if let Some(period_end) = src.effective_period_end {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.end = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.effective = Some(EffectiveX {
                choice: Some(EffectiveChoice::Period(period)),
            });
        }

        // ------------------------------------------------------------------
        // 9. Date (Documented Date)  ----------------------------------------
        if let Some(documented_date) = src.documented_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&documented_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.date = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 10. Performer  ----------------------------------------------------
        if let Some(performer_id) = src.performer_id {
            let performer_type = src.performer_type.unwrap_or_else(|| "Practitioner".to_string());
            dest.performer = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", performer_type, performer_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 11. Previous  -----------------------------------------------------
        if let Some(previous_impression_id) = src.previous_impression_id {
            dest.previous = Some(Reference {
                reference: Some(String {
                    value: format!("ClinicalImpression/{}", previous_impression_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 12. Problem (Conditions)  -----------------------------------------
        if let Some(problem_condition_ids) = src.problem_condition_ids {
            for condition_id in problem_condition_ids {
                dest.problem.push(Reference {
                    reference: Some(String {
                        value: format!("Condition/{}", condition_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 13. Problem (Allergies)  ------------------------------------------
        if let Some(problem_allergy_ids) = src.problem_allergy_ids {
            for allergy_id in problem_allergy_ids {
                dest.problem.push(Reference {
                    reference: Some(String {
                        value: format!("AllergyIntolerance/{}", allergy_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Change Pattern  -----------------------------------------------
        if let Some(change_pattern) = src.change_pattern {
            dest.change_pattern = Some(CodeableConcept {
                text: Some(String {
                    value: change_pattern,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 15. Protocol  -----------------------------------------------------
        if let Some(protocol_uris) = src.protocol_uris {
            for protocol_uri in protocol_uris {
                dest.protocol.push(Uri {
                    value: protocol_uri,
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 16. Summary  ------------------------------------------------------
        if let Some(summary) = src.summary {
            dest.summary = Some(String {
                value: summary,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 17. Findings  -----------------------------------------------------
        if let Some(finding_items) = src.finding_items {
            for (i, item) in finding_items.iter().enumerate() {
                let mut finding = Finding {
                    ..Default::default()
                };

                // Item (What was found)
                finding.item = Some(CodeableReference {
                    concept: Some(CodeableConcept {
                        text: Some(String {
                            value: item.clone(),
                            ..Default::default()
                        }),
                        coding: src.finding_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .and_then(|code| {
                                src.finding_systems.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| vec![Coding {
                                        system: Some(Uri {
                                            value: system.clone(),
                                            ..Default::default()
                                        }),
                                        code: Some(Code {
                                            value: code.clone(),
                                            ..Default::default()
                                        }),
                                        display: src.finding_descriptions.as_ref()
                                            .and_then(|descriptions| descriptions.get(i))
                                            .map(|description| String {
                                                value: description.clone(),
                                                ..Default::default()
                                            }),
                                        ..Default::default()
                                    }])
                            })
                            .unwrap_or_default(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Basis (Which investigations support finding)
                if let Some(finding_basis) = &src.finding_basis {
                    if let Some(basis) = finding_basis.get(i) {
                        finding.basis = Some(String {
                            value: basis.clone(),
                            ..Default::default()
                        });
                    }
                }

                dest.finding.push(finding);
            }
        }

        // ------------------------------------------------------------------
        // 18. Prognosis (CodeableConcept)  ----------------------------------
        if let Some(prognosis_codes) = src.prognosis_codes {
            for (i, code) in prognosis_codes.iter().enumerate() {
                let mut codeable_concept = CodeableConcept {
                    ..Default::default()
                };

                codeable_concept.coding.push(Coding {
                    system: src.prognosis_systems.as_ref()
                        .and_then(|systems| systems.get(i))
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                            ..Default::default()
                        })),
                    code: Some(Code {
                        value: code.clone(),
                        ..Default::default()
                    }),
                    display: src.prognosis_descriptions.as_ref()
                        .and_then(|descriptions| descriptions.get(i))
                        .map(|description| String {
                            value: description.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                });

                dest.prognosis_codeable_concept.push(codeable_concept);
            }
        }

        // ------------------------------------------------------------------
        // 19. Prognosis (Reference)  ----------------------------------------
        if let Some(prognosis_reference_ids) = src.prognosis_reference_ids {
            for prognosis_reference_id in prognosis_reference_ids {
                dest.prognosis_reference.push(Reference {
                    reference: Some(String {
                        value: format!("RiskAssessment/{}", prognosis_reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 20. Supporting Info  ----------------------------------------------
        if let Some(supporting_info_ids) = src.supporting_info_ids {
            for supporting_info_id in supporting_info_ids {
                dest.supporting_info.push(Reference {
                    reference: Some(String {
                        value: format!("Resource/{}", supporting_info_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 21. Notes  --------------------------------------------------------
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

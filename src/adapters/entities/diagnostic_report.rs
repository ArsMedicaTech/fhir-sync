use chrono::{DateTime, Utc};
use crate::domain::diagnostic_report::DomainDiagnosticReport;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: DiagnosticReport struct may not be generated yet
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
    Markdown,
    Attachment,
    Instant,
};

// Placeholder for DiagnosticReport until it's generated
// This will be replaced with the actual struct when available
pub struct DiagnosticReport {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub based_on: Vec<Reference>,
    pub status: Option<StatusCode>,
    pub category: Vec<CodeableConcept>,
    pub code: Option<CodeableConcept>,
    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub effective: Option<EffectiveX>,
    pub issued: Option<Instant>,
    pub performer: Vec<Reference>,
    pub results_interpreter: Vec<Reference>,
    pub specimen: Vec<Reference>,
    pub result: Vec<Reference>,
    pub note: Vec<Annotation>,
    pub study: Vec<Reference>,
    pub supporting_info: Vec<SupportingInfo>,
    pub media: Vec<Media>,
    pub composition: Option<Reference>,
    pub conclusion: Option<Markdown>,
    pub conclusion_code: Vec<CodeableConcept>,
    pub presented_form: Vec<Attachment>,
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

pub struct SupportingInfo {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub r#type: Option<CodeableConcept>,
    pub reference: Option<Reference>,
}

pub struct Media {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub comment: Option<String>,
    pub link: Option<Reference>,
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

impl Default for DiagnosticReport {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            based_on: Vec::new(),
            status: None,
            category: Vec::new(),
            code: None,
            subject: None,
            encounter: None,
            effective: None,
            issued: None,
            performer: Vec::new(),
            results_interpreter: Vec::new(),
            specimen: Vec::new(),
            result: Vec::new(),
            note: Vec::new(),
            study: Vec::new(),
            supporting_info: Vec::new(),
            media: Vec::new(),
            composition: None,
            conclusion: None,
            conclusion_code: Vec::new(),
            presented_form: Vec::new(),
        }
    }
}

impl From<DomainDiagnosticReport> for DiagnosticReport {
    fn from(src: DomainDiagnosticReport) -> Self {
        // Start with a completely empty message
        let mut dest = DiagnosticReport::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.diagnostic_report_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:diagnostic_report_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.diagnostic_report_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Based On  ------------------------------------------------------
        if let Some(based_on_ids) = src.based_on_ids {
            for (i, based_on_id) in based_on_ids.iter().enumerate() {
                let based_on_type = src.based_on_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "ServiceRequest".to_string());

                dest.based_on.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", based_on_type, based_on_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 4. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "registered" => 1,
                "partial" => 2,
                "preliminary" => 3,
                "modified" => 4,
                "final" => 5,
                "amended" => 6,
                "corrected" => 7,
                "appended" => 8,
                "cancelled" => 9,
                "entered-in-error" => 10,
                "unknown" => 11,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 5. Category  ------------------------------------------------------
        if let Some(category) = src.category {
            dest.category.push(CodeableConcept {
                coding: vec![Coding {
                    system: src.category_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v2-0074".to_string(),
                        ..Default::default()
                    })),
                    code: src.category_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.category_display.map(|display| String {
                        value: display,
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
        // 6. Code  ----------------------------------------------------------
        if let Some(code) = src.code {
            dest.code = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.code_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://loinc.org".to_string(),
                        ..Default::default()
                    })),
                    code: src.code_code.map(|code_code| Code {
                        value: code_code,
                        ..Default::default()
                    }),
                    display: src.code_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: code,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Subject (Patient)  --------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 8. Encounter  -----------------------------------------------------
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
        // 9. Effective (Report Time)  ---------------------------------------
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
        // 10. Issued  -------------------------------------------------------
        if let Some(issued_date) = src.issued_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&issued_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.issued = Some(Instant {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 11. Performer  ----------------------------------------------------
        if let Some(performer_ids) = src.performer_ids {
            for (i, performer_id) in performer_ids.iter().enumerate() {
                let performer_type = src.performer_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "Practitioner".to_string());

                dest.performer.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", performer_type, performer_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 12. Results Interpreter  ------------------------------------------
        if let Some(interpreter_ids) = src.results_interpreter_ids {
            for (i, interpreter_id) in interpreter_ids.iter().enumerate() {
                let interpreter_type = src.results_interpreter_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "Practitioner".to_string());

                dest.results_interpreter.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", interpreter_type, interpreter_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 13. Specimen  -----------------------------------------------------
        if let Some(specimen_ids) = src.specimen_ids {
            for specimen_id in specimen_ids {
                dest.specimen.push(Reference {
                    reference: Some(String {
                        value: format!("Specimen/{}", specimen_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Result (Observations)  ----------------------------------------
        if let Some(result_observation_ids) = src.result_observation_ids {
            for result_id in result_observation_ids {
                dest.result.push(Reference {
                    reference: Some(String {
                        value: format!("Observation/{}", result_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 15. Study  --------------------------------------------------------
        if let Some(study_ids) = src.study_ids {
            for (i, study_id) in study_ids.iter().enumerate() {
                let study_type = src.study_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "ImagingStudy".to_string());

                dest.study.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", study_type, study_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 16. Supporting Info  ----------------------------------------------
        if let Some(supporting_info_types) = src.supporting_info_types {
            for (i, info_type) in supporting_info_types.iter().enumerate() {
                let mut supporting_info = SupportingInfo {
                    ..Default::default()
                };

                // Type
                supporting_info.r#type = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.supporting_info_type_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v2-0074".to_string(),
                                ..Default::default()
                            })),
                        code: src.supporting_info_type_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.supporting_info_type_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: info_type.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Reference
                if let Some(reference_ids) = &src.supporting_info_reference_ids {
                    if let Some(reference_id) = reference_ids.get(i) {
                        let reference_type = src.supporting_info_reference_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Observation".to_string());

                        supporting_info.reference = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", reference_type, reference_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.supporting_info.push(supporting_info);
            }
        }

        // ------------------------------------------------------------------
        // 17. Media  --------------------------------------------------------
        if let Some(media_comments) = src.media_comments {
            for (i, comment) in media_comments.iter().enumerate() {
                let mut media_item = Media {
                    ..Default::default()
                };

                // Comment
                media_item.comment = Some(String {
                    value: comment.clone(),
                    ..Default::default()
                });

                // Link
                if let Some(media_link_ids) = &src.media_link_ids {
                    if let Some(link_id) = media_link_ids.get(i) {
                        media_item.link = Some(Reference {
                            reference: Some(String {
                                value: format!("DocumentReference/{}", link_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.media.push(media_item);
            }
        }

        // ------------------------------------------------------------------
        // 18. Composition  --------------------------------------------------
        if let Some(composition_id) = src.composition_id {
            dest.composition = Some(Reference {
                reference: Some(String {
                    value: format!("Composition/{}", composition_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 19. Conclusion  ---------------------------------------------------
        if let Some(conclusion) = src.conclusion {
            dest.conclusion = Some(Markdown {
                value: conclusion,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 20. Conclusion Code  ----------------------------------------------
        if let Some(conclusion_codes) = src.conclusion_codes {
            for (i, conclusion_code) in conclusion_codes.iter().enumerate() {
                let mut codeable_concept = CodeableConcept {
                    ..Default::default()
                };

                codeable_concept.coding.push(Coding {
                    system: src.conclusion_code_systems.as_ref()
                        .and_then(|systems| systems.get(i))
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                            ..Default::default()
                        })),
                    code: src.conclusion_code_codes.as_ref()
                        .and_then(|codes| codes.get(i))
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.conclusion_code_displays.as_ref()
                        .and_then(|displays| displays.get(i))
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                });

                codeable_concept.text = Some(String {
                    value: conclusion_code.clone(),
                    ..Default::default()
                });

                dest.conclusion_code.push(codeable_concept);
            }
        }

        // ------------------------------------------------------------------
        // 21. Presented Form  -----------------------------------------------
        if let Some(presented_form_ids) = src.presented_form_ids {
            for form_id in presented_form_ids {
                dest.presented_form.push(Attachment {
                    id: Some(String {
                        value: form_id,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 22. Notes  --------------------------------------------------------
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

use chrono::{DateTime, Utc};
use crate::domain::allergy_intolerance::DomainAllergyIntolerance;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: AllergyIntolerance struct may not be generated yet
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
    Annotation,
    CodeableReference,
    Age,
    Period,
    Range,
};

// Placeholder for AllergyIntolerance until it's generated
// This will be replaced with the actual struct when available
pub struct AllergyIntolerance {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub clinical_status: Option<CodeableConcept>,
    pub verification_status: Option<CodeableConcept>,
    pub r#type: Option<CodeableConcept>,
    pub category: Vec<CodeableConcept>,
    pub criticality: Option<CodeableConcept>,
    pub code: Option<CodeableConcept>,
    pub patient: Option<Reference>,
    pub encounter: Option<Reference>,
    pub onset: Option<OnsetX>,
    pub recorded_date: Option<FhirDateTime>,
    pub participant: Vec<Participant>,
    pub last_occurrence: Option<FhirDateTime>,
    pub note: Vec<Annotation>,
    pub reaction: Vec<Reaction>,
}

// Placeholder nested types
pub struct OnsetX {
    pub choice: Option<OnsetChoice>,
}

pub enum OnsetChoice {
    DateTime(FhirDateTime),
    Age(Age),
    Period(Period),
    Range(Range),
    StringValue(String),
}

pub struct Participant {
    pub id: Option<String>,
    pub function: Option<CodeableConcept>,
    pub actor: Option<Reference>,
}

pub struct Reaction {
    pub id: Option<String>,
    pub substance: Option<CodeableConcept>,
    pub manifestation: Vec<CodeableReference>,
    pub description: Option<String>,
    pub onset: Option<FhirDateTime>,
    pub severity: Option<CodeableConcept>,
    pub exposure_route: Option<CodeableConcept>,
    pub note: Vec<Annotation>,
}

impl Default for AllergyIntolerance {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            clinical_status: None,
            verification_status: None,
            r#type: None,
            category: Vec::new(),
            criticality: None,
            code: None,
            patient: None,
            encounter: None,
            onset: None,
            recorded_date: None,
            participant: Vec::new(),
            last_occurrence: None,
            note: Vec::new(),
            reaction: Vec::new(),
        }
    }
}

impl From<DomainAllergyIntolerance> for AllergyIntolerance {
    fn from(src: DomainAllergyIntolerance) -> Self {
        // Start with a completely empty message
        let mut dest = AllergyIntolerance::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.allergy_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:allergy_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.allergy_id.clone(),
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
                        value: "http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: status.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match status.as_str() {
                            "active" => "Active",
                            "inactive" => "Inactive",
                            "resolved" => "Resolved",
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
                        value: "http://terminology.hl7.org/CodeSystem/allergyintolerance-verification".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: verification_status.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match verification_status.as_str() {
                            "unconfirmed" => "Unconfirmed",
                            "presumed" => "Presumed",
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
        // 5. Type (Allergy vs Intolerance)  ---------------------------------
        if let Some(allergy_type) = src.allergy_type {
            dest.r#type = Some(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://hl7.org/fhir/allergy-intolerance-type".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: allergy_type.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match allergy_type.as_str() {
                            "allergy" => "Allergy",
                            "intolerance" => "Intolerance",
                            _ => &allergy_type,
                        }.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: allergy_type,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Category  ------------------------------------------------------
        if let Some(category) = src.category {
            dest.category.push(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://hl7.org/fhir/allergy-intolerance-category".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: category.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match category.as_str() {
                            "food" => "Food",
                            "medication" => "Medication",
                            "environment" => "Environment",
                            "biologic" => "Biologic",
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
        // 7. Criticality  ---------------------------------------------------
        if let Some(criticality) = src.criticality {
            dest.criticality = Some(CodeableConcept {
                coding: vec![Coding {
                    system: Some(Uri {
                        value: "http://hl7.org/fhir/allergy-intolerance-criticality".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: criticality.clone(),
                        ..Default::default()
                    }),
                    display: Some(String {
                        value: match criticality.as_str() {
                            "low" => "Low",
                            "high" => "High",
                            "unable-to-assess" => "Unable to Assess",
                            _ => &criticality,
                        }.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: criticality,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 8. Code (Substance)  ----------------------------------------------
        if let Some(substance_code) = src.substance_code {
            let mut codeable_concept = CodeableConcept {
                ..Default::default()
            };

            codeable_concept.coding.push(Coding {
                system: src.substance_system.map(|system| Uri {
                    value: system,
                    ..Default::default()
                }).or_else(|| Some(Uri {
                    value: "http://snomed.info/sct".to_string(),
                    ..Default::default()
                })),
                code: Some(Code {
                    value: substance_code,
                    ..Default::default()
                }),
                display: src.substance_display.map(|display| String {
                    value: display,
                    ..Default::default()
                }),
                ..Default::default()
            });

            dest.code = Some(codeable_concept);
        }

        // ------------------------------------------------------------------
        // 9. Patient Reference  ---------------------------------------------
        dest.patient = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 10. Encounter Reference  ------------------------------------------
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
                    choice: Some(OnsetChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    })),
                });
            }
        } else if let Some(onset_age) = src.onset_age {
            dest.onset = Some(OnsetX {
                choice: Some(OnsetChoice::StringValue(String {
                    value: onset_age,
                    ..Default::default()
                })),
            });
        } else if let Some(onset_description) = src.onset_description {
            dest.onset = Some(OnsetX {
                choice: Some(OnsetChoice::StringValue(String {
                    value: onset_description,
                    ..Default::default()
                })),
            });
        }

        // ------------------------------------------------------------------
        // 12. Recorded Date  ------------------------------------------------
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
        // 13. Last Occurrence  ----------------------------------------------
        if let Some(last_occurrence_date) = src.last_occurrence_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&last_occurrence_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.last_occurrence = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Participants  -------------------------------------------------
        if let Some(recorder_id) = src.recorder_id {
            let recorder_type = src.recorder_type.unwrap_or_else(|| "Practitioner".to_string());
            dest.participant.push(Participant {
                function: Some(CodeableConcept {
                    text: Some(String {
                        value: "Recorder".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                actor: Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", recorder_type, recorder_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 15. Reactions  ----------------------------------------------------
        if let Some(reaction_substances) = src.reaction_substances {
            for (i, substance) in reaction_substances.iter().enumerate() {
                let mut reaction = Reaction {
                    ..Default::default()
                };

                // Substance
                reaction.substance = Some(CodeableConcept {
                    text: Some(String {
                        value: substance.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Manifestations
                if let Some(manifestations) = &src.reaction_manifestations {
                    for manifestation in manifestations {
                        reaction.manifestation.push(CodeableReference {
                            concept: Some(CodeableConcept {
                                text: Some(String {
                                    value: manifestation.clone(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Description
                if let Some(descriptions) = &src.reaction_descriptions {
                    if let Some(description) = descriptions.get(i) {
                        reaction.description = Some(String {
                            value: description.clone(),
                            ..Default::default()
                        });
                    }
                }

                // Onset
                if let Some(onset_dates) = &src.reaction_onset_dates {
                    if let Some(onset_date) = onset_dates.get(i) {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(onset_date) {
                            let fhir_dt = dt.with_timezone(&Utc);
                            reaction.onset = Some(FhirDateTime {
                                value_us: fhir_dt.timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }
                }

                // Severity
                if let Some(severities) = &src.reaction_severities {
                    if let Some(severity) = severities.get(i) {
                        reaction.severity = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: Some(Uri {
                                    value: "http://hl7.org/fhir/reaction-event-severity".to_string(),
                                    ..Default::default()
                                }),
                                code: Some(Code {
                                    value: severity.clone(),
                                    ..Default::default()
                                }),
                                display: Some(String {
                                    value: match severity.as_str() {
                                        "mild" => "Mild",
                                        "moderate" => "Moderate",
                                        "severe" => "Severe",
                                        _ => severity,
                                    }.to_string(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: severity.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Exposure Route
                if let Some(exposure_routes) = &src.reaction_exposure_routes {
                    if let Some(exposure_route) = exposure_routes.get(i) {
                        reaction.exposure_route = Some(CodeableConcept {
                            text: Some(String {
                                value: exposure_route.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Notes
                if let Some(notes) = &src.reaction_notes {
                    if let Some(note) = notes.get(i) {
                        reaction.note.push(Annotation {
                            text: Some(String {
                                value: note.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.reaction.push(reaction);
            }
        }

        // ------------------------------------------------------------------
        // 16. Notes  --------------------------------------------------------
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

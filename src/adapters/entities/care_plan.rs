use chrono::{DateTime, Utc};
use crate::domain::care_plan::DomainCarePlan;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: CarePlan struct may not be generated yet
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
    Canonical,
};

// Placeholder for CarePlan until it's generated
// This will be replaced with the actual struct when available
pub struct CarePlan {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub instantiates_canonical: Vec<Canonical>,
    pub instantiates_uri: Vec<Uri>,
    pub based_on: Vec<Reference>,
    pub replaces: Vec<Reference>,
    pub part_of: Vec<Reference>,
    pub status: Option<StatusCode>,
    pub intent: Option<IntentCode>,
    pub category: Vec<CodeableConcept>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub period: Option<Period>,
    pub created: Option<FhirDateTime>,
    pub custodian: Option<Reference>,
    pub contributor: Vec<Reference>,
    pub care_team: Vec<Reference>,
    pub addresses: Vec<CodeableReference>,
    pub supporting_info: Vec<Reference>,
    pub goal: Vec<Reference>,
    pub activity: Vec<Activity>,
    pub note: Vec<Annotation>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct IntentCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct Activity {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub performed_activity: Vec<CodeableReference>,
    pub progress: Vec<Annotation>,
    pub planned_activity_reference: Option<Reference>,
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

impl Default for CarePlan {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            instantiates_canonical: Vec::new(),
            instantiates_uri: Vec::new(),
            based_on: Vec::new(),
            replaces: Vec::new(),
            part_of: Vec::new(),
            status: None,
            intent: None,
            category: Vec::new(),
            title: None,
            description: None,
            subject: None,
            encounter: None,
            period: None,
            created: None,
            custodian: None,
            contributor: Vec::new(),
            care_team: Vec::new(),
            addresses: Vec::new(),
            supporting_info: Vec::new(),
            goal: Vec::new(),
            activity: Vec::new(),
            note: Vec::new(),
        }
    }
}

impl From<DomainCarePlan> for CarePlan {
    fn from(src: DomainCarePlan) -> Self {
        // Start with a completely empty message
        let mut dest = CarePlan::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.care_plan_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:care_plan_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.care_plan_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "draft" => 1,
                "active" => 2,
                "on-hold" => 3,
                "revoked" => 4,
                "completed" => 5,
                "entered-in-error" => 6,
                "unknown" => 7,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 4. Intent  --------------------------------------------------------
        if let Some(intent) = src.intent {
            let intent_value = match intent.to_lowercase().as_str() {
                "proposal" => 1,
                "plan" => 2,
                "order" => 3,
                "option" => 4,
                "directive" => 5,
                _ => 0,
            };

            dest.intent = Some(IntentCode {
                value: intent_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 5. Category  ------------------------------------------------------
        if let Some(category) = src.category {
            dest.category.push(CodeableConcept {
                text: Some(String {
                    value: category,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Title  ---------------------------------------------------------
        if let Some(title) = src.title {
            dest.title = Some(String {
                value: title,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Description  ---------------------------------------------------
        if let Some(description) = src.description {
            dest.description = Some(String {
                value: description,
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
        // 9. Encounter  -----------------------------------------------------
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
        // 10. Period  -------------------------------------------------------
        if src.period_start.is_some() || src.period_end.is_some() {
            let mut period = Period {
                ..Default::default()
            };

            if let Some(period_start) = src.period_start {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.start = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            if let Some(period_end) = src.period_end {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.end = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.period = Some(period);
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
        // 12. Custodian  ----------------------------------------------------
        if let Some(custodian_id) = src.custodian_id {
            let custodian_type = src.custodian_type.unwrap_or_else(|| "Practitioner".to_string());
            dest.custodian = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", custodian_type, custodian_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 13. Contributors  -------------------------------------------------
        if let Some(contributor_ids) = src.contributor_ids {
            for (i, contributor_id) in contributor_ids.iter().enumerate() {
                let contributor_type = src.contributor_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "Practitioner".to_string());

                dest.contributor.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", contributor_type, contributor_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Care Team  ----------------------------------------------------
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
        // 15. Based On  -----------------------------------------------------
        if let Some(based_on_ids) = src.based_on_ids {
            for (i, based_on_id) in based_on_ids.iter().enumerate() {
                let based_on_type = src.based_on_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "CarePlan".to_string());

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
        // 16. Replaces  -----------------------------------------------------
        if let Some(replaces_ids) = src.replaces_ids {
            for replaces_id in replaces_ids {
                dest.replaces.push(Reference {
                    reference: Some(String {
                        value: format!("CarePlan/{}", replaces_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 17. Part Of  ------------------------------------------------------
        if let Some(part_of_ids) = src.part_of_ids {
            for part_of_id in part_of_ids {
                dest.part_of.push(Reference {
                    reference: Some(String {
                        value: format!("CarePlan/{}", part_of_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 18. Addresses (Health Issues)  ------------------------------------
        if let Some(addresses_codes) = src.addresses_codes {
            for (i, code) in addresses_codes.iter().enumerate() {
                let mut codeable_ref = CodeableReference {
                    ..Default::default()
                };

                codeable_ref.concept = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: Some(Uri {
                            value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                        display: src.addresses_descriptions.as_ref()
                            .and_then(|descriptions| descriptions.get(i))
                            .map(|description| String {
                                value: description.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    ..Default::default()
                });

                dest.addresses.push(codeable_ref);
            }
        }

        // ------------------------------------------------------------------
        // 19. Supporting Info  ----------------------------------------------
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
        // 20. Goals  --------------------------------------------------------
        if let Some(goal_ids) = src.goal_ids {
            for goal_id in goal_ids {
                dest.goal.push(Reference {
                    reference: Some(String {
                        value: format!("Goal/{}", goal_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 21. Activities  ---------------------------------------------------
        if let Some(activity_descriptions) = src.activity_descriptions {
            for (i, description) in activity_descriptions.iter().enumerate() {
                let mut activity = Activity {
                    ..Default::default()
                };

                // Performed Activity
                activity.performed_activity.push(CodeableReference {
                    concept: Some(CodeableConcept {
                        text: Some(String {
                            value: description.clone(),
                            ..Default::default()
                        }),
                        coding: src.activity_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| vec![Coding {
                                system: Some(Uri {
                                    value: "http://arsmedicatech.com/care-plan-activities".to_string(),
                                    ..Default::default()
                                }),
                                code: Some(Code {
                                    value: code.clone(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }])
                            .unwrap_or_default(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Progress Notes
                if let Some(progress_notes) = &src.activity_progress_notes {
                    if let Some(progress_note) = progress_notes.get(i) {
                        activity.progress.push(Annotation {
                            text: Some(String {
                                value: progress_note.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Planned Activity Reference
                if let Some(activity_references) = &src.activity_references {
                    if let Some(activity_reference) = activity_references.get(i) {
                        let reference_type = src.activity_reference_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Task".to_string());

                        activity.planned_activity_reference = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", reference_type, activity_reference),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.activity.push(activity);
            }
        }

        // ------------------------------------------------------------------
        // 22. Instantiates Canonical  ---------------------------------------
        if let Some(instantiates_canonical) = src.instantiates_canonical {
            for canonical in instantiates_canonical {
                dest.instantiates_canonical.push(Canonical {
                    value: canonical,
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 23. Instantiates URI  ---------------------------------------------
        if let Some(instantiates_uri) = src.instantiates_uri {
            for uri in instantiates_uri {
                dest.instantiates_uri.push(Uri {
                    value: uri,
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 24. Notes  --------------------------------------------------------
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

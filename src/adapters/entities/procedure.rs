use chrono::{DateTime, Utc};
use crate::domain::procedure::DomainProcedure;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Procedure struct may not be generated yet
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
    Boolean,
    Quantity,
    Ratio,
    CodeableReference,
    Annotation,
    Markdown,
    Period,
    UnsignedInt,
    Duration,
    SimpleQuantity,
    Decimal,
    Age,
    Range,
    Timing,
    Canonical,
};

// Placeholder for Procedure until it's generated
// This will be replaced with the actual struct when available
pub struct Procedure {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub instantiates_canonical: Vec<Canonical>,
    pub instantiates_uri: Vec<Uri>,
    pub based_on: Vec<Reference>,
    pub part_of: Vec<Reference>,
    pub status: Option<StatusCode>,
    pub status_reason: Option<CodeableConcept>,
    pub category: Vec<CodeableConcept>,
    pub code: Option<CodeableConcept>,
    pub subject: Option<Reference>,
    pub focus: Option<Reference>,
    pub encounter: Option<Reference>,
    pub occurrence: Option<OccurrenceX>,
    pub recorded: Option<FhirDateTime>,
    pub recorder: Option<Reference>,
    pub reported: Option<ReportedX>,
    pub performer: Vec<Performer>,
    pub location: Option<Reference>,
    pub reason: Vec<CodeableReference>,
    pub body_site: Vec<CodeableConcept>,
    pub outcome: Option<CodeableConcept>,
    pub report: Vec<Reference>,
    pub complication: Vec<CodeableReference>,
    pub follow_up: Vec<CodeableConcept>,
    pub note: Vec<Annotation>,
    pub focal_device: Vec<FocalDevice>,
    pub used: Vec<CodeableReference>,
    pub supporting_info: Vec<Reference>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct OccurrenceX {
    pub choice: Option<OccurrenceChoice>,
}

pub enum OccurrenceChoice {
    DateTime(FhirDateTime),
    Period(Period),
    String(String),
    Age(Age),
    Range(Range),
    Timing(Timing),
}

pub struct ReportedX {
    pub choice: Option<ReportedChoice>,
}

pub enum ReportedChoice {
    Boolean(Boolean),
    Reference(Reference),
}

pub struct Performer {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub function: Option<CodeableConcept>,
    pub actor: Option<Reference>,
    pub on_behalf_of: Option<Reference>,
    pub period: Option<Period>,
}

pub struct FocalDevice {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub action: Option<CodeableConcept>,
    pub manipulated: Option<Reference>,
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

impl Default for Procedure {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            instantiates_canonical: Vec::new(),
            instantiates_uri: Vec::new(),
            based_on: Vec::new(),
            part_of: Vec::new(),
            status: None,
            status_reason: None,
            category: Vec::new(),
            code: None,
            subject: None,
            focus: None,
            encounter: None,
            occurrence: None,
            recorded: None,
            recorder: None,
            reported: None,
            performer: Vec::new(),
            location: None,
            reason: Vec::new(),
            body_site: Vec::new(),
            outcome: None,
            report: Vec::new(),
            complication: Vec::new(),
            follow_up: Vec::new(),
            note: Vec::new(),
            focal_device: Vec::new(),
            used: Vec::new(),
            supporting_info: Vec::new(),
        }
    }
}

impl From<DomainProcedure> for Procedure {
    fn from(src: DomainProcedure) -> Self {
        // Start with a completely empty message
        let mut dest = Procedure::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:procedure_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Instantiates Canonical  ----------------------------------------
        for instantiates_canonical in src.instantiates_canonical {
            dest.instantiates_canonical.push(Canonical {
                value: instantiates_canonical,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 4. Instantiates URI  ----------------------------------------------
        for instantiates_uri in src.instantiates_uri {
            dest.instantiates_uri.push(Uri {
                value: instantiates_uri,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 5. Based On  ------------------------------------------------------
        for (i, based_on_id) in src.based_on_ids.iter().enumerate() {
            let based_on_type = src.based_on_types.get(i)
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

        // ------------------------------------------------------------------
        // 6. Part Of  -------------------------------------------------------
        for (i, part_of_id) in src.part_of_ids.iter().enumerate() {
            let part_of_type = src.part_of_types.get(i)
                .cloned()
                .unwrap_or_else(|| "Procedure".to_string());

            dest.part_of.push(Reference {
                reference: Some(String {
                    value: format!("{}/{}", part_of_type, part_of_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Status  --------------------------------------------------------
        let status_value = match src.status.to_lowercase().as_str() {
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

        // ------------------------------------------------------------------
        // 8. Status Reason  -------------------------------------------------
        if let Some(status_reason) = src.status_reason {
            dest.status_reason = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.status_reason_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/procedure-status-reason".to_string(),
                        ..Default::default()
                    })),
                    code: src.status_reason_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.status_reason_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: status_reason,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 9. Category  ------------------------------------------------------
        for (i, category) in src.category.iter().enumerate() {
            dest.category.push(CodeableConcept {
                coding: vec![Coding {
                    system: src.category_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/procedure-category".to_string(),
                            ..Default::default()
                        })),
                    code: src.category_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.category_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: category.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Code  ---------------------------------------------------------
        if let Some(code) = src.code {
            dest.code = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.code_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://snomed.info/sct".to_string(),
                        ..Default::default()
                    })),
                    code: Some(Code {
                        value: code,
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
        // 11. Subject  ------------------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("{}/{}", src.subject_type, src.subject_id),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 12. Focus  --------------------------------------------------------
        if let Some(focus_id) = src.focus_id {
            let focus_type = src.focus_type
                .unwrap_or_else(|| "Patient".to_string());

            dest.focus = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", focus_type, focus_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 13. Encounter  ----------------------------------------------------
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
        // 14. Occurrence  ---------------------------------------------------
        if src.occurrence_date_time.is_some() || src.occurrence_period_start.is_some() || 
           src.occurrence_string.is_some() || src.occurrence_age_value.is_some() || 
           src.occurrence_range_low_value.is_some() || src.occurrence_timing_code.is_some() {
            
            let mut occurrence_x = OccurrenceX {
                choice: None,
            };

            // Try DateTime first
            if let Some(occurrence_date_time) = src.occurrence_date_time {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&occurrence_date_time) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    occurrence_x.choice = Some(OccurrenceChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    }));
                }
            }
            // Try Period
            else if src.occurrence_period_start.is_some() || src.occurrence_period_end.is_some() {
                let mut period = Period {
                    ..Default::default()
                };

                if let Some(period_start) = src.occurrence_period_start {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.start = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                if let Some(period_end) = src.occurrence_period_end {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.end = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                occurrence_x.choice = Some(OccurrenceChoice::Period(period));
            }
            // Try String
            else if let Some(occurrence_string) = src.occurrence_string {
                occurrence_x.choice = Some(OccurrenceChoice::String(String {
                    value: occurrence_string,
                    ..Default::default()
                }));
            }
            // Try Age
            else if let Some(age_value) = src.occurrence_age_value {
                let age = Age {
                    value: Some(age_value),
                    unit: src.occurrence_age_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    }),
                    code: Some(Code {
                        value: "a".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                };

                occurrence_x.choice = Some(OccurrenceChoice::Age(age));
            }
            // Try Range
            else if src.occurrence_range_low_value.is_some() || src.occurrence_range_high_value.is_some() {
                let mut range = Range {
                    ..Default::default()
                };

                if let Some(low_value) = src.occurrence_range_low_value {
                    range.low = Some(SimpleQuantity {
                        value: Some(low_value),
                        unit: src.occurrence_range_low_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: "a".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                if let Some(high_value) = src.occurrence_range_high_value {
                    range.high = Some(SimpleQuantity {
                        value: Some(high_value),
                        unit: src.occurrence_range_high_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: "a".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                occurrence_x.choice = Some(OccurrenceChoice::Range(range));
            }
            // Try Timing
            else if let Some(timing_code) = src.occurrence_timing_code {
                let timing = Timing {
                    code: Some(CodeableConcept {
                        coding: vec![Coding {
                            system: src.occurrence_timing_system.map(|system| Uri {
                                value: system,
                                ..Default::default()
                            }).or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/timing-abbreviation".to_string(),
                                ..Default::default()
                            })),
                            code: Some(Code {
                                value: timing_code,
                                ..Default::default()
                            }),
                            display: src.occurrence_timing_display.map(|display| String {
                                value: display,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }],
                        text: Some(String {
                            value: timing_code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                };

                occurrence_x.choice = Some(OccurrenceChoice::Timing(timing));
            }

            dest.occurrence = Some(occurrence_x);
        }

        // ------------------------------------------------------------------
        // 15. Recorded  -----------------------------------------------------
        if let Some(recorded) = src.recorded {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&recorded) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.recorded = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 16. Recorder  -----------------------------------------------------
        if let Some(recorder_id) = src.recorder_id {
            let recorder_type = src.recorder_type
                .unwrap_or_else(|| "Practitioner".to_string());

            dest.recorder = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", recorder_type, recorder_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 17. Reported  -----------------------------------------------------
        if src.reported_boolean.is_some() || src.reported_reference_id.is_some() {
            let mut reported_x = ReportedX {
                choice: None,
            };

            if let Some(reported_boolean) = src.reported_boolean {
                reported_x.choice = Some(ReportedChoice::Boolean(Boolean {
                    value: reported_boolean,
                    ..Default::default()
                }));
            } else if let Some(reported_reference_id) = src.reported_reference_id {
                let reported_reference_type = src.reported_reference_type
                    .unwrap_or_else(|| "Patient".to_string());

                reported_x.choice = Some(ReportedChoice::Reference(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", reported_reference_type, reported_reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }

            dest.reported = Some(reported_x);
        }

        // ------------------------------------------------------------------
        // 18. Performer  ----------------------------------------------------
        for performer in src.performer {
            let mut fhir_performer = Performer {
                ..Default::default()
            };

            // Function
            if let Some(function) = performer.function {
                fhir_performer.function = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: performer.function_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/procedure-performer-function".to_string(),
                            ..Default::default()
                        })),
                        code: performer.function_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: performer.function_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: function,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Actor
            fhir_performer.actor = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", performer.actor_type, performer.actor_id),
                    ..Default::default()
                }),
                ..Default::default()
            });

            // On Behalf Of
            if let Some(on_behalf_of_id) = performer.on_behalf_of_id {
                fhir_performer.on_behalf_of = Some(Reference {
                    reference: Some(String {
                        value: format!("Organization/{}", on_behalf_of_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Period
            if performer.period_start.is_some() || performer.period_end.is_some() {
                let mut period = Period {
                    ..Default::default()
                };

                if let Some(period_start) = performer.period_start {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.start = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                if let Some(period_end) = performer.period_end {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.end = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                fhir_performer.period = Some(period);
            }

            dest.performer.push(fhir_performer);
        }

        // ------------------------------------------------------------------
        // 19. Location  -----------------------------------------------------
        if let Some(location_id) = src.location_id {
            dest.location = Some(Reference {
                reference: Some(String {
                    value: format!("Location/{}", location_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 20. Reason  -------------------------------------------------------
        for (i, reason) in src.reason.iter().enumerate() {
            let mut reason_ref = CodeableReference {
                ..Default::default()
            };

            // Concept
            reason_ref.concept = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.reason_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://snomed.info/sct".to_string(),
                            ..Default::default()
                        })),
                    code: src.reason_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.reason_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: reason.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });

            // Reference
            if let Some(reason_reference_id) = src.reason_reference_id.get(i) {
                let reason_reference_type = src.reason_reference_type.get(i)
                    .cloned()
                    .unwrap_or_else(|| "Condition".to_string());

                reason_ref.reference = Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", reason_reference_type, reason_reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.reason.push(reason_ref);
        }

        // ------------------------------------------------------------------
        // 21. Body Site  ----------------------------------------------------
        for (i, body_site) in src.body_site.iter().enumerate() {
            dest.body_site.push(CodeableConcept {
                coding: vec![Coding {
                    system: src.body_site_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://snomed.info/sct".to_string(),
                            ..Default::default()
                        })),
                    code: src.body_site_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.body_site_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: body_site.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 22. Outcome  ------------------------------------------------------
        if let Some(outcome) = src.outcome {
            dest.outcome = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.outcome_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(),
                        ..Default::default()
                    })),
                    code: src.outcome_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.outcome_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: outcome,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 23. Report  -------------------------------------------------------
        for (i, report_id) in src.report_ids.iter().enumerate() {
            let report_type = src.report_types.get(i)
                .cloned()
                .unwrap_or_else(|| "DiagnosticReport".to_string());

            dest.report.push(Reference {
                reference: Some(String {
                    value: format!("{}/{}", report_type, report_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 24. Complication  -------------------------------------------------
        for (i, complication) in src.complication.iter().enumerate() {
            let mut complication_ref = CodeableReference {
                ..Default::default()
            };

            // Concept
            complication_ref.concept = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.complication_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://snomed.info/sct".to_string(),
                            ..Default::default()
                        })),
                    code: src.complication_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.complication_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: complication.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });

            // Reference
            if let Some(complication_reference_id) = src.complication_reference_id.get(i) {
                let complication_reference_type = src.complication_reference_type.get(i)
                    .cloned()
                    .unwrap_or_else(|| "Condition".to_string());

                complication_ref.reference = Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", complication_reference_type, complication_reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.complication.push(complication_ref);
        }

        // ------------------------------------------------------------------
        // 25. Follow Up  ----------------------------------------------------
        for (i, follow_up) in src.follow_up.iter().enumerate() {
            dest.follow_up.push(CodeableConcept {
                coding: vec![Coding {
                    system: src.follow_up_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/procedure-follow-up".to_string(),
                            ..Default::default()
                        })),
                    code: src.follow_up_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.follow_up_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: follow_up.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 26. Note  ---------------------------------------------------------
        for note_text in src.note {
            dest.note.push(Annotation {
                text: Some(String {
                    value: note_text,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 27. Focal Device  -------------------------------------------------
        for focal_device in src.focal_device {
            let mut fhir_focal_device = FocalDevice {
                ..Default::default()
            };

            // Action
            if let Some(action) = focal_device.action {
                fhir_focal_device.action = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: focal_device.action_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/device-action".to_string(),
                            ..Default::default()
                        })),
                        code: focal_device.action_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: focal_device.action_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: action,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Manipulated
            fhir_focal_device.manipulated = Some(Reference {
                reference: Some(String {
                    value: format!("Device/{}", focal_device.manipulated_id),
                    ..Default::default()
                }),
                ..Default::default()
            });

            dest.focal_device.push(fhir_focal_device);
        }

        // ------------------------------------------------------------------
        // 28. Used  ---------------------------------------------------------
        for (i, used) in src.used.iter().enumerate() {
            let mut used_ref = CodeableReference {
                ..Default::default()
            };

            // Concept
            used_ref.concept = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.used_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://snomed.info/sct".to_string(),
                            ..Default::default()
                        })),
                    code: src.used_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.used_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: used.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });

            // Reference
            if let Some(used_reference_id) = src.used_reference_id.get(i) {
                let used_reference_type = src.used_reference_type.get(i)
                    .cloned()
                    .unwrap_or_else(|| "Device".to_string());

                used_ref.reference = Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", used_reference_type, used_reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.used.push(used_ref);
        }

        // ------------------------------------------------------------------
        // 29. Supporting Info  ----------------------------------------------
        for (i, supporting_info_id) in src.supporting_info_ids.iter().enumerate() {
            let supporting_info_type = src.supporting_info_types.get(i)
                .cloned()
                .unwrap_or_else(|| "Resource".to_string());

            dest.supporting_info.push(Reference {
                reference: Some(String {
                    value: format!("{}/{}", supporting_info_type, supporting_info_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        dest
    }
}

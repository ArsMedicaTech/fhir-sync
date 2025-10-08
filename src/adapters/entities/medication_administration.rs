use chrono::{DateTime, Utc};
use crate::domain::medication_administration::DomainMedicationAdministration;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: MedicationAdministration struct may not be generated yet
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
};

// Placeholder for MedicationAdministration until it's generated
// This will be replaced with the actual struct when available
pub struct MedicationAdministration {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub status: Option<StatusCode>,
    pub status_reason: Vec<CodeableConcept>,
    pub category: Vec<CodeableConcept>,
    pub medication: Option<CodeableReference>,
    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub occurence: Option<OccurenceX>,
    pub recorded: Option<FhirDateTime>,
    pub is_sub_potent: Option<Boolean>,
    pub sub_potent_reason: Vec<CodeableConcept>,
    pub performer: Vec<Performer>,
    pub reason: Vec<CodeableReference>,
    pub request: Option<Reference>,
    pub device: Vec<CodeableReference>,
    pub note: Vec<Annotation>,
    pub dosage: Option<Dosage>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct OccurenceX {
    pub choice: Option<OccurenceChoice>,
}

pub enum OccurenceChoice {
    DateTime(FhirDateTime),
    Period(Period),
    Timing(Timing),
}

pub struct Timing {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub event: Vec<FhirDateTime>,
    pub repeat: Option<TimingRepeat>,
    pub code: Option<CodeableConcept>,
}

pub struct TimingRepeat {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub bounds_duration: Option<Duration>,
    pub bounds_range: Option<Range>,
    pub bounds_period: Option<Period>,
    pub count: Option<UnsignedInt>,
    pub count_max: Option<UnsignedInt>,
    pub duration: Option<Decimal>,
    pub duration_max: Option<Decimal>,
    pub duration_unit: Option<String>,
    pub frequency: Option<UnsignedInt>,
    pub frequency_max: Option<UnsignedInt>,
    pub period: Option<Decimal>,
    pub period_max: Option<Decimal>,
    pub period_unit: Option<String>,
    pub day_of_week: Vec<String>,
    pub time_of_day: Vec<Time>,
    pub when: Vec<String>,
    pub offset: Option<UnsignedInt>,
}

pub struct Duration {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub value: Option<Decimal>,
    pub unit: Option<String>,
    pub system: Option<Uri>,
    pub code: Option<Code>,
}

pub struct Range {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub low: Option<SimpleQuantity>,
    pub high: Option<SimpleQuantity>,
}

pub struct SimpleQuantity {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub value: Option<Decimal>,
    pub unit: Option<String>,
    pub system: Option<Uri>,
    pub code: Option<Code>,
}

pub struct Decimal {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub value: Option<f64>,
}

pub struct UnsignedInt {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub value: Option<u32>,
}

pub struct Time {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub value: Option<String>,
}

pub struct Performer {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub function: Option<CodeableConcept>,
    pub actor: Option<CodeableReference>,
}

pub struct Dosage {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub text: Option<String>,
    pub site: Option<CodeableConcept>,
    pub route: Option<CodeableConcept>,
    pub method: Option<CodeableConcept>,
    pub dose: Option<SimpleQuantity>,
    pub rate: Option<RateX>,
}

pub struct RateX {
    pub choice: Option<RateChoice>,
}

pub enum RateChoice {
    Ratio(Ratio),
    SimpleQuantity(SimpleQuantity),
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

impl Default for MedicationAdministration {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            status: None,
            status_reason: Vec::new(),
            category: Vec::new(),
            medication: None,
            subject: None,
            encounter: None,
            occurence: None,
            recorded: None,
            is_sub_potent: None,
            sub_potent_reason: Vec::new(),
            performer: Vec::new(),
            reason: Vec::new(),
            request: None,
            device: Vec::new(),
            note: Vec::new(),
            dosage: None,
        }
    }
}

impl From<DomainMedicationAdministration> for MedicationAdministration {
    fn from(src: DomainMedicationAdministration) -> Self {
        // Start with a completely empty message
        let mut dest = MedicationAdministration::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.medication_administration_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:medication_administration_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.medication_administration_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        let status_value = match src.status.to_lowercase().as_str() {
            "in-progress" => 1,
            "not-done" => 2,
            "on-hold" => 3,
            "completed" => 4,
            "entered-in-error" => 5,
            "stopped" => 6,
            "unknown" => 7,
            _ => 0,
        };

        dest.status = Some(StatusCode {
            value: status_value,
            id: None,
            extension: Vec::new(),
        });

        // ------------------------------------------------------------------
        // 4. Status Reason  -------------------------------------------------
        if let Some(status_reasons) = src.status_reason {
            for (i, reason) in status_reasons.iter().enumerate() {
                dest.status_reason.push(CodeableConcept {
                    coding: vec![Coding {
                        system: src.status_reason_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/medication-admin-status-reason".to_string(),
                                ..Default::default()
                            })),
                        code: src.status_reason_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.status_reason_displays.as_ref()
                            .and_then(|displays| displays.get(i))
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
            }
        }

        // ------------------------------------------------------------------
        // 5. Category  ------------------------------------------------------
        if let Some(categories) = src.category {
            for (i, category) in categories.iter().enumerate() {
                dest.category.push(CodeableConcept {
                    coding: vec![Coding {
                        system: src.category_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/medication-admin-category".to_string(),
                                ..Default::default()
                            })),
                        code: src.category_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.category_displays.as_ref()
                            .and_then(|displays| displays.get(i))
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
        }

        // ------------------------------------------------------------------
        // 6. Medication  ----------------------------------------------------
        if src.medication_code.is_some() || src.medication_reference_id.is_some() {
            let mut medication = CodeableReference {
                ..Default::default()
            };

            // Concept
            if let Some(medication_code) = src.medication_code {
                medication.concept = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.medication_code_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://www.nlm.nih.gov/research/umls/rxnorm".to_string(),
                            ..Default::default()
                        })),
                        code: Some(Code {
                            value: medication_code,
                            ..Default::default()
                        }),
                        display: src.medication_code_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: medication_code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Reference
            if let Some(medication_reference_id) = src.medication_reference_id {
                let medication_reference_type = src.medication_reference_type
                    .unwrap_or_else(|| "Medication".to_string());

                medication.reference = Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", medication_reference_type, medication_reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.medication = Some(medication);
        }

        // ------------------------------------------------------------------
        // 7. Subject  -------------------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("{}/{}", src.subject_type, src.subject_id),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 8. Encounter  -----------------------------------------------------
        if let Some(encounter_id) = src.encounter_id {
            let encounter_type = src.encounter_type
                .unwrap_or_else(|| "Encounter".to_string());

            dest.encounter = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", encounter_type, encounter_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 9. Occurrence  ----------------------------------------------------
        if src.occurrence_date_time.is_some() || src.occurrence_period_start.is_some() {
            let mut occurence_x = OccurenceX {
                choice: None,
            };

            // Try date time first
            if let Some(occurrence_date_time) = src.occurrence_date_time {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&occurrence_date_time) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    occurence_x.choice = Some(OccurenceChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    }));
                }
            }
            // Try period
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

                occurence_x.choice = Some(OccurenceChoice::Period(period));
            }

            dest.occurence = Some(occurence_x);
        }

        // ------------------------------------------------------------------
        // 10. Recorded  -----------------------------------------------------
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
        // 11. Is Sub Potent  ------------------------------------------------
        if let Some(is_sub_potent) = src.is_sub_potent {
            dest.is_sub_potent = Some(Boolean {
                value: is_sub_potent,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 12. Sub Potent Reason  --------------------------------------------
        if let Some(sub_potent_reasons) = src.sub_potent_reason {
            for (i, reason) in sub_potent_reasons.iter().enumerate() {
                dest.sub_potent_reason.push(CodeableConcept {
                    coding: vec![Coding {
                        system: src.sub_potent_reason_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/medication-admin-sub-potent-reason".to_string(),
                                ..Default::default()
                            })),
                        code: src.sub_potent_reason_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.sub_potent_reason_displays.as_ref()
                            .and_then(|displays| displays.get(i))
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
            }
        }

        // ------------------------------------------------------------------
        // 13. Performer  ----------------------------------------------------
        if let Some(performer_functions) = src.performer_function {
            for (i, function) in performer_functions.iter().enumerate() {
                let mut performer = Performer {
                    ..Default::default()
                };

                // Function
                performer.function = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.performer_function_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/medication-admin-performer-function".to_string(),
                                ..Default::default()
                            })),
                        code: src.performer_function_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.performer_function_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: function.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Actor
                if let Some(performer_actor_ids) = &src.performer_actor_id {
                    if let Some(actor_id) = performer_actor_ids.get(i) {
                        let mut actor = CodeableReference {
                            ..Default::default()
                        };

                        // Reference
                        let actor_type = src.performer_actor_type.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Practitioner".to_string());

                        actor.reference = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", actor_type, actor_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });

                        // Concept
                        if let Some(actor_codes) = &src.performer_actor_code {
                            if let Some(actor_code) = actor_codes.get(i) {
                                actor.concept = Some(CodeableConcept {
                                    coding: vec![Coding {
                                        system: src.performer_actor_system.as_ref()
                                            .and_then(|systems| systems.get(i))
                                            .map(|system| Uri {
                                                value: system.clone(),
                                                ..Default::default()
                                            })
                                            .or_else(|| Some(Uri {
                                                value: "http://terminology.hl7.org/CodeSystem/v2-0443".to_string(),
                                                ..Default::default()
                                            })),
                                        code: Some(Code {
                                            value: actor_code.clone(),
                                            ..Default::default()
                                        }),
                                        display: src.performer_actor_display.as_ref()
                                            .and_then(|displays| displays.get(i))
                                            .map(|display| String {
                                                value: display.clone(),
                                                ..Default::default()
                                            }),
                                        ..Default::default()
                                    }],
                                    text: Some(String {
                                        value: actor_code.clone(),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                });
                            }
                        }

                        performer.actor = Some(actor);
                    }
                }

                dest.performer.push(performer);
            }
        }

        // ------------------------------------------------------------------
        // 14. Reason  -------------------------------------------------------
        if let Some(reason_codes) = src.reason_code {
            for (i, reason_code) in reason_codes.iter().enumerate() {
                let mut reason = CodeableReference {
                    ..Default::default()
                };

                // Concept
                reason.concept = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.reason_code_system.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/condition-code".to_string(),
                                ..Default::default()
                            })),
                        code: Some(Code {
                            value: reason_code.clone(),
                            ..Default::default()
                        }),
                        display: src.reason_code_display.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: reason_code.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Reference
                if let Some(reason_reference_ids) = &src.reason_reference_id {
                    if let Some(reference_id) = reason_reference_ids.get(i) {
                        let reference_type = src.reason_reference_type.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Condition".to_string());

                        reason.reference = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", reference_type, reference_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.reason.push(reason);
            }
        }

        // ------------------------------------------------------------------
        // 15. Request  ------------------------------------------------------
        if let Some(request_id) = src.request_id {
            let request_type = src.request_type
                .unwrap_or_else(|| "MedicationRequest".to_string());

            dest.request = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", request_type, request_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 16. Device  -------------------------------------------------------
        if let Some(device_codes) = src.device_code {
            for (i, device_code) in device_codes.iter().enumerate() {
                let mut device = CodeableReference {
                    ..Default::default()
                };

                // Concept
                device.concept = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.device_code_system.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/device-type".to_string(),
                                ..Default::default()
                            })),
                        code: Some(Code {
                            value: device_code.clone(),
                            ..Default::default()
                        }),
                        display: src.device_code_display.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: device_code.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Reference
                if let Some(device_reference_ids) = &src.device_reference_id {
                    if let Some(reference_id) = device_reference_ids.get(i) {
                        let reference_type = src.device_reference_type.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Device".to_string());

                        device.reference = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", reference_type, reference_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.device.push(device);
            }
        }

        // ------------------------------------------------------------------
        // 17. Note  ---------------------------------------------------------
        if let Some(notes) = src.note {
            for note_text in notes {
                dest.note.push(Annotation {
                    text: Some(String {
                        value: note_text,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 18. Dosage  -------------------------------------------------------
        if src.dosage_text.is_some() || src.dosage_site.is_some() || src.dosage_route.is_some() || 
           src.dosage_method.is_some() || src.dosage_dose_value.is_some() || 
           src.dosage_rate_ratio_numerator_value.is_some() || src.dosage_rate_quantity_value.is_some() {
            
            let mut dosage = Dosage {
                ..Default::default()
            };

            // Text
            if let Some(dosage_text) = src.dosage_text {
                dosage.text = Some(String {
                    value: dosage_text,
                    ..Default::default()
                });
            }

            // Site
            if let Some(dosage_site) = src.dosage_site {
                dosage.site = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.dosage_site_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/body-site".to_string(),
                            ..Default::default()
                        })),
                        code: src.dosage_site_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: src.dosage_site_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: dosage_site,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Route
            if let Some(dosage_route) = src.dosage_route {
                dosage.route = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.dosage_route_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/route-codes".to_string(),
                            ..Default::default()
                        })),
                        code: src.dosage_route_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: src.dosage_route_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: dosage_route,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Method
            if let Some(dosage_method) = src.dosage_method {
                dosage.method = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.dosage_method_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/medication-admin-method".to_string(),
                            ..Default::default()
                        })),
                        code: src.dosage_method_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: src.dosage_method_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: dosage_method,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Dose
            if let Some(dosage_dose_value) = src.dosage_dose_value {
                dosage.dose = Some(SimpleQuantity {
                    value: Some(dosage_dose_value),
                    unit: src.dosage_dose_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: src.dosage_dose_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: src.dosage_dose_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Rate
            if src.dosage_rate_ratio_numerator_value.is_some() || src.dosage_rate_quantity_value.is_some() {
                let mut rate_x = RateX {
                    choice: None,
                };

                // Try ratio first
                if let Some(numerator_value) = src.dosage_rate_ratio_numerator_value {
                    let mut ratio = Ratio {
                        ..Default::default()
                    };

                    // Numerator
                    ratio.numerator = Some(SimpleQuantity {
                        value: Some(numerator_value),
                        unit: src.dosage_rate_ratio_numerator_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: src.dosage_rate_ratio_numerator_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: src.dosage_rate_ratio_numerator_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });

                    // Denominator
                    if let Some(denominator_value) = src.dosage_rate_ratio_denominator_value {
                        ratio.denominator = Some(SimpleQuantity {
                            value: Some(denominator_value),
                            unit: src.dosage_rate_ratio_denominator_unit.map(|unit| String {
                                value: unit,
                                ..Default::default()
                            }),
                            system: src.dosage_rate_ratio_denominator_system.map(|system| Uri {
                                value: system,
                                ..Default::default()
                            }).or_else(|| Some(Uri {
                                value: "http://unitsofmeasure.org".to_string(),
                                ..Default::default()
                            })),
                            code: src.dosage_rate_ratio_denominator_code.map(|code| Code {
                                value: code,
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }

                    rate_x.choice = Some(RateChoice::Ratio(ratio));
                }
                // Try quantity
                else if let Some(quantity_value) = src.dosage_rate_quantity_value {
                    let quantity = SimpleQuantity {
                        value: Some(quantity_value),
                        unit: src.dosage_rate_quantity_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: src.dosage_rate_quantity_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: src.dosage_rate_quantity_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    };

                    rate_x.choice = Some(RateChoice::SimpleQuantity(quantity));
                }

                dosage.rate = Some(rate_x);
            }

            dest.dosage = Some(dosage);
        }

        dest
    }
}

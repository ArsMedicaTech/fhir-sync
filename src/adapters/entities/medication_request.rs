use chrono::{DateTime, Utc};
use crate::domain::medication_request::DomainMedicationRequest;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: MedicationRequest struct may not be generated yet
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
};

// Placeholder for MedicationRequest until it's generated
// This will be replaced with the actual struct when available
pub struct MedicationRequest {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub status: Option<StatusCode>,
    pub status_reason: Option<CodeableConcept>,
    pub status_changed: Option<FhirDateTime>,
    pub intent: Option<IntentCode>,
    pub category: Vec<CodeableConcept>,
    pub priority: Option<PriorityCode>,
    pub do_not_perform: Option<Boolean>,
    pub medication: Option<CodeableReference>,
    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub authored_on: Option<FhirDateTime>,
    pub requester: Option<Reference>,
    pub reported: Option<Boolean>,
    pub performer_type: Option<CodeableConcept>,
    pub performer: Vec<Reference>,
    pub device: Vec<CodeableReference>,
    pub recorder: Option<Reference>,
    pub reason: Vec<CodeableReference>,
    pub course_of_therapy_type: Option<CodeableConcept>,
    pub insurance: Vec<Reference>,
    pub note: Vec<Annotation>,
    pub rendered_dosage_instruction: Option<Markdown>,
    pub effective_dose_period: Option<Period>,
    pub dosage_instruction: Vec<Dosage>,
    pub dispense_request: Option<DispenseRequest>,
    pub substitution: Option<Substitution>,
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

pub struct PriorityCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
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

pub struct DispenseRequest {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub initial_fill: Option<InitialFill>,
    pub dispense_interval: Option<Duration>,
    pub validity_period: Option<Period>,
    pub number_of_repeats_allowed: Option<UnsignedInt>,
    pub quantity: Option<SimpleQuantity>,
    pub expected_supply_duration: Option<Duration>,
    pub dispenser: Option<Reference>,
    pub dispenser_instruction: Vec<Annotation>,
    pub dose_administration_aid: Option<CodeableConcept>,
}

pub struct InitialFill {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub quantity: Option<SimpleQuantity>,
    pub duration: Option<Duration>,
}

pub struct Substitution {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub allowed: Option<AllowedX>,
    pub reason: Option<CodeableConcept>,
}

pub struct AllowedX {
    pub choice: Option<AllowedChoice>,
}

pub enum AllowedChoice {
    Boolean(Boolean),
    CodeableConcept(CodeableConcept),
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

impl Default for MedicationRequest {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            status: None,
            status_reason: None,
            status_changed: None,
            intent: None,
            category: Vec::new(),
            priority: None,
            do_not_perform: None,
            medication: None,
            subject: None,
            encounter: None,
            authored_on: None,
            requester: None,
            reported: None,
            performer_type: None,
            performer: Vec::new(),
            device: Vec::new(),
            recorder: None,
            reason: Vec::new(),
            course_of_therapy_type: None,
            insurance: Vec::new(),
            note: Vec::new(),
            rendered_dosage_instruction: None,
            effective_dose_period: None,
            dosage_instruction: Vec::new(),
            dispense_request: None,
            substitution: None,
        }
    }
}

impl From<DomainMedicationRequest> for MedicationRequest {
    fn from(src: DomainMedicationRequest) -> Self {
        // Start with a completely empty message
        let mut dest = MedicationRequest::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.medication_request_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:medication_request_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.medication_request_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        let status_value = match src.status.to_lowercase().as_str() {
            "active" => 1,
            "on-hold" => 2,
            "ended" => 3,
            "stopped" => 4,
            "completed" => 5,
            "cancelled" => 6,
            "entered-in-error" => 7,
            "draft" => 8,
            "unknown" => 9,
            _ => 0,
        };

        dest.status = Some(StatusCode {
            value: status_value,
            id: None,
            extension: Vec::new(),
        });

        // ------------------------------------------------------------------
        // 4. Status Reason  -------------------------------------------------
        if let Some(status_reason) = src.status_reason {
            dest.status_reason = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.status_reason_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/medicationrequest-status-reason".to_string(),
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
        // 5. Status Changed  ------------------------------------------------
        if let Some(status_changed) = src.status_changed {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&status_changed) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.status_changed = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 6. Intent  --------------------------------------------------------
        let intent_value = match src.intent.to_lowercase().as_str() {
            "proposal" => 1,
            "plan" => 2,
            "order" => 3,
            "original-order" => 4,
            "reflex-order" => 5,
            "filler-order" => 6,
            "instance-order" => 7,
            "option" => 8,
            _ => 0,
        };

        dest.intent = Some(IntentCode {
            value: intent_value,
            id: None,
            extension: Vec::new(),
        });

        // ------------------------------------------------------------------
        // 7. Category  ------------------------------------------------------
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
                                value: "http://terminology.hl7.org/CodeSystem/medicationrequest-category".to_string(),
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
        // 8. Priority  ------------------------------------------------------
        if let Some(priority) = src.priority {
            let priority_value = match priority.to_lowercase().as_str() {
                "routine" => 1,
                "urgent" => 2,
                "asap" => 3,
                "stat" => 4,
                _ => 0,
            };

            dest.priority = Some(PriorityCode {
                value: priority_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 9. Do Not Perform  ------------------------------------------------
        if let Some(do_not_perform) = src.do_not_perform {
            dest.do_not_perform = Some(Boolean {
                value: do_not_perform,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Medication  ---------------------------------------------------
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
        // 11. Subject  ------------------------------------------------------
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("{}/{}", src.subject_type, src.subject_id),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 12. Encounter  ----------------------------------------------------
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
        // 13. Authored On  --------------------------------------------------
        if let Some(authored_on) = src.authored_on {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&authored_on) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.authored_on = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Requester  ----------------------------------------------------
        if let Some(requester_id) = src.requester_id {
            let requester_type = src.requester_type
                .unwrap_or_else(|| "Practitioner".to_string());

            dest.requester = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", requester_type, requester_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 15. Reported  -----------------------------------------------------
        if let Some(reported) = src.reported {
            dest.reported = Some(Boolean {
                value: reported,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 16. Performer Type  -----------------------------------------------
        if let Some(performer_type) = src.performer_type {
            dest.performer_type = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.performer_type_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/medicationrequest-performer-type".to_string(),
                        ..Default::default()
                    })),
                    code: src.performer_type_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.performer_type_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: performer_type,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 17. Performer  ----------------------------------------------------
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
        // 18. Device  -------------------------------------------------------
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
        // 19. Recorder  -----------------------------------------------------
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
        // 20. Reason  -------------------------------------------------------
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
        // 21. Course of Therapy Type  ---------------------------------------
        if let Some(course_of_therapy_type) = src.course_of_therapy_type {
            dest.course_of_therapy_type = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.course_of_therapy_type_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/medicationrequest-course-of-therapy".to_string(),
                        ..Default::default()
                    })),
                    code: src.course_of_therapy_type_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.course_of_therapy_type_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: course_of_therapy_type,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 22. Insurance  ----------------------------------------------------
        if let Some(insurance_ids) = src.insurance_ids {
            for (i, insurance_id) in insurance_ids.iter().enumerate() {
                let insurance_type = src.insurance_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "Coverage".to_string());

                dest.insurance.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", insurance_type, insurance_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 23. Note  ---------------------------------------------------------
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
        // 24. Rendered Dosage Instruction  ----------------------------------
        if let Some(rendered_dosage_instruction) = src.rendered_dosage_instruction {
            dest.rendered_dosage_instruction = Some(Markdown {
                value: rendered_dosage_instruction,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 25. Effective Dose Period  ----------------------------------------
        if src.effective_dose_period_start.is_some() || src.effective_dose_period_end.is_some() {
            let mut period = Period {
                ..Default::default()
            };

            if let Some(period_start) = src.effective_dose_period_start {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.start = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            if let Some(period_end) = src.effective_dose_period_end {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.end = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.effective_dose_period = Some(period);
        }

        // ------------------------------------------------------------------
        // 26. Dosage Instruction  -------------------------------------------
        if let Some(dosage_texts) = src.dosage_text {
            for (i, dosage_text) in dosage_texts.iter().enumerate() {
                let mut dosage = Dosage {
                    ..Default::default()
                };

                // Text
                dosage.text = Some(String {
                    value: dosage_text.clone(),
                    ..Default::default()
                });

                // Site
                if let Some(dosage_sites) = &src.dosage_site {
                    if let Some(dosage_site) = dosage_sites.get(i) {
                        dosage.site = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.dosage_site_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://terminology.hl7.org/CodeSystem/body-site".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.dosage_site_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.dosage_site_display.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: dosage_site.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Route
                if let Some(dosage_routes) = &src.dosage_route {
                    if let Some(dosage_route) = dosage_routes.get(i) {
                        dosage.route = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.dosage_route_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://terminology.hl7.org/CodeSystem/route-codes".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.dosage_route_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.dosage_route_display.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: dosage_route.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Method
                if let Some(dosage_methods) = &src.dosage_method {
                    if let Some(dosage_method) = dosage_methods.get(i) {
                        dosage.method = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.dosage_method_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://terminology.hl7.org/CodeSystem/medication-admin-method".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.dosage_method_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.dosage_method_display.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: dosage_method.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Dose
                if let Some(dosage_dose_values) = &src.dosage_dose_value {
                    if let Some(dosage_dose_value) = dosage_dose_values.get(i) {
                        dosage.dose = Some(SimpleQuantity {
                            value: Some(*dosage_dose_value),
                            unit: src.dosage_dose_unit.as_ref()
                                .and_then(|units| units.get(i))
                                .map(|unit| String {
                                    value: unit.clone(),
                                    ..Default::default()
                                }),
                            system: src.dosage_dose_system.as_ref()
                                .and_then(|systems| systems.get(i))
                                .map(|system| Uri {
                                    value: system.clone(),
                                    ..Default::default()
                                })
                                .or_else(|| Some(Uri {
                                    value: "http://unitsofmeasure.org".to_string(),
                                    ..Default::default()
                                })),
                            code: src.dosage_dose_code.as_ref()
                                .and_then(|codes| codes.get(i))
                                .map(|code| Code {
                                    value: code.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        });
                    }
                }

                // Rate
                if src.dosage_rate_ratio_numerator_value.is_some() || src.dosage_rate_quantity_value.is_some() {
                    let mut rate_x = RateX {
                        choice: None,
                    };

                    // Try ratio first
                    if let Some(numerator_values) = &src.dosage_rate_ratio_numerator_value {
                        if let Some(numerator_value) = numerator_values.get(i) {
                            let mut ratio = Ratio {
                                ..Default::default()
                            };

                            // Numerator
                            ratio.numerator = Some(SimpleQuantity {
                                value: Some(*numerator_value),
                                unit: src.dosage_rate_ratio_numerator_unit.as_ref()
                                    .and_then(|units| units.get(i))
                                    .map(|unit| String {
                                        value: unit.clone(),
                                        ..Default::default()
                                    }),
                                system: src.dosage_rate_ratio_numerator_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://unitsofmeasure.org".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.dosage_rate_ratio_numerator_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            });

                            // Denominator
                            if let Some(denominator_values) = &src.dosage_rate_ratio_denominator_value {
                                if let Some(denominator_value) = denominator_values.get(i) {
                                    ratio.denominator = Some(SimpleQuantity {
                                        value: Some(*denominator_value),
                                        unit: src.dosage_rate_ratio_denominator_unit.as_ref()
                                            .and_then(|units| units.get(i))
                                            .map(|unit| String {
                                                value: unit.clone(),
                                                ..Default::default()
                                            }),
                                        system: src.dosage_rate_ratio_denominator_system.as_ref()
                                            .and_then(|systems| systems.get(i))
                                            .map(|system| Uri {
                                                value: system.clone(),
                                                ..Default::default()
                                            })
                                            .or_else(|| Some(Uri {
                                                value: "http://unitsofmeasure.org".to_string(),
                                                ..Default::default()
                                            })),
                                        code: src.dosage_rate_ratio_denominator_code.as_ref()
                                            .and_then(|codes| codes.get(i))
                                            .map(|code| Code {
                                                value: code.clone(),
                                                ..Default::default()
                                            }),
                                        ..Default::default()
                                    });
                                }
                            }

                            rate_x.choice = Some(RateChoice::Ratio(ratio));
                        }
                    }
                    // Try quantity
                    else if let Some(quantity_values) = &src.dosage_rate_quantity_value {
                        if let Some(quantity_value) = quantity_values.get(i) {
                            let quantity = SimpleQuantity {
                                value: Some(*quantity_value),
                                unit: src.dosage_rate_quantity_unit.as_ref()
                                    .and_then(|units| units.get(i))
                                    .map(|unit| String {
                                        value: unit.clone(),
                                        ..Default::default()
                                    }),
                                system: src.dosage_rate_quantity_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://unitsofmeasure.org".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.dosage_rate_quantity_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            };

                            rate_x.choice = Some(RateChoice::SimpleQuantity(quantity));
                        }
                    }

                    dosage.rate = Some(rate_x);
                }

                dest.dosage_instruction.push(dosage);
            }
        }

        // ------------------------------------------------------------------
        // 27. Dispense Request  ---------------------------------------------
        if src.dispense_initial_fill_quantity_value.is_some() || src.dispense_interval_value.is_some() || 
           src.dispense_validity_period_start.is_some() || src.dispense_number_of_repeats_allowed.is_some() || 
           src.dispense_quantity_value.is_some() || src.dispense_expected_supply_duration_value.is_some() || 
           src.dispense_dispenser_id.is_some() || src.dispense_dose_administration_aid.is_some() {
            
            let mut dispense_request = DispenseRequest {
                ..Default::default()
            };

            // Initial Fill
            if src.dispense_initial_fill_quantity_value.is_some() || src.dispense_initial_fill_duration_value.is_some() {
                let mut initial_fill = InitialFill {
                    ..Default::default()
                };

                // Quantity
                if let Some(quantity_value) = src.dispense_initial_fill_quantity_value {
                    initial_fill.quantity = Some(SimpleQuantity {
                        value: Some(quantity_value),
                        unit: src.dispense_initial_fill_quantity_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: src.dispense_initial_fill_quantity_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: src.dispense_initial_fill_quantity_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                // Duration
                if let Some(duration_value) = src.dispense_initial_fill_duration_value {
                    initial_fill.duration = Some(Duration {
                        value: Some(duration_value),
                        unit: src.dispense_initial_fill_duration_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: src.dispense_initial_fill_duration_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: src.dispense_initial_fill_duration_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                dispense_request.initial_fill = Some(initial_fill);
            }

            // Dispense Interval
            if let Some(interval_value) = src.dispense_interval_value {
                dispense_request.dispense_interval = Some(Duration {
                    value: Some(interval_value),
                    unit: src.dispense_interval_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: src.dispense_interval_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: src.dispense_interval_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Validity Period
            if src.dispense_validity_period_start.is_some() || src.dispense_validity_period_end.is_some() {
                let mut period = Period {
                    ..Default::default()
                };

                if let Some(period_start) = src.dispense_validity_period_start {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.start = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                if let Some(period_end) = src.dispense_validity_period_end {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.end = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                dispense_request.validity_period = Some(period);
            }

            // Number of Repeats Allowed
            if let Some(number_of_repeats_allowed) = src.dispense_number_of_repeats_allowed {
                dispense_request.number_of_repeats_allowed = Some(UnsignedInt {
                    value: Some(number_of_repeats_allowed),
                    ..Default::default()
                });
            }

            // Quantity
            if let Some(quantity_value) = src.dispense_quantity_value {
                dispense_request.quantity = Some(SimpleQuantity {
                    value: Some(quantity_value),
                    unit: src.dispense_quantity_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: src.dispense_quantity_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: src.dispense_quantity_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Expected Supply Duration
            if let Some(duration_value) = src.dispense_expected_supply_duration_value {
                dispense_request.expected_supply_duration = Some(Duration {
                    value: Some(duration_value),
                    unit: src.dispense_expected_supply_duration_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: src.dispense_expected_supply_duration_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: src.dispense_expected_supply_duration_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Dispenser
            if let Some(dispenser_id) = src.dispense_dispenser_id {
                let dispenser_type = src.dispense_dispenser_type
                    .unwrap_or_else(|| "Organization".to_string());

                dispense_request.dispenser = Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", dispenser_type, dispenser_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Dispenser Instruction
            if let Some(dispenser_instructions) = src.dispense_dispenser_instruction {
                for instruction_text in dispenser_instructions {
                    dispense_request.dispenser_instruction.push(Annotation {
                        text: Some(String {
                            value: instruction_text,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }
            }

            // Dose Administration Aid
            if let Some(dose_administration_aid) = src.dispense_dose_administration_aid {
                dispense_request.dose_administration_aid = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.dispense_dose_administration_aid_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/medicationrequest-dose-administration-aid".to_string(),
                            ..Default::default()
                        })),
                        code: src.dispense_dose_administration_aid_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: src.dispense_dose_administration_aid_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: dose_administration_aid,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.dispense_request = Some(dispense_request);
        }

        // ------------------------------------------------------------------
        // 28. Substitution  -------------------------------------------------
        if src.substitution_allowed.is_some() || src.substitution_reason.is_some() {
            let mut substitution = Substitution {
                ..Default::default()
            };

            // Allowed
            if let Some(substitution_allowed) = src.substitution_allowed {
                substitution.allowed = Some(AllowedX {
                    choice: Some(AllowedChoice::Boolean(Boolean {
                        value: substitution_allowed,
                        ..Default::default()
                    })),
                });
            }

            // Reason
            if let Some(substitution_reason) = src.substitution_reason {
                substitution.reason = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.substitution_reason_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/medicationrequest-substitution-reason".to_string(),
                            ..Default::default()
                        })),
                        code: src.substitution_reason_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: src.substitution_reason_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: substitution_reason,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.substitution = Some(substitution);
        }

        dest
    }
}

use chrono::{DateTime, Utc};
use crate::domain::immunization::DomainImmunization;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Immunization struct may not be generated yet
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
    Date,
    Annotation,
    SimpleQuantity,
    Boolean,
    CodeableReference,
    Canonical,
};

// Placeholder for Immunization until it's generated
// This will be replaced with the actual struct when available
pub struct Immunization {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub based_on: Vec<Reference>,
    pub status: Option<StatusCode>,
    pub status_reason: Option<CodeableConcept>,
    pub vaccine_code: Option<CodeableConcept>,
    pub administered_product: Option<CodeableReference>,
    pub manufacturer: Option<CodeableReference>,
    pub lot_number: Option<String>,
    pub expiration_date: Option<Date>,
    pub patient: Option<Reference>,
    pub encounter: Option<Reference>,
    pub supporting_information: Vec<Reference>,
    pub occurrence: Option<OccurrenceX>,
    pub primary_source: Option<Boolean>,
    pub information_source: Option<CodeableReference>,
    pub location: Option<Reference>,
    pub site: Option<CodeableConcept>,
    pub route: Option<CodeableConcept>,
    pub dose_quantity: Option<SimpleQuantity>,
    pub performer: Vec<Performer>,
    pub note: Vec<Annotation>,
    pub reason: Vec<CodeableReference>,
    pub is_subpotent: Option<Boolean>,
    pub subpotent_reason: Vec<CodeableConcept>,
    pub program_eligibility: Vec<ProgramEligibility>,
    pub funding_source: Option<CodeableConcept>,
    pub reaction: Vec<Reaction>,
    pub protocol_applied: Vec<ProtocolApplied>,
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
    String(String),
}

pub struct Performer {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub function: Option<CodeableConcept>,
    pub actor: Option<Reference>,
}

pub struct ProgramEligibility {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub program: Option<CodeableConcept>,
    pub program_status: Option<CodeableConcept>,
}

pub struct Reaction {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub date: Option<FhirDateTime>,
    pub manifestation: Option<CodeableReference>,
    pub reported: Option<Boolean>,
}

pub struct ProtocolApplied {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub series: Option<String>,
    pub authority: Option<Reference>,
    pub target_disease: Vec<CodeableConcept>,
    pub dose_number: Option<String>,
    pub series_doses: Option<String>,
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

impl Default for Immunization {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            based_on: Vec::new(),
            status: None,
            status_reason: None,
            vaccine_code: None,
            administered_product: None,
            manufacturer: None,
            lot_number: None,
            expiration_date: None,
            patient: None,
            encounter: None,
            supporting_information: Vec::new(),
            occurrence: None,
            primary_source: None,
            information_source: None,
            location: None,
            site: None,
            route: None,
            dose_quantity: None,
            performer: Vec::new(),
            note: Vec::new(),
            reason: Vec::new(),
            is_subpotent: None,
            subpotent_reason: Vec::new(),
            program_eligibility: Vec::new(),
            funding_source: None,
            reaction: Vec::new(),
            protocol_applied: Vec::new(),
        }
    }
}

impl From<DomainImmunization> for Immunization {
    fn from(src: DomainImmunization) -> Self {
        // Start with a completely empty message
        let mut dest = Immunization::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.immunization_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:immunization_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.immunization_id.clone(),
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
                "completed" => 1,
                "entered-in-error" => 2,
                "not-done" => 3,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 5. Status Reason  -------------------------------------------------
        if let Some(status_reason) = src.status_reason {
            dest.status_reason = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.status_reason_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string(),
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
        // 6. Vaccine Code  --------------------------------------------------
        if let Some(vaccine_code) = src.vaccine_code {
            dest.vaccine_code = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.vaccine_code_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://hl7.org/fhir/sid/cvx".to_string(),
                        ..Default::default()
                    })),
                    code: src.vaccine_code_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.vaccine_code_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: vaccine_code,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Administered Product  ------------------------------------------
        if let Some(administered_product) = src.administered_product {
            let mut codeable_reference = CodeableReference {
                ..Default::default()
            };

            // Concept
            codeable_reference.concept = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.administered_product_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://www.fda.gov/".to_string(),
                        ..Default::default()
                    })),
                    code: src.administered_product_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.administered_product_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: administered_product,
                    ..Default::default()
                }),
                ..Default::default()
            });

            dest.administered_product = Some(codeable_reference);
        }

        // ------------------------------------------------------------------
        // 8. Manufacturer  --------------------------------------------------
        if let Some(manufacturer) = src.manufacturer {
            let mut codeable_reference = CodeableReference {
                ..Default::default()
            };

            // Concept
            codeable_reference.concept = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.manufacturer_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://hl7.org/fhir/sid/mvx".to_string(),
                        ..Default::default()
                    })),
                    code: src.manufacturer_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.manufacturer_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: manufacturer,
                    ..Default::default()
                }),
                ..Default::default()
            });

            dest.manufacturer = Some(codeable_reference);
        }

        // ------------------------------------------------------------------
        // 9. Lot Number  ----------------------------------------------------
        if let Some(lot_number) = src.lot_number {
            dest.lot_number = Some(String {
                value: lot_number,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Expiration Date  ----------------------------------------------
        if let Some(expiration_date) = src.expiration_date {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&expiration_date, "%Y-%m-%d") {
                dest.expiration_date = Some(Date {
                    value: date.format("%Y-%m-%d").to_string(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 11. Patient  ------------------------------------------------------
        dest.patient = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 12. Encounter  ----------------------------------------------------
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
        // 13. Supporting Information  ---------------------------------------
        if let Some(supporting_information_ids) = src.supporting_information_ids {
            for info_id in supporting_information_ids {
                dest.supporting_information.push(Reference {
                    reference: Some(String {
                        value: format!("Resource/{}", info_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 14. Occurrence  ---------------------------------------------------
        if let Some(occurrence_date) = src.occurrence_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&occurrence_date) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.occurrence = Some(OccurrenceX {
                    choice: Some(OccurrenceChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    })),
                });
            }
        } else if let Some(occurrence_string) = src.occurrence_string {
            dest.occurrence = Some(OccurrenceX {
                choice: Some(OccurrenceChoice::String(String {
                    value: occurrence_string,
                    ..Default::default()
                })),
            });
        }

        // ------------------------------------------------------------------
        // 15. Primary Source  -----------------------------------------------
        if let Some(primary_source) = src.primary_source {
            dest.primary_source = Some(Boolean {
                value: primary_source,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 16. Information Source  -------------------------------------------
        if let Some(information_source) = src.information_source {
            let mut codeable_reference = CodeableReference {
                ..Default::default()
            };

            // Concept
            codeable_reference.concept = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.information_source_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/information-source".to_string(),
                        ..Default::default()
                    })),
                    code: src.information_source_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.information_source_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: information_source,
                    ..Default::default()
                }),
                ..Default::default()
            });

            dest.information_source = Some(codeable_reference);
        }

        // ------------------------------------------------------------------
        // 17. Location  -----------------------------------------------------
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
        // 18. Site  ---------------------------------------------------------
        if let Some(site) = src.site {
            dest.site = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.site_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-ActSite".to_string(),
                        ..Default::default()
                    })),
                    code: src.site_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.site_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: site,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 19. Route  --------------------------------------------------------
        if let Some(route) = src.route {
            dest.route = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.route_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration".to_string(),
                        ..Default::default()
                    })),
                    code: src.route_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.route_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: route,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 20. Dose Quantity  ------------------------------------------------
        if let Some(dose_quantity_value) = src.dose_quantity_value {
            dest.dose_quantity = Some(SimpleQuantity {
                value: dose_quantity_value,
                unit: src.dose_quantity_unit.map(|unit| String {
                    value: unit,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 21. Performer  ----------------------------------------------------
        if let Some(performer_function_codes) = src.performer_function_codes {
            for (i, function_code) in performer_function_codes.iter().enumerate() {
                let mut performer = Performer {
                    ..Default::default()
                };

                // Function
                performer.function = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.performer_function_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string(),
                                ..Default::default()
                            })),
                        code: src.performer_function_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.performer_function_code_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: function_code.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Actor
                if let Some(performer_actor_ids) = &src.performer_actor_ids {
                    if let Some(actor_id) = performer_actor_ids.get(i) {
                        let actor_type = src.performer_actor_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Practitioner".to_string());

                        performer.actor = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", actor_type, actor_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.performer.push(performer);
            }
        }

        // ------------------------------------------------------------------
        // 22. Reason  -------------------------------------------------------
        if let Some(reason_codes) = src.reason_codes {
            for (i, reason_code) in reason_codes.iter().enumerate() {
                let mut codeable_reference = CodeableReference {
                    ..Default::default()
                };

                // Concept
                codeable_reference.concept = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.reason_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string(),
                                ..Default::default()
                            })),
                        code: src.reason_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.reason_code_displays.as_ref()
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
                if let Some(reason_reference_ids) = &src.reason_reference_ids {
                    if let Some(reference_id) = reason_reference_ids.get(i) {
                        let reference_type = src.reason_reference_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Observation".to_string());

                        codeable_reference.reference = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", reference_type, reference_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.reason.push(codeable_reference);
            }
        }

        // ------------------------------------------------------------------
        // 23. Is Subpotent  -------------------------------------------------
        if let Some(is_subpotent) = src.is_subpotent {
            dest.is_subpotent = Some(Boolean {
                value: is_subpotent,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 24. Subpotent Reason  ---------------------------------------------
        if let Some(subpotent_reason_codes) = src.subpotent_reason_codes {
            for (i, reason_code) in subpotent_reason_codes.iter().enumerate() {
                dest.subpotent_reason.push(CodeableConcept {
                    coding: vec![Coding {
                        system: src.subpotent_reason_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string(),
                                ..Default::default()
                            })),
                        code: src.subpotent_reason_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.subpotent_reason_code_displays.as_ref()
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
            }
        }

        // ------------------------------------------------------------------
        // 25. Program Eligibility  ------------------------------------------
        if let Some(program_eligibility_programs) = src.program_eligibility_programs {
            for (i, program) in program_eligibility_programs.iter().enumerate() {
                let mut program_eligibility = ProgramEligibility {
                    ..Default::default()
                };

                // Program
                program_eligibility.program = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.program_eligibility_program_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/vaccination-program".to_string(),
                                ..Default::default()
                            })),
                        code: src.program_eligibility_program_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.program_eligibility_program_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: program.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Program Status
                if let Some(program_eligibility_statuses) = &src.program_eligibility_statuses {
                    if let Some(status) = program_eligibility_statuses.get(i) {
                        program_eligibility.program_status = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.program_eligibility_status_systems.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://terminology.hl7.org/CodeSystem/vaccination-program-status".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.program_eligibility_status_codes.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.program_eligibility_status_displays.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: status.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.program_eligibility.push(program_eligibility);
            }
        }

        // ------------------------------------------------------------------
        // 26. Funding Source  -----------------------------------------------
        if let Some(funding_source) = src.funding_source {
            dest.funding_source = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.funding_source_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/vaccination-funding-source".to_string(),
                        ..Default::default()
                    })),
                    code: src.funding_source_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.funding_source_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: funding_source,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 27. Reaction  -----------------------------------------------------
        if let Some(reaction_dates) = src.reaction_dates {
            for (i, reaction_date) in reaction_dates.iter().enumerate() {
                let mut reaction = Reaction {
                    ..Default::default()
                };

                // Date
                if let Ok(dt) = DateTime::parse_from_rfc3339(reaction_date) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    reaction.date = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }

                // Manifestation
                if let Some(manifestations) = &src.reaction_manifestations {
                    if let Some(manifestation) = manifestations.get(i) {
                        let mut codeable_reference = CodeableReference {
                            ..Default::default()
                        };

                        // Concept
                        codeable_reference.concept = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.reaction_manifestation_systems.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://snomed.info/sct".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.reaction_manifestation_codes.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.reaction_manifestation_displays.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: manifestation.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });

                        // Reference
                        if let Some(reaction_reference_ids) = &src.reaction_reference_ids {
                            if let Some(reference_id) = reaction_reference_ids.get(i) {
                                let reference_type = src.reaction_reference_types.as_ref()
                                    .and_then(|types| types.get(i))
                                    .cloned()
                                    .unwrap_or_else(|| "Observation".to_string());

                                codeable_reference.reference = Some(Reference {
                                    reference: Some(String {
                                        value: format!("{}/{}", reference_type, reference_id),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                });
                            }
                        }

                        reaction.manifestation = Some(codeable_reference);
                    }
                }

                // Reported
                if let Some(reaction_reported) = &src.reaction_reported {
                    if let Some(reported) = reaction_reported.get(i) {
                        reaction.reported = Some(Boolean {
                            value: *reported,
                            ..Default::default()
                        });
                    }
                }

                dest.reaction.push(reaction);
            }
        }

        // ------------------------------------------------------------------
        // 28. Protocol Applied  ---------------------------------------------
        if let Some(protocol_series) = src.protocol_series {
            for (i, series) in protocol_series.iter().enumerate() {
                let mut protocol_applied = ProtocolApplied {
                    ..Default::default()
                };

                // Series
                protocol_applied.series = Some(String {
                    value: series.clone(),
                    ..Default::default()
                });

                // Authority
                if let Some(authority_ids) = &src.protocol_authority_ids {
                    if let Some(authority_id) = authority_ids.get(i) {
                        protocol_applied.authority = Some(Reference {
                            reference: Some(String {
                                value: format!("Organization/{}", authority_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Target Disease
                if let Some(target_diseases) = &src.protocol_target_diseases {
                    if let Some(target_disease) = target_diseases.get(i) {
                        protocol_applied.target_disease.push(CodeableConcept {
                            coding: vec![Coding {
                                system: src.protocol_target_disease_systems.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://snomed.info/sct".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.protocol_target_disease_codes.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.protocol_target_disease_displays.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: target_disease.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Dose Number
                if let Some(dose_numbers) = &src.protocol_dose_numbers {
                    if let Some(dose_number) = dose_numbers.get(i) {
                        protocol_applied.dose_number = Some(String {
                            value: dose_number.clone(),
                            ..Default::default()
                        });
                    }
                }

                // Series Doses
                if let Some(series_doses) = &src.protocol_series_doses {
                    if let Some(series_dose) = series_doses.get(i) {
                        protocol_applied.series_doses = Some(String {
                            value: series_dose.clone(),
                            ..Default::default()
                        });
                    }
                }

                dest.protocol_applied.push(protocol_applied);
            }
        }

        // ------------------------------------------------------------------
        // 29. Notes  ---------------------------------------------------------
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

use chrono::{DateTime, Utc};
use crate::domain::medication::DomainMedication;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Medication struct may not be generated yet
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
};

// Placeholder for Medication until it's generated
// This will be replaced with the actual struct when available
pub struct Medication {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub code: Option<CodeableConcept>,
    pub status: Option<StatusCode>,
    pub marketing_authorization_holder: Option<Reference>,
    pub dose_form: Option<CodeableConcept>,
    pub total_volume: Option<Quantity>,
    pub ingredient: Vec<Ingredient>,
    pub batch: Option<Batch>,
    pub definition: Option<Reference>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct Ingredient {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub item: Option<CodeableReference>,
    pub is_active: Option<Boolean>,
    pub strength: Option<StrengthX>,
}

pub struct StrengthX {
    pub choice: Option<StrengthChoice>,
}

pub enum StrengthChoice {
    Ratio(Ratio),
    CodeableConcept(CodeableConcept),
    Quantity(Quantity),
}

pub struct Batch {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub lot_number: Option<String>,
    pub expiration_date: Option<FhirDateTime>,
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

impl Default for Medication {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            code: None,
            status: None,
            marketing_authorization_holder: None,
            dose_form: None,
            total_volume: None,
            ingredient: Vec::new(),
            batch: None,
            definition: None,
        }
    }
}

impl From<DomainMedication> for Medication {
    fn from(src: DomainMedication) -> Self {
        // Start with a completely empty message
        let mut dest = Medication::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.medication_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:medication_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.medication_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Code  ----------------------------------------------------------
        if let Some(code) = src.code {
            dest.code = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.code_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://www.nlm.nih.gov/research/umls/rxnorm".to_string(),
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
        // 4. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "active" => 1,
                "inactive" => 2,
                "entered-in-error" => 3,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 5. Marketing Authorization Holder  --------------------------------
        if let Some(marketing_authorization_holder_id) = src.marketing_authorization_holder_id {
            let marketing_authorization_holder_type = src.marketing_authorization_holder_type
                .unwrap_or_else(|| "Organization".to_string());

            dest.marketing_authorization_holder = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", marketing_authorization_holder_type, marketing_authorization_holder_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Dose Form  -----------------------------------------------------
        if let Some(dose_form) = src.dose_form {
            dest.dose_form = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.dose_form_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm".to_string(),
                        ..Default::default()
                    })),
                    code: src.dose_form_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.dose_form_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: dose_form,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Total Volume  --------------------------------------------------
        if let Some(total_volume_value) = src.total_volume_value {
            dest.total_volume = Some(Quantity {
                value: total_volume_value,
                unit: src.total_volume_unit.map(|unit| String {
                    value: unit,
                    ..Default::default()
                }),
                system: src.total_volume_system.map(|system| Uri {
                    value: system,
                    ..Default::default()
                }).or_else(|| Some(Uri {
                    value: "http://unitsofmeasure.org".to_string(),
                    ..Default::default()
                })),
                code: src.total_volume_code.map(|code| Code {
                    value: code,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 8. Ingredients  ---------------------------------------------------
        if let Some(ingredient_item_codes) = src.ingredient_item_codes {
            for (i, item_code) in ingredient_item_codes.iter().enumerate() {
                let mut ingredient = Ingredient {
                    ..Default::default()
                };

                // Item
                ingredient.item = Some(CodeableReference {
                    concept: Some(CodeableConcept {
                        coding: vec![Coding {
                            system: src.ingredient_item_systems.as_ref()
                                .and_then(|systems| systems.get(i))
                                .map(|system| Uri {
                                    value: system.clone(),
                                    ..Default::default()
                                })
                                .or_else(|| Some(Uri {
                                    value: "http://www.nlm.nih.gov/research/umls/rxnorm".to_string(),
                                    ..Default::default()
                                })),
                            code: Some(Code {
                                value: item_code.clone(),
                                ..Default::default()
                            }),
                            display: src.ingredient_item_displays.as_ref()
                                .and_then(|displays| displays.get(i))
                                .map(|display| String {
                                    value: display.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        }],
                        text: Some(String {
                            value: item_code.clone(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    reference: src.ingredient_item_reference_ids.as_ref()
                        .and_then(|ids| ids.get(i))
                        .and_then(|id| {
                            let reference_type = src.ingredient_item_reference_types.as_ref()
                                .and_then(|types| types.get(i))
                                .cloned()
                                .unwrap_or_else(|| "Substance".to_string());
                            
                            Some(Reference {
                                reference: Some(String {
                                    value: format!("{}/{}", reference_type, id),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            })
                        }),
                    ..Default::default()
                });

                // Is Active
                if let Some(ingredient_is_active) = &src.ingredient_is_active {
                    if let Some(is_active) = ingredient_is_active.get(i) {
                        ingredient.is_active = Some(Boolean {
                            value: *is_active,
                            ..Default::default()
                        });
                    }
                }

                // Strength
                if src.ingredient_strength_ratio_numerator_value.is_some() || 
                   src.ingredient_strength_codeable_concept_codes.is_some() || 
                   src.ingredient_strength_quantity_value.is_some() {
                    
                    let mut strength_x = StrengthX {
                        choice: None,
                    };

                    // Try ratio first
                    if let Some(numerator_values) = &src.ingredient_strength_ratio_numerator_value {
                        if let Some(numerator_value) = numerator_values.get(i) {
                            let mut ratio = Ratio {
                                ..Default::default()
                            };

                            // Numerator
                            ratio.numerator = Some(Quantity {
                                value: *numerator_value,
                                unit: src.ingredient_strength_ratio_numerator_unit.as_ref()
                                    .and_then(|units| units.get(i))
                                    .map(|unit| String {
                                        value: unit.clone(),
                                        ..Default::default()
                                    }),
                                system: src.ingredient_strength_ratio_numerator_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://unitsofmeasure.org".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.ingredient_strength_ratio_numerator_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            });

                            // Denominator
                            if let Some(denominator_values) = &src.ingredient_strength_ratio_denominator_value {
                                if let Some(denominator_value) = denominator_values.get(i) {
                                    ratio.denominator = Some(Quantity {
                                        value: *denominator_value,
                                        unit: src.ingredient_strength_ratio_denominator_unit.as_ref()
                                            .and_then(|units| units.get(i))
                                            .map(|unit| String {
                                                value: unit.clone(),
                                                ..Default::default()
                                            }),
                                        system: src.ingredient_strength_ratio_denominator_system.as_ref()
                                            .and_then(|systems| systems.get(i))
                                            .map(|system| Uri {
                                                value: system.clone(),
                                                ..Default::default()
                                            })
                                            .or_else(|| Some(Uri {
                                                value: "http://unitsofmeasure.org".to_string(),
                                                ..Default::default()
                                            })),
                                        code: src.ingredient_strength_ratio_denominator_code.as_ref()
                                            .and_then(|codes| codes.get(i))
                                            .map(|code| Code {
                                                value: code.clone(),
                                                ..Default::default()
                                            }),
                                        ..Default::default()
                                    });
                                }
                            }

                            strength_x.choice = Some(StrengthChoice::Ratio(ratio));
                        }
                    }
                    // Try codeable concept
                    else if let Some(codeable_concept_codes) = &src.ingredient_strength_codeable_concept_codes {
                        if let Some(code) = codeable_concept_codes.get(i) {
                            let codeable_concept = CodeableConcept {
                                coding: vec![Coding {
                                    system: src.ingredient_strength_codeable_concept_systems.as_ref()
                                        .and_then(|systems| systems.get(i))
                                        .map(|system| Uri {
                                            value: system.clone(),
                                            ..Default::default()
                                        })
                                        .or_else(|| Some(Uri {
                                            value: "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm".to_string(),
                                            ..Default::default()
                                        })),
                                    code: Some(Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                    display: src.ingredient_strength_codeable_concept_displays.as_ref()
                                        .and_then(|displays| displays.get(i))
                                        .map(|display| String {
                                            value: display.clone(),
                                            ..Default::default()
                                        }),
                                    ..Default::default()
                                }],
                                text: Some(String {
                                    value: code.clone(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            };

                            strength_x.choice = Some(StrengthChoice::CodeableConcept(codeable_concept));
                        }
                    }
                    // Try quantity
                    else if let Some(quantity_values) = &src.ingredient_strength_quantity_value {
                        if let Some(quantity_value) = quantity_values.get(i) {
                            let quantity = Quantity {
                                value: *quantity_value,
                                unit: src.ingredient_strength_quantity_unit.as_ref()
                                    .and_then(|units| units.get(i))
                                    .map(|unit| String {
                                        value: unit.clone(),
                                        ..Default::default()
                                    }),
                                system: src.ingredient_strength_quantity_system.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://unitsofmeasure.org".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.ingredient_strength_quantity_code.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            };

                            strength_x.choice = Some(StrengthChoice::Quantity(quantity));
                        }
                    }

                    ingredient.strength = Some(strength_x);
                }

                dest.ingredient.push(ingredient);
            }
        }

        // ------------------------------------------------------------------
        // 9. Batch  ---------------------------------------------------------
        if src.batch_lot_number.is_some() || src.batch_expiration_date.is_some() {
            let mut batch = Batch {
                ..Default::default()
            };

            if let Some(lot_number) = src.batch_lot_number {
                batch.lot_number = Some(String {
                    value: lot_number,
                    ..Default::default()
                });
            }

            if let Some(expiration_date) = src.batch_expiration_date {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&expiration_date) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    batch.expiration_date = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.batch = Some(batch);
        }

        // ------------------------------------------------------------------
        // 10. Definition  ---------------------------------------------------
        if let Some(definition_id) = src.definition_id {
            let definition_type = src.definition_type
                .unwrap_or_else(|| "MedicationKnowledge".to_string());

            dest.definition = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", definition_type, definition_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        dest
    }
}

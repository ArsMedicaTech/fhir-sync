use chrono::{DateTime, Utc};
use crate::domain::observation::DomainObservation;

// Generated modules ---------------------------------------------
use crate::proto::google::fhir::proto::r5::core::{
    Id, Identifier, Uri, String, CodeableConcept, Coding, Code, Reference,
    DateTime as FhirDateTime, Boolean, Quantity, Ratio, CodeableReference,
    Annotation, Markdown, Period, UnsignedInt, Duration, SimpleQuantity,
    Decimal, Age, Range, Timing, Canonical, Integer, Time, Attachment,
    Instant, SampledData,
};

// Placeholder for Observation until it's generated
pub struct Observation {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub instantiates: Option<InstantiatesX>,
    pub based_on: Vec<Reference>,
    pub triggered_by: Vec<TriggeredBy>,
    pub part_of: Vec<Reference>,
    pub status: Option<StatusCode>,
    pub category: Vec<CodeableConcept>,
    pub code: Option<CodeableConcept>,
    pub subject: Option<Reference>,
    pub focus: Vec<Reference>,
    pub encounter: Option<Reference>,
    pub effective: Option<EffectiveX>,
    pub issued: Option<Instant>,
    pub performer: Vec<Reference>,
    pub value: Option<ValueX>,
    pub data_absent_reason: Option<CodeableConcept>,
    pub interpretation: Vec<CodeableConcept>,
    pub note: Vec<Annotation>,
    pub body_site: Option<CodeableConcept>,
    pub body_structure: Option<Reference>,
    pub method: Option<CodeableConcept>,
    pub specimen: Option<Reference>,
    pub device: Option<Reference>,
    pub reference_range: Vec<ReferenceRange>,
    pub has_member: Vec<Reference>,
    pub derived_from: Vec<Reference>,
    pub component: Vec<Component>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct InstantiatesX {
    pub choice: Option<InstantiatesChoice>,
}

pub enum InstantiatesChoice {
    Canonical(Canonical),
    Reference(Reference),
}

pub struct TriggeredBy {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub observation: Option<Reference>,
    pub r#type: Option<TypeCode>,
    pub reason: Option<String>,
}

pub struct TypeCode {
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
    Timing(Timing),
    Instant(Instant),
}

pub struct ValueX {
    pub choice: Option<ValueChoice>,
}

pub enum ValueChoice {
    Quantity(Quantity),
    CodeableConcept(CodeableConcept),
    String(String),
    Boolean(Boolean),
    Integer(Integer),
    Range(Range),
    Ratio(Ratio),
    SampledData(SampledData),
    Time(Time),
    DateTime(FhirDateTime),
    Period(Period),
    Attachment(Attachment),
    Reference(Reference),
}

pub struct ReferenceRange {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub low: Option<SimpleQuantity>,
    pub high: Option<SimpleQuantity>,
    pub normal_value: Option<CodeableConcept>,
    pub r#type: Option<CodeableConcept>,
    pub applies_to: Vec<CodeableConcept>,
    pub age: Option<Range>,
    pub text: Option<Markdown>,
}

pub struct Component {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub code: Option<CodeableConcept>,
    pub value: Option<ValueX>,
    pub data_absent_reason: Option<CodeableConcept>,
    pub interpretation: Vec<CodeableConcept>,
    pub reference_range: Vec<ReferenceRange>,
}

pub struct Extension {
    pub id: Option<String>,
    pub url: String,
    pub value: Option<ExtensionValue>,
}

pub enum ExtensionValue {
    String(String),
    CodeableConcept(CodeableConcept),
}

impl Default for Observation {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            instantiates: None,
            based_on: Vec::new(),
            triggered_by: Vec::new(),
            part_of: Vec::new(),
            status: None,
            category: Vec::new(),
            code: None,
            subject: None,
            focus: Vec::new(),
            encounter: None,
            effective: None,
            issued: None,
            performer: Vec::new(),
            value: None,
            data_absent_reason: None,
            interpretation: Vec::new(),
            note: Vec::new(),
            body_site: None,
            body_structure: None,
            method: None,
            specimen: None,
            device: None,
            reference_range: Vec::new(),
            has_member: Vec::new(),
            derived_from: Vec::new(),
            component: Vec::new(),
        }
    }
}

impl From<DomainObservation> for Observation {
    fn from(src: DomainObservation) -> Self {
        let mut dest = Observation::default();

        // 1. Logical ID
        dest.id = Some(Id {
            value: src.id.clone(),
            ..Default::default()
        });

        // 2. Identifier
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:observation_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // 3. Instantiates
        if src.instantiates_canonical.is_some() || src.instantiates_reference_id.is_some() {
            let mut instantiates_x = InstantiatesX {
                choice: None,
            };

            if let Some(canonical) = src.instantiates_canonical {
                instantiates_x.choice = Some(InstantiatesChoice::Canonical(Canonical {
                    value: canonical,
                    ..Default::default()
                }));
            } else if let Some(reference_id) = src.instantiates_reference_id {
                instantiates_x.choice = Some(InstantiatesChoice::Reference(Reference {
                    reference: Some(String {
                        value: format!("ObservationDefinition/{}", reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }

            dest.instantiates = Some(instantiates_x);
        }

        // 4. Based On
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

        // 5. Triggered By
        for triggered_by in src.triggered_by {
            let mut fhir_triggered_by = TriggeredBy {
                ..Default::default()
            };

            fhir_triggered_by.observation = Some(Reference {
                reference: Some(String {
                    value: format!("Observation/{}", triggered_by.observation_id),
                    ..Default::default()
                }),
                ..Default::default()
            });

            let type_value = match triggered_by.r#type.to_lowercase().as_str() {
                "reflex" => 1,
                "repeat" => 2,
                "re-run" => 3,
                _ => 0,
            };

            fhir_triggered_by.r#type = Some(TypeCode {
                value: type_value,
                id: None,
                extension: Vec::new(),
            });

            if let Some(reason) = triggered_by.reason {
                fhir_triggered_by.reason = Some(String {
                    value: reason,
                    ..Default::default()
                });
            }

            dest.triggered_by.push(fhir_triggered_by);
        }

        // 6. Part Of
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

        // 7. Status
        let status_value = match src.status.to_lowercase().as_str() {
            "registered" => 1,
            "preliminary" => 2,
            "final" => 3,
            "amended" => 4,
            _ => 0,
        };

        dest.status = Some(StatusCode {
            value: status_value,
            id: None,
            extension: Vec::new(),
        });

        // 8. Category
        for (i, category) in src.category.iter().enumerate() {
            dest.category.push(CodeableConcept {
                coding: vec![Coding {
                    system: src.category_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
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

        // 9. Code
        dest.code = Some(CodeableConcept {
            coding: vec![Coding {
                system: src.code_system.map(|system| Uri {
                    value: system,
                    ..Default::default()
                }).or_else(|| Some(Uri {
                    value: "http://loinc.org".to_string(),
                    ..Default::default()
                })),
                code: Some(Code {
                    value: src.code,
                    ..Default::default()
                }),
                display: src.code_display.map(|display| String {
                    value: display,
                    ..Default::default()
                }),
                ..Default::default()
            }],
            text: Some(String {
                value: src.code.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // 10. Subject
        dest.subject = Some(Reference {
            reference: Some(String {
                value: format!("{}/{}", src.subject_type, src.subject_id),
                ..Default::default()
            }),
            ..Default::default()
        });

        // 11. Focus
        for (i, focus_id) in src.focus_ids.iter().enumerate() {
            let focus_type = src.focus_types.get(i)
                .cloned()
                .unwrap_or_else(|| "Resource".to_string());

            dest.focus.push(Reference {
                reference: Some(String {
                    value: format!("{}/{}", focus_type, focus_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 12. Encounter
        if let Some(encounter_id) = src.encounter_id {
            dest.encounter = Some(Reference {
                reference: Some(String {
                    value: format!("Encounter/{}", encounter_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 13. Effective
        if src.effective_date_time.is_some() || src.effective_period_start.is_some() || 
           src.effective_timing_code.is_some() || src.effective_instant.is_some() {
            
            let mut effective_x = EffectiveX {
                choice: None,
            };

            if let Some(effective_date_time) = src.effective_date_time {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&effective_date_time) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    effective_x.choice = Some(EffectiveChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    }));
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

                effective_x.choice = Some(EffectiveChoice::Period(period));
            } else if let Some(timing_code) = src.effective_timing_code {
                let timing = Timing {
                    code: Some(CodeableConcept {
                        coding: vec![Coding {
                            system: src.effective_timing_system.map(|system| Uri {
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
                            display: src.effective_timing_display.map(|display| String {
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

                effective_x.choice = Some(EffectiveChoice::Timing(timing));
            } else if let Some(instant) = src.effective_instant {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&instant) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    effective_x.choice = Some(EffectiveChoice::Instant(Instant {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    }));
                }
            }

            dest.effective = Some(effective_x);
        }

        // 14. Issued
        if let Some(issued) = src.issued {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&issued) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.issued = Some(Instant {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // 15. Performer
        for (i, performer_id) in src.performer_ids.iter().enumerate() {
            let performer_type = src.performer_types.get(i)
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

        // 16. Value
        if src.value_quantity_value.is_some() || src.value_codeable_concept_code.is_some() || 
           src.value_string.is_some() || src.value_boolean.is_some() || src.value_integer.is_some() ||
           src.value_range_low_value.is_some() || src.value_ratio_numerator_value.is_some() ||
           src.value_time.is_some() || src.value_date_time.is_some() || src.value_period_start.is_some() ||
           src.value_reference_id.is_some() {
            
            let mut value_x = ValueX {
                choice: None,
            };

            // Try Quantity first
            if let Some(quantity_value) = src.value_quantity_value {
                value_x.choice = Some(ValueChoice::Quantity(Quantity {
                    value: Some(quantity_value),
                    unit: src.value_quantity_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: src.value_quantity_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: src.value_quantity_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }
            // Try CodeableConcept
            else if let Some(concept_code) = src.value_codeable_concept_code {
                value_x.choice = Some(ValueChoice::CodeableConcept(CodeableConcept {
                    coding: vec![Coding {
                        system: src.value_codeable_concept_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: concept_code,
                            ..Default::default()
                        }),
                        display: src.value_codeable_concept_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: concept_code,
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }
            // Try String
            else if let Some(string_value) = src.value_string {
                value_x.choice = Some(ValueChoice::String(String {
                    value: string_value,
                    ..Default::default()
                }));
            }
            // Try Boolean
            else if let Some(boolean_value) = src.value_boolean {
                value_x.choice = Some(ValueChoice::Boolean(Boolean {
                    value: boolean_value,
                    ..Default::default()
                }));
            }
            // Try Integer
            else if let Some(integer_value) = src.value_integer {
                value_x.choice = Some(ValueChoice::Integer(Integer {
                    value: Some(integer_value),
                    ..Default::default()
                }));
            }
            // Try Range
            else if src.value_range_low_value.is_some() || src.value_range_high_value.is_some() {
                let mut range = Range {
                    ..Default::default()
                };

                if let Some(low_value) = src.value_range_low_value {
                    range.low = Some(SimpleQuantity {
                        value: Some(low_value),
                        unit: src.value_range_low_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: "1".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                if let Some(high_value) = src.value_range_high_value {
                    range.high = Some(SimpleQuantity {
                        value: Some(high_value),
                        unit: src.value_range_high_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: "1".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                value_x.choice = Some(ValueChoice::Range(range));
            }
            // Try Ratio
            else if src.value_ratio_numerator_value.is_some() || src.value_ratio_denominator_value.is_some() {
                let mut ratio = Ratio {
                    ..Default::default()
                };

                if let Some(numerator_value) = src.value_ratio_numerator_value {
                    ratio.numerator = Some(SimpleQuantity {
                        value: Some(numerator_value),
                        unit: src.value_ratio_numerator_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: "1".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                if let Some(denominator_value) = src.value_ratio_denominator_value {
                    ratio.denominator = Some(SimpleQuantity {
                        value: Some(denominator_value),
                        unit: src.value_ratio_denominator_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        }),
                        code: Some(Code {
                            value: "1".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                value_x.choice = Some(ValueChoice::Ratio(ratio));
            }
            // Try Time
            else if let Some(time_value) = src.value_time {
                value_x.choice = Some(ValueChoice::Time(Time {
                    value: time_value,
                    ..Default::default()
                }));
            }
            // Try DateTime
            else if let Some(date_time_value) = src.value_date_time {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&date_time_value) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    value_x.choice = Some(ValueChoice::DateTime(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    }));
                }
            }
            // Try Period
            else if src.value_period_start.is_some() || src.value_period_end.is_some() {
                let mut period = Period {
                    ..Default::default()
                };

                if let Some(period_start) = src.value_period_start {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.start = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                if let Some(period_end) = src.value_period_end {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.end = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                value_x.choice = Some(ValueChoice::Period(period));
            }
            // Try Reference
            else if let Some(reference_id) = src.value_reference_id {
                let reference_type = src.value_reference_type
                    .unwrap_or_else(|| "MolecularSequence".to_string());

                value_x.choice = Some(ValueChoice::Reference(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", reference_type, reference_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }

            dest.value = Some(value_x);
        }

        // 17. Data Absent Reason
        if let Some(data_absent_reason) = src.data_absent_reason {
            dest.data_absent_reason = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.data_absent_reason_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/data-absent-reason".to_string(),
                        ..Default::default()
                    })),
                    code: src.data_absent_reason_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.data_absent_reason_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: data_absent_reason,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 18. Interpretation
        for (i, interpretation) in src.interpretation.iter().enumerate() {
            dest.interpretation.push(CodeableConcept {
                coding: vec![Coding {
                    system: src.interpretation_system.get(i)
                        .map(|system| Uri {
                            value: system.clone(),
                            ..Default::default()
                        })
                        .or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation".to_string(),
                            ..Default::default()
                        })),
                    code: src.interpretation_code.get(i)
                        .map(|code| Code {
                            value: code.clone(),
                            ..Default::default()
                        }),
                    display: src.interpretation_display.get(i)
                        .map(|display| String {
                            value: display.clone(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: interpretation.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 19. Note
        for note_text in src.note {
            dest.note.push(Annotation {
                text: Some(String {
                    value: note_text,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 20. Body Site
        if let Some(body_site) = src.body_site {
            dest.body_site = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.body_site_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://snomed.info/sct".to_string(),
                        ..Default::default()
                    })),
                    code: src.body_site_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.body_site_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: body_site,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 21. Body Structure
        if let Some(body_structure_id) = src.body_structure_id {
            dest.body_structure = Some(Reference {
                reference: Some(String {
                    value: format!("BodyStructure/{}", body_structure_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 22. Method
        if let Some(method) = src.method {
            dest.method = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.method_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/observation-method".to_string(),
                        ..Default::default()
                    })),
                    code: src.method_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.method_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: method,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 23. Specimen
        if let Some(specimen_id) = src.specimen_id {
            let specimen_type = src.specimen_type
                .unwrap_or_else(|| "Specimen".to_string());

            dest.specimen = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", specimen_type, specimen_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 24. Device
        if let Some(device_id) = src.device_id {
            let device_type = src.device_type
                .unwrap_or_else(|| "Device".to_string());

            dest.device = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", device_type, device_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 25. Reference Range
        for reference_range in src.reference_range {
            let mut fhir_reference_range = ReferenceRange {
                ..Default::default()
            };

            // Low
            if let Some(low_value) = reference_range.low_value {
                fhir_reference_range.low = Some(SimpleQuantity {
                    value: Some(low_value),
                    unit: reference_range.low_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: reference_range.low_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: reference_range.low_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // High
            if let Some(high_value) = reference_range.high_value {
                fhir_reference_range.high = Some(SimpleQuantity {
                    value: Some(high_value),
                    unit: reference_range.high_unit.map(|unit| String {
                        value: unit,
                        ..Default::default()
                    }),
                    system: reference_range.high_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://unitsofmeasure.org".to_string(),
                        ..Default::default()
                    })),
                    code: reference_range.high_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Normal Value
            if let Some(normal_value) = reference_range.normal_value {
                fhir_reference_range.normal_value = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: reference_range.normal_value_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }),
                        code: reference_range.normal_value_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: reference_range.normal_value_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: normal_value,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Type
            if let Some(r#type) = reference_range.r#type {
                fhir_reference_range.r#type = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: reference_range.type_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/referencerange-meaning".to_string(),
                            ..Default::default()
                        })),
                        code: reference_range.type_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: reference_range.type_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: r#type,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Applies To
            for (i, applies_to) in reference_range.applies_to.iter().enumerate() {
                fhir_reference_range.applies_to.push(CodeableConcept {
                    coding: vec![Coding {
                        system: reference_range.applies_to_system.get(i)
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            }),
                        code: reference_range.applies_to_code.get(i)
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: reference_range.applies_to_display.get(i)
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: applies_to.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Age
            if reference_range.age_low_value.is_some() || reference_range.age_high_value.is_some() {
                let mut age_range = Range {
                    ..Default::default()
                };

                if let Some(low_value) = reference_range.age_low_value {
                    age_range.low = Some(SimpleQuantity {
                        value: Some(low_value),
                        unit: reference_range.age_low_unit.map(|unit| String {
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

                if let Some(high_value) = reference_range.age_high_value {
                    age_range.high = Some(SimpleQuantity {
                        value: Some(high_value),
                        unit: reference_range.age_high_unit.map(|unit| String {
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

                fhir_reference_range.age = Some(age_range);
            }

            // Text
            if let Some(text) = reference_range.text {
                fhir_reference_range.text = Some(Markdown {
                    value: text,
                    ..Default::default()
                });
            }

            dest.reference_range.push(fhir_reference_range);
        }

        // 26. Has Member
        for (i, has_member_id) in src.has_member_ids.iter().enumerate() {
            let has_member_type = src.has_member_types.get(i)
                .cloned()
                .unwrap_or_else(|| "Observation".to_string());

            dest.has_member.push(Reference {
                reference: Some(String {
                    value: format!("{}/{}", has_member_type, has_member_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 27. Derived From
        for (i, derived_from_id) in src.derived_from_ids.iter().enumerate() {
            let derived_from_type = src.derived_from_types.get(i)
                .cloned()
                .unwrap_or_else(|| "DocumentReference".to_string());

            dest.derived_from.push(Reference {
                reference: Some(String {
                    value: format!("{}/{}", derived_from_type, derived_from_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // 28. Component
        for component in src.component {
            let mut fhir_component = Component {
                ..Default::default()
            };

            // Code
            fhir_component.code = Some(CodeableConcept {
                coding: vec![Coding {
                    system: component.code_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://loinc.org".to_string(),
                        ..Default::default()
                    })),
                    code: Some(Code {
                        value: component.code,
                        ..Default::default()
                    }),
                    display: component.code_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: component.code.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            });

            // Value (similar logic to main value)
            if component.value_quantity_value.is_some() || component.value_codeable_concept_code.is_some() || 
               component.value_string.is_some() || component.value_boolean.is_some() || component.value_integer.is_some() {
                
                let mut value_x = ValueX {
                    choice: None,
                };

                if let Some(quantity_value) = component.value_quantity_value {
                    value_x.choice = Some(ValueChoice::Quantity(Quantity {
                        value: Some(quantity_value),
                        unit: component.value_quantity_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: component.value_quantity_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: component.value_quantity_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }));
                } else if let Some(concept_code) = component.value_codeable_concept_code {
                    value_x.choice = Some(ValueChoice::CodeableConcept(CodeableConcept {
                        coding: vec![Coding {
                            system: component.value_codeable_concept_system.map(|system| Uri {
                                value: system,
                                ..Default::default()
                            }),
                            code: Some(Code {
                                value: concept_code,
                                ..Default::default()
                            }),
                            display: component.value_codeable_concept_display.map(|display| String {
                                value: display,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }],
                        text: Some(String {
                            value: concept_code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }));
                } else if let Some(string_value) = component.value_string {
                    value_x.choice = Some(ValueChoice::String(String {
                        value: string_value,
                        ..Default::default()
                    }));
                } else if let Some(boolean_value) = component.value_boolean {
                    value_x.choice = Some(ValueChoice::Boolean(Boolean {
                        value: boolean_value,
                        ..Default::default()
                    }));
                } else if let Some(integer_value) = component.value_integer {
                    value_x.choice = Some(ValueChoice::Integer(Integer {
                        value: Some(integer_value),
                        ..Default::default()
                    }));
                }

                fhir_component.value = Some(value_x);
            }

            // Data Absent Reason
            if let Some(data_absent_reason) = component.data_absent_reason {
                fhir_component.data_absent_reason = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: component.data_absent_reason_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://terminology.hl7.org/CodeSystem/data-absent-reason".to_string(),
                            ..Default::default()
                        })),
                        code: component.data_absent_reason_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        display: component.data_absent_reason_display.map(|display| String {
                            value: display,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: data_absent_reason,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Interpretation
            for (i, interpretation) in component.interpretation.iter().enumerate() {
                fhir_component.interpretation.push(CodeableConcept {
                    coding: vec![Coding {
                        system: component.interpretation_system.get(i)
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation".to_string(),
                                ..Default::default()
                            })),
                        code: component.interpretation_code.get(i)
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: component.interpretation_display.get(i)
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: interpretation.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            // Reference Range (for component)
            for reference_range in component.reference_range {
                let mut fhir_reference_range = ReferenceRange {
                    ..Default::default()
                };

                if let Some(low_value) = reference_range.low_value {
                    fhir_reference_range.low = Some(SimpleQuantity {
                        value: Some(low_value),
                        unit: reference_range.low_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: reference_range.low_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: reference_range.low_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                if let Some(high_value) = reference_range.high_value {
                    fhir_reference_range.high = Some(SimpleQuantity {
                        value: Some(high_value),
                        unit: reference_range.high_unit.map(|unit| String {
                            value: unit,
                            ..Default::default()
                        }),
                        system: reference_range.high_system.map(|system| Uri {
                            value: system,
                            ..Default::default()
                        }).or_else(|| Some(Uri {
                            value: "http://unitsofmeasure.org".to_string(),
                            ..Default::default()
                        })),
                        code: reference_range.high_code.map(|code| Code {
                            value: code,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                if let Some(text) = reference_range.text {
                    fhir_reference_range.text = Some(Markdown {
                        value: text,
                        ..Default::default()
                    });
                }

                fhir_component.reference_range.push(fhir_reference_range);
            }

            dest.component.push(fhir_component);
        }

        dest
    }
}

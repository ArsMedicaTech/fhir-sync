use chrono::{DateTime, Utc};
use crate::domain::family_member_history::DomainFamilyMemberHistory;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: FamilyMemberHistory struct may not be generated yet
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
    Period,
    Annotation,
    Age,
    Range,
    Boolean,
    CodeableReference,
    Canonical,
};

// Placeholder for FamilyMemberHistory until it's generated
// This will be replaced with the actual struct when available
pub struct FamilyMemberHistory {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub instantiates_canonical: Vec<Canonical>,
    pub instantiates_uri: Vec<Uri>,
    pub status: Option<StatusCode>,
    pub data_absent_reason: Option<CodeableConcept>,
    pub patient: Option<Reference>,
    pub date: Option<FhirDateTime>,
    pub participant: Vec<Participant>,
    pub name: Option<String>,
    pub relationship: Option<CodeableConcept>,
    pub sex: Option<CodeableConcept>,
    pub born: Option<BornX>,
    pub age: Option<AgeX>,
    pub estimated_age: Option<Boolean>,
    pub deceased: Option<DeceasedX>,
    pub reason: Vec<CodeableReference>,
    pub note: Vec<Annotation>,
    pub condition: Vec<Condition>,
    pub procedure: Vec<Procedure>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct BornX {
    pub choice: Option<BornChoice>,
}

pub enum BornChoice {
    Period(Period),
    Date(Date),
    String(String),
}

pub struct AgeX {
    pub choice: Option<AgeChoice>,
}

pub enum AgeChoice {
    Age(Age),
    Range(Range),
    String(String),
}

pub struct DeceasedX {
    pub choice: Option<DeceasedChoice>,
}

pub enum DeceasedChoice {
    Boolean(Boolean),
    Age(Age),
    Range(Range),
    Date(Date),
    String(String),
}

pub struct Participant {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub function: Option<CodeableConcept>,
    pub actor: Option<Reference>,
}

pub struct Condition {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub code: Option<CodeableConcept>,
    pub outcome: Option<CodeableConcept>,
    pub contributed_to_death: Option<Boolean>,
    pub onset: Option<OnsetX>,
    pub note: Vec<Annotation>,
}

pub struct OnsetX {
    pub choice: Option<OnsetChoice>,
}

pub enum OnsetChoice {
    Age(Age),
    Range(Range),
    Period(Period),
    String(String),
}

pub struct Procedure {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub code: Option<CodeableConcept>,
    pub outcome: Option<CodeableConcept>,
    pub contributed_to_death: Option<Boolean>,
    pub performed: Option<PerformedX>,
    pub note: Vec<Annotation>,
}

pub struct PerformedX {
    pub choice: Option<PerformedChoice>,
}

pub enum PerformedChoice {
    Age(Age),
    Range(Range),
    Period(Period),
    String(String),
    DateTime(FhirDateTime),
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

impl Default for FamilyMemberHistory {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            instantiates_canonical: Vec::new(),
            instantiates_uri: Vec::new(),
            status: None,
            data_absent_reason: None,
            patient: None,
            date: None,
            participant: Vec::new(),
            name: None,
            relationship: None,
            sex: None,
            born: None,
            age: None,
            estimated_age: None,
            deceased: None,
            reason: Vec::new(),
            note: Vec::new(),
            condition: Vec::new(),
            procedure: Vec::new(),
        }
    }
}

impl From<DomainFamilyMemberHistory> for FamilyMemberHistory {
    fn from(src: DomainFamilyMemberHistory) -> Self {
        // Start with a completely empty message
        let mut dest = FamilyMemberHistory::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.family_member_history_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:family_member_history_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.family_member_history_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "partial" => 1,
                "completed" => 2,
                "entered-in-error" => 3,
                "health-unknown" => 4,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 4. Data Absent Reason  --------------------------------------------
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

        // ------------------------------------------------------------------
        // 5. Patient  -------------------------------------------------------
        dest.patient = Some(Reference {
            reference: Some(String {
                value: format!("Patient/{}", src.patient_demographic_no),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 6. Date  ----------------------------------------------------------
        if let Some(date_recorded) = src.date_recorded {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&date_recorded) {
                let fhir_dt = dt.with_timezone(&Utc);
                dest.date = Some(FhirDateTime {
                    value_us: fhir_dt.timestamp_micros(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 7. Participant  ---------------------------------------------------
        if let Some(participant_function_codes) = src.participant_function_codes {
            for (i, function_code) in participant_function_codes.iter().enumerate() {
                let mut participant = Participant {
                    ..Default::default()
                };

                // Function
                participant.function = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.participant_function_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string(),
                                ..Default::default()
                            })),
                        code: src.participant_function_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.participant_function_code_displays.as_ref()
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
                if let Some(participant_actor_ids) = &src.participant_actor_ids {
                    if let Some(actor_id) = participant_actor_ids.get(i) {
                        let actor_type = src.participant_actor_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Patient".to_string());

                        participant.actor = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", actor_type, actor_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.participant.push(participant);
            }
        }

        // ------------------------------------------------------------------
        // 8. Name  ----------------------------------------------------------
        if let Some(name) = src.name {
            dest.name = Some(String {
                value: name,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 9. Relationship  --------------------------------------------------
        if let Some(relationship) = src.relationship {
            dest.relationship = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.relationship_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(),
                        ..Default::default()
                    })),
                    code: src.relationship_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.relationship_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: relationship,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Sex  -----------------------------------------------------------
        if let Some(sex) = src.sex {
            dest.sex = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.sex_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://hl7.org/fhir/administrative-gender".to_string(),
                        ..Default::default()
                    })),
                    code: src.sex_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.sex_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: sex,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 11. Born  ----------------------------------------------------------
        if let Some(born_date) = src.born_date {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&born_date, "%Y-%m-%d") {
                dest.born = Some(BornX {
                    choice: Some(BornChoice::Date(Date {
                        value: date.format("%Y-%m-%d").to_string(),
                        ..Default::default()
                    })),
                });
            }
        } else if src.born_period_start.is_some() || src.born_period_end.is_some() {
            let mut period = Period {
                ..Default::default()
            };

            if let Some(period_start) = src.born_period_start {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(&period_start, "%Y-%m-%d") {
                    period.start = Some(FhirDateTime {
                        value_us: date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            if let Some(period_end) = src.born_period_end {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(&period_end, "%Y-%m-%d") {
                    period.end = Some(FhirDateTime {
                        value_us: date.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.born = Some(BornX {
                choice: Some(BornChoice::Period(period)),
            });
        } else if let Some(born_string) = src.born_string {
            dest.born = Some(BornX {
                choice: Some(BornChoice::String(String {
                    value: born_string,
                    ..Default::default()
                })),
            });
        }

        // ------------------------------------------------------------------
        // 12. Age  -----------------------------------------------------------
        if let Some(age_value) = src.age_value {
            let age_unit = src.age_unit.unwrap_or_else(|| "years".to_string());
            dest.age = Some(AgeX {
                choice: Some(AgeChoice::Age(Age {
                    value: age_value,
                    unit: Some(String {
                        value: age_unit,
                        ..Default::default()
                    }),
                    ..Default::default()
                })),
            });
        } else if src.age_range_low.is_some() || src.age_range_high.is_some() {
            let mut range = Range {
                ..Default::default()
            };

            if let Some(low) = src.age_range_low {
                range.low = Some(Age {
                    value: low,
                    unit: src.age_range_unit.as_ref().map(|unit| String {
                        value: unit.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            if let Some(high) = src.age_range_high {
                range.high = Some(Age {
                    value: high,
                    unit: src.age_range_unit.as_ref().map(|unit| String {
                        value: unit.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }

            dest.age = Some(AgeX {
                choice: Some(AgeChoice::Range(range)),
            });
        } else if let Some(age_string) = src.age_string {
            dest.age = Some(AgeX {
                choice: Some(AgeChoice::String(String {
                    value: age_string,
                    ..Default::default()
                })),
            });
        }

        // ------------------------------------------------------------------
        // 13. Estimated Age  -------------------------------------------------
        if let Some(estimated_age) = src.estimated_age {
            dest.estimated_age = Some(Boolean {
                value: estimated_age,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 14. Deceased  ------------------------------------------------------
        if let Some(deceased) = src.deceased {
            if deceased {
                if let Some(deceased_age_value) = src.deceased_age_value {
                    let deceased_age_unit = src.deceased_age_unit.unwrap_or_else(|| "years".to_string());
                    dest.deceased = Some(DeceasedX {
                        choice: Some(DeceasedChoice::Age(Age {
                            value: deceased_age_value,
                            unit: Some(String {
                                value: deceased_age_unit,
                                ..Default::default()
                            }),
                            ..Default::default()
                        })),
                    });
                } else if src.deceased_age_range_low.is_some() || src.deceased_age_range_high.is_some() {
                    let mut range = Range {
                        ..Default::default()
                    };

                    if let Some(low) = src.deceased_age_range_low {
                        range.low = Some(Age {
                            value: low,
                            unit: src.deceased_age_range_unit.as_ref().map(|unit| String {
                                value: unit.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }

                    if let Some(high) = src.deceased_age_range_high {
                        range.high = Some(Age {
                            value: high,
                            unit: src.deceased_age_range_unit.as_ref().map(|unit| String {
                                value: unit.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }

                    dest.deceased = Some(DeceasedX {
                        choice: Some(DeceasedChoice::Range(range)),
                    });
                } else if let Some(deceased_date) = src.deceased_date {
                    if let Ok(date) = chrono::NaiveDate::parse_from_str(&deceased_date, "%Y-%m-%d") {
                        dest.deceased = Some(DeceasedX {
                            choice: Some(DeceasedChoice::Date(Date {
                                value: date.format("%Y-%m-%d").to_string(),
                                ..Default::default()
                            })),
                        });
                    }
                } else if let Some(deceased_string) = src.deceased_string {
                    dest.deceased = Some(DeceasedX {
                        choice: Some(DeceasedChoice::String(String {
                            value: deceased_string,
                            ..Default::default()
                        })),
                    });
                } else {
                    dest.deceased = Some(DeceasedX {
                        choice: Some(DeceasedChoice::Boolean(Boolean {
                            value: true,
                            ..Default::default()
                        })),
                    });
                }
            } else {
                dest.deceased = Some(DeceasedX {
                    choice: Some(DeceasedChoice::Boolean(Boolean {
                        value: false,
                        ..Default::default()
                    })),
                });
            }
        }

        // ------------------------------------------------------------------
        // 15. Reason  --------------------------------------------------------
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
        // 16. Condition  -----------------------------------------------------
        if let Some(condition_codes) = src.condition_codes {
            for (i, condition_code) in condition_codes.iter().enumerate() {
                let mut condition = Condition {
                    ..Default::default()
                };

                // Code
                condition.code = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.condition_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
                                ..Default::default()
                            })),
                        code: src.condition_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.condition_code_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: condition_code.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Outcome
                if let Some(condition_outcomes) = &src.condition_outcomes {
                    if let Some(outcome) = condition_outcomes.get(i) {
                        condition.outcome = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.condition_outcome_systems.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.condition_outcome_codes.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.condition_outcome_displays.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: outcome.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Contributed to Death
                if let Some(contributed_to_death) = &src.condition_contributed_to_death {
                    if let Some(contributed) = contributed.get(i) {
                        condition.contributed_to_death = Some(Boolean {
                            value: *contributed,
                            ..Default::default()
                        });
                    }
                }

                // Onset
                if let Some(onset_ages) = &src.condition_onset_ages {
                    if let Some(onset_age) = onset_ages.get(i) {
                        let onset_age_unit = src.condition_onset_age_units.as_ref()
                            .and_then(|units| units.get(i))
                            .cloned()
                            .unwrap_or_else(|| "years".to_string());

                        condition.onset = Some(OnsetX {
                            choice: Some(OnsetChoice::Age(Age {
                                value: *onset_age,
                                unit: Some(String {
                                    value: onset_age_unit,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            })),
                        });
                    }
                } else if src.condition_onset_age_ranges_low.is_some() || src.condition_onset_age_ranges_high.is_some() {
                    let mut range = Range {
                        ..Default::default()
                    };

                    if let Some(low) = src.condition_onset_age_ranges_low.as_ref().and_then(|lows| lows.get(i)) {
                        range.low = Some(Age {
                            value: *low,
                            unit: src.condition_onset_age_range_units.as_ref()
                                .and_then(|units| units.get(i))
                                .map(|unit| String {
                                    value: unit.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        });
                    }

                    if let Some(high) = src.condition_onset_age_ranges_high.as_ref().and_then(|highs| highs.get(i)) {
                        range.high = Some(Age {
                            value: *high,
                            unit: src.condition_onset_age_range_units.as_ref()
                                .and_then(|units| units.get(i))
                                .map(|unit| String {
                                    value: unit.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        });
                    }

                    condition.onset = Some(OnsetX {
                        choice: Some(OnsetChoice::Range(range)),
                    });
                } else if src.condition_onset_periods_start.is_some() || src.condition_onset_periods_end.is_some() {
                    let mut period = Period {
                        ..Default::default()
                    };

                    if let Some(period_start) = src.condition_onset_periods_start.as_ref().and_then(|starts| starts.get(i)) {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(period_start, "%Y-%m-%d") {
                            period.start = Some(FhirDateTime {
                                value_us: date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(period_end) = src.condition_onset_periods_end.as_ref().and_then(|ends| ends.get(i)) {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(period_end, "%Y-%m-%d") {
                            period.end = Some(FhirDateTime {
                                value_us: date.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    condition.onset = Some(OnsetX {
                        choice: Some(OnsetChoice::Period(period)),
                    });
                } else if let Some(onset_strings) = &src.condition_onset_strings {
                    if let Some(onset_string) = onset_strings.get(i) {
                        condition.onset = Some(OnsetX {
                            choice: Some(OnsetChoice::String(String {
                                value: onset_string.clone(),
                                ..Default::default()
                            })),
                        });
                    }
                }

                // Note
                if let Some(condition_notes) = &src.condition_notes {
                    if let Some(note) = condition_notes.get(i) {
                        condition.note.push(Annotation {
                            text: Some(String {
                                value: note.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.condition.push(condition);
            }
        }

        // ------------------------------------------------------------------
        // 17. Procedure  -----------------------------------------------------
        if let Some(procedure_codes) = src.procedure_codes {
            for (i, procedure_code) in procedure_codes.iter().enumerate() {
                let mut procedure = Procedure {
                    ..Default::default()
                };

                // Code
                procedure.code = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.procedure_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://www.ama-assn.org/go/cpt".to_string(),
                                ..Default::default()
                            })),
                        code: src.procedure_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.procedure_code_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: procedure_code.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Outcome
                if let Some(procedure_outcomes) = &src.procedure_outcomes {
                    if let Some(outcome) = procedure_outcomes.get(i) {
                        procedure.outcome = Some(CodeableConcept {
                            coding: vec![Coding {
                                system: src.procedure_outcome_systems.as_ref()
                                    .and_then(|systems| systems.get(i))
                                    .map(|system| Uri {
                                        value: system.clone(),
                                        ..Default::default()
                                    })
                                    .or_else(|| Some(Uri {
                                        value: "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(),
                                        ..Default::default()
                                    })),
                                code: src.procedure_outcome_codes.as_ref()
                                    .and_then(|codes| codes.get(i))
                                    .map(|code| Code {
                                        value: code.clone(),
                                        ..Default::default()
                                    }),
                                display: src.procedure_outcome_displays.as_ref()
                                    .and_then(|displays| displays.get(i))
                                    .map(|display| String {
                                        value: display.clone(),
                                        ..Default::default()
                                    }),
                                ..Default::default()
                            }],
                            text: Some(String {
                                value: outcome.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Contributed to Death
                if let Some(contributed_to_death) = &src.procedure_contributed_to_death {
                    if let Some(contributed) = contributed.get(i) {
                        procedure.contributed_to_death = Some(Boolean {
                            value: *contributed,
                            ..Default::default()
                        });
                    }
                }

                // Performed
                if let Some(performed_ages) = &src.procedure_performed_ages {
                    if let Some(performed_age) = performed_ages.get(i) {
                        let performed_age_unit = src.procedure_performed_age_units.as_ref()
                            .and_then(|units| units.get(i))
                            .cloned()
                            .unwrap_or_else(|| "years".to_string());

                        procedure.performed = Some(PerformedX {
                            choice: Some(PerformedChoice::Age(Age {
                                value: *performed_age,
                                unit: Some(String {
                                    value: performed_age_unit,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            })),
                        });
                    }
                } else if src.procedure_performed_age_ranges_low.is_some() || src.procedure_performed_age_ranges_high.is_some() {
                    let mut range = Range {
                        ..Default::default()
                    };

                    if let Some(low) = src.procedure_performed_age_ranges_low.as_ref().and_then(|lows| lows.get(i)) {
                        range.low = Some(Age {
                            value: *low,
                            unit: src.procedure_performed_age_range_units.as_ref()
                                .and_then(|units| units.get(i))
                                .map(|unit| String {
                                    value: unit.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        });
                    }

                    if let Some(high) = src.procedure_performed_age_ranges_high.as_ref().and_then(|highs| highs.get(i)) {
                        range.high = Some(Age {
                            value: *high,
                            unit: src.procedure_performed_age_range_units.as_ref()
                                .and_then(|units| units.get(i))
                                .map(|unit| String {
                                    value: unit.clone(),
                                    ..Default::default()
                                }),
                            ..Default::default()
                        });
                    }

                    procedure.performed = Some(PerformedX {
                        choice: Some(PerformedChoice::Range(range)),
                    });
                } else if src.procedure_performed_periods_start.is_some() || src.procedure_performed_periods_end.is_some() {
                    let mut period = Period {
                        ..Default::default()
                    };

                    if let Some(period_start) = src.procedure_performed_periods_start.as_ref().and_then(|starts| starts.get(i)) {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(period_start, "%Y-%m-%d") {
                            period.start = Some(FhirDateTime {
                                value_us: date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(period_end) = src.procedure_performed_periods_end.as_ref().and_then(|ends| ends.get(i)) {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(period_end, "%Y-%m-%d") {
                            period.end = Some(FhirDateTime {
                                value_us: date.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    procedure.performed = Some(PerformedX {
                        choice: Some(PerformedChoice::Period(period)),
                    });
                } else if let Some(performed_dates) = &src.procedure_performed_dates {
                    if let Some(performed_date) = performed_dates.get(i) {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(performed_date) {
                            let fhir_dt = dt.with_timezone(&Utc);
                            procedure.performed = Some(PerformedX {
                                choice: Some(PerformedChoice::DateTime(FhirDateTime {
                                    value_us: fhir_dt.timestamp_micros(),
                                    ..Default::default()
                                })),
                            });
                        }
                    }
                } else if let Some(performed_strings) = &src.procedure_performed_strings {
                    if let Some(performed_string) = performed_strings.get(i) {
                        procedure.performed = Some(PerformedX {
                            choice: Some(PerformedChoice::String(String {
                                value: performed_string.clone(),
                                ..Default::default()
                            })),
                        });
                    }
                }

                // Note
                if let Some(procedure_notes) = &src.procedure_notes {
                    if let Some(note) = procedure_notes.get(i) {
                        procedure.note.push(Annotation {
                            text: Some(String {
                                value: note.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.procedure.push(procedure);
            }
        }

        // ------------------------------------------------------------------
        // 18. Notes  ---------------------------------------------------------
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

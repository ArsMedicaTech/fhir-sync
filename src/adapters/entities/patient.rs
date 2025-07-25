use chrono::NaiveDate;
use prost::Message;

use crate::domain::DomainPatient;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use google::fhir::proto::r5::core::{
    resources::patient::Patient, // proto message we’re producing
    datatypes,                   // primitive / complex data types
    codes,                       // code value-sets
};

// Shorthand for nested message that lives *inside* Patient.
type GenderCode = patient::GenderCode;

impl From<DomainPatient> for Patient {
    fn from(src: DomainPatient) -> Self {
        // Start with a completely empty message
        let mut dest = Patient::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(datatypes::Id {
            value: src.demographic_no.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(datatypes::Identifier {
            system: Some(datatypes::Uri {
                value: "urn:arsmedicatech:demographic_no".to_string(),
                ..Default::default()
            }),
            value: Some(datatypes::String {
                value: src.demographic_no.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Name  ----------------------------------------------------------
        if src.first_name.is_some() || src.last_name.is_some() {
            let mut name = datatypes::HumanName::default();

            if let Some(fam) = src.last_name {
                name.family = Some(datatypes::String { value: fam, ..Default::default() });
            }
            if let Some(giv) = src.first_name {
                name.given.push(datatypes::String { value: giv, ..Default::default() });
            }

            dest.name.push(name);
        }

        // ------------------------------------------------------------------
        // 4. Telecom  (phone & e-mail) --------------------------------------
        if let Some(phone) = src.phone {
            dest.telecom.push(datatypes::ContactPoint {
                system: Some(datatypes::contact_point::SystemCode {
                    value: codes::ContactPointSystemCode::Phone as i32,
                    ..Default::default()
                }),
                value: Some(datatypes::String { value: phone, ..Default::default() }),
                ..Default::default()
            });
        }
        if let Some(email) = src.email {
            dest.telecom.push(datatypes::ContactPoint {
                system: Some(datatypes::contact_point::SystemCode {
                    value: codes::ContactPointSystemCode::Email as i32,
                    ..Default::default()
                }),
                value: Some(datatypes::String { value: email, ..Default::default() }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 5. Gender  --------------------------------------------------------
        if let Some(sex_raw) = src.sex {
            let code_val = match sex_raw.to_lowercase().as_str() {
                "male" | "m"      => codes::AdministrativeGenderCode::Male,
                "female" | "f"    => codes::AdministrativeGenderCode::Female,
                "other"           => codes::AdministrativeGenderCode::Other,
                _                 => codes::AdministrativeGenderCode::Unknown,
            };

            dest.gender = Some(GenderCode {
                value: code_val as i32,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Birth date  ----------------------------------------------------
        if let Some(dob) = src.date_of_birth {
            if let Ok(date) = NaiveDate::parse_from_str(&dob, "%Y-%m-%d") {
                // FHIR primitive Date uses microseconds since Unix epoch
                let us = date.and_hms_opt(0, 0, 0).unwrap().timestamp() * 1_000_000;
                dest.birth_date = Some(datatypes::Date { value_us: us, ..Default::default() });
            }
        }

        // ------------------------------------------------------------------
        // 7. Address  -------------------------------------------------------
        if let Some((city, province, country, postal)) = src.location {
            dest.address.push(datatypes::Address {
                city:        Some(datatypes::String { value: city,      ..Default::default() }),
                state:       Some(datatypes::String { value: province,  ..Default::default() }),
                country:     Some(datatypes::String { value: country,   ..Default::default() }),
                postal_code: Some(datatypes::String { value: postal,    ..Default::default() }),
                ..Default::default()
            });
        }

        dest
    }
}

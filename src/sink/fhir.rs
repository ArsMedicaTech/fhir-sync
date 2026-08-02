//! Sink task: consumes `SyncEvent`s and conditionally upserts a FHIR R4B
//! `Patient` into HAPI (D5). Owns the `rx` end of the channel — there is
//! exactly one consumer (D4).

use anyhow::{Context, Result};
use fhirbolt::model::r4b::resources::Patient;
use fhirbolt::model::r4b::types::{Address, ContactPoint, HumanName, Identifier, Meta};
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

use crate::config::{Config, FhirConfig};
use crate::domain::patient::DomainPatient;
use crate::event::{Op, SyncEvent};

const META_SOURCE: &str = "urn:arsmedicatech:fhir-sync:oscar";

/// Runs the sink to completion (until the channel closes).
pub async fn run(cfg: Config, mut rx: Receiver<SyncEvent>) -> Result<()> {
    let client = reqwest::Client::new();
    let token = cfg
        .fhir
        .token_env
        .as_ref()
        .and_then(|key| std::env::var(key).ok());

    while let Some(event) = rx.recv().await {
        let key = event.idempotency_key.clone();
        if let Err(e) = sync_one(&client, &cfg.fhir, token.as_deref(), &event).await {
            // Phase 2 (F6/dead-letter) will add retry + backoff here.
            // For now: log and keep the stream alive (never crash on one bad record).
            error!("fhir sink: failed to sync {key}: {e:?}");
        }
    }

    Ok(())
}

async fn sync_one(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<&str>,
    event: &SyncEvent,
) -> Result<()> {
    let mut patient = build_patient(&event.payload, fhir_cfg);
    if event.op == Op::Delete {
        patient.active = Some(false.into());
    }

    let body = fhirbolt::json::to_string(&patient, None).context("serializing FHIR Patient")?;

    let url = format!(
        "{}/Patient?identifier={}|{}",
        fhir_cfg.base_url.trim_end_matches('/'),
        fhir_cfg.oscar_demographic_system,
        event.payload.demographic_no,
    );

    let mut req = client
        .put(&url)
        .header("Content-Type", "application/fhir+json")
        .body(body);

    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let resp = req.send().await.context("sending conditional PUT to HAPI")?;
    let status = resp.status();

    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("HAPI conditional PUT failed ({status}): {text}");
    }

    info!("fhir sink: synced {} -> {}", event.idempotency_key, url);
    Ok(())
}

/// M/male -> male, F/female -> female, else unknown. Never omitted (D5).
fn map_gender(sex: Option<&str>) -> &'static str {
    match sex.map(|s| s.to_ascii_uppercase()) {
        Some(s) if s == "M" || s == "MALE" => "male",
        Some(s) if s == "F" || s == "FEMALE" => "female",
        _ => "unknown",
    }
}

fn build_patient(payload: &DomainPatient, cfg: &FhirConfig) -> Patient {
    let mut patient = Patient::default();

    patient.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    patient.identifier.push(Identifier {
        system: Some(cfg.oscar_demographic_system.clone().into()),
        value: Some(payload.demographic_no.clone().into()),
        ..Default::default()
    });

    // Provincial PHN, only if this Oscar instance has it.
    if let Some(hin) = &payload.hin {
        patient.identifier.push(Identifier {
            system: Some(cfg.oscar_hin_system.clone().into()),
            value: Some(hin.clone().into()),
            ..Default::default()
        });
    }

    if payload.first_name.is_some() || payload.last_name.is_some() {
        patient.name.push(HumanName {
            family: payload.last_name.clone().map(Into::into),
            given: payload
                .first_name
                .clone()
                .map(|g| vec![g.into()])
                .unwrap_or_default(),
            ..Default::default()
        });
    }

    if let Some(dob) = &payload.date_of_birth {
        patient.birth_date = Some(dob.clone().into());
    }

    // Never omitted — falls back to "unknown" (D5).
    patient.gender = Some(map_gender(payload.sex.as_deref()).into());

    if let Some(email) = &payload.email {
        patient.telecom.push(ContactPoint {
            system: Some("email".into()),
            value: Some(email.clone().into()),
            ..Default::default()
        });
    }

    if let Some(phone) = &payload.phone {
        patient.telecom.push(ContactPoint {
            system: Some("phone".into()),
            value: Some(phone.clone().into()),
            ..Default::default()
        });
    }

    if let Some((city, province, country, postal)) = &payload.location {
        patient.address.push(Address {
            city: Some(city.clone().into()),
            state: Some(province.clone().into()),
            country: Some(country.clone().into()),
            postal_code: Some(postal.clone().into()),
            ..Default::default()
        });
    }

    patient
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fhir_cfg() -> FhirConfig {
        FhirConfig {
            base_url: "http://localhost:8082/fhir".to_string(),
            oscar_demographic_system: "https://arsmedicatech.com/fhir/sid/oscar-demographic-no"
                .to_string(),
            oscar_hin_system: "https://arsmedicatech.com/fhir/sid/oscar-hin".to_string(),
            token_env: None,
        }
    }

    #[test]
    fn gender_mapping_never_omits() {
        assert_eq!(map_gender(Some("M")), "male");
        assert_eq!(map_gender(Some("female")), "female");
        assert_eq!(map_gender(Some("other")), "unknown");
        assert_eq!(map_gender(None), "unknown");
    }

    #[test]
    fn build_patient_omits_absent_telecom_and_address() {
        let payload = DomainPatient {
            demographic_no: "123".to_string(),
            first_name: Some("Alice".to_string()),
            last_name: Some("Smith".to_string()),
            date_of_birth: Some("1990-03-05".to_string()),
            location: None,
            sex: Some("F".to_string()),
            phone: None,
            email: None,
            hin: None,
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert!(patient.telecom.is_empty());
        assert!(patient.address.is_empty());
        assert_eq!(patient.identifier[0].value, Some("123".to_string().into()));
    }
}

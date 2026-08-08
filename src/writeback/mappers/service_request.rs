use std::collections::HashMap;

use chrono::{DateTime, LocalResult, NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use serde_json::Value;
use tracing::warn;

use super::MappingError;

const PLACER_ORDER_SYSTEM: &str = "https://arsmedicatech.com/fhir/identifier/placer-order";

#[derive(Debug, Default)]
pub struct ConsultationRequestRow {
    pub demographic_no: Option<String>,
    pub provider_no: Option<String>,
    pub service_id: Option<String>,
    pub spec_id: Option<String>,
    pub referal_date: Option<String>,
    pub reason: Option<String>,
    pub clinical_info: Option<String>,
    pub concurrent_problems: Option<String>,
    pub current_meds: Option<String>,
    pub allergies: Option<String>,
    pub status: Option<String>,
    pub urgency: Option<String>,
    pub source: Option<String>,
    pub send_to: Option<String>,
    pub placer_order_id: Option<String>,
}

/// Maps a FHIR `ServiceRequest` to an Oscar `consultationRequests` row.
///
/// Returns the existing `oscar-consult-request` identifier value (if present)
/// and the mapped row. A missing identifier means INSERT; a present value
/// means UPDATE.
pub fn fhir_service_request_to_row(
    service_request: &Value,
    oscar_demographic_system: &str,
    oscar_provider_system: &str,
    oscar_consult_request_system: &str,
    consult_service_map: &HashMap<String, String>,
    default_consult_provider_no: &Option<String>,
    tz: &Tz,
) -> Result<(Option<String>, ConsultationRequestRow), MappingError> {
    let mut row = ConsultationRequestRow::default();

    let request_id = identifier_value(service_request, oscar_consult_request_system);
    row.placer_order_id = identifier_value(service_request, PLACER_ORDER_SYSTEM);

    let subject = service_request
        .get("subject")
        .ok_or_else(|| MappingError::MissingField("subject".to_string()))?;
    let demographic_no = reference_identifier(subject, oscar_demographic_system)
        .ok_or(MappingError::NoDemographic)?;
    if demographic_no == "0" {
        return Err(MappingError::PlaceholderPatient);
    }
    row.demographic_no = Some(demographic_no);

    if let Some(requester) = service_request.get("requester") {
        row.provider_no = reference_identifier(requester, oscar_provider_system)
            .or_else(|| default_consult_provider_no.clone());
    } else {
        row.provider_no = default_consult_provider_no.clone();
    }
    if row.provider_no.is_none() {
        return Err(MappingError::MissingField("provider_no".to_string()));
    }

    let (service_id, reason) = code_to_service_and_reason(service_request, consult_service_map);
    row.service_id = Some(service_id);
    row.reason = reason;

    // Phase 1: no performer -> professionalSpecialists resolution.
    row.spec_id = None;

    if let Some(authored) = service_request.get("authoredOn").and_then(Value::as_str) {
        row.referal_date = Some(parse_local_date(authored, tz)?);
    }

    if let Some(notes) = service_request.get("note").and_then(Value::as_array) {
        let texts: Vec<String> = notes
            .iter()
            .filter_map(|n| n.get("text").and_then(Value::as_str).map(String::from))
            .collect();
        if !texts.is_empty() {
            row.clinical_info = Some(texts.join("\n\n"));
        }
    }

    if service_request.get("priority").is_some() {
        warn!("ServiceRequest.priority mapping is unresolved; leaving urgency NULL");
    }

    row.status = Some("1".to_string());
    row.source = Some("AMT-eReferral".to_string());

    Ok((request_id, row))
}

fn code_to_service_and_reason(
    service_request: &Value,
    consult_service_map: &HashMap<String, String>,
) -> (String, Option<String>) {
    let code = service_request.get("code");

    let text = code
        .and_then(|c| c.get("text"))
        .and_then(Value::as_str)
        .map(String::from);

    let first_display = code
        .and_then(|c| c.get("coding"))
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(|c| c.get("display"))
        .and_then(Value::as_str)
        .map(String::from);

    let reason = text.clone().or_else(|| first_display.clone());

    let mut service_id = None;
    for candidate in [&text, &first_display] {
        if let Some(c) = candidate {
            if let Some(id) = consult_service_map.get(&c.to_lowercase()) {
                service_id = Some(id.clone());
                break;
            }
        }
    }

    let service_id = service_id.unwrap_or_else(|| {
        let looked_at = text.as_deref().or_else(|| first_display.as_deref()).unwrap_or("?");
        warn!("unmapped consultation service {looked_at:?}; using SEE NOTES (57)");
        "57".to_string()
    });

    (service_id, reason)
}

fn parse_local_date(s: &str, tz: &Tz) -> Result<String, MappingError> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(tz).date_naive().format("%Y-%m-%d").to_string());
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return match tz.from_local_datetime(&dt) {
            LocalResult::Single(t) => Ok(t.date_naive().format("%Y-%m-%d").to_string()),
            LocalResult::Ambiguous(earliest, _latest) => {
                warn!(
                    "ambiguous local date {s} in {tz}: using earliest occurrence ({})",
                    earliest.format("%Y-%m-%dT%H:%M:%S%:z")
                );
                Ok(earliest.date_naive().format("%Y-%m-%d").to_string())
            }
            LocalResult::None => Err(MappingError::NonexistentLocalTime(s.to_string())),
        };
    }

    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(d.format("%Y-%m-%d").to_string());
    }

    Err(MappingError::InvalidValue {
        field: "authoredOn".to_string(),
        value: s.to_string(),
    })
}

fn identifier_value(resource: &Value, system: &str) -> Option<String> {
    resource
        .get("identifier")
        .and_then(Value::as_array)
        .and_then(|ids| {
            ids.iter()
                .find(|i| i.get("system").and_then(Value::as_str) == Some(system))
                .and_then(|i| i.get("value").and_then(Value::as_str))
        })
        .map(String::from)
}

fn reference_identifier(reference: &Value, system: &str) -> Option<String> {
    if let Some(id) = reference.get("identifier") {
        if let Some(arr) = id.as_array() {
            if let Some(v) = arr
                .iter()
                .find(|i| i.get("system").and_then(Value::as_str) == Some(system))
                .and_then(|i| i.get("value").and_then(Value::as_str))
            {
                return Some(v.to_string());
            }
        }
        if id.is_object() {
            if id.get("system").and_then(Value::as_str) == Some(system) {
                return id.get("value").and_then(Value::as_str).map(String::from);
            }
        }
    }

    if let Some(reference_str) = reference.get("reference").and_then(Value::as_str) {
        if let Some(query) = reference_str.split('?').nth(1) {
            for (k, v) in url::form_urlencoded::parse(query.as_bytes()) {
                if k == "identifier" {
                    if let Some((sys, val)) = v.split_once('|') {
                        if sys == system {
                            return Some(val.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

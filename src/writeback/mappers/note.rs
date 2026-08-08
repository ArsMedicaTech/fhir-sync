use base64ct::{Base64, Encoding};
use chrono::DateTime;
use chrono_tz::Tz;
use serde_json::Value;

use super::MappingError;

#[derive(Debug, Default)]
pub struct NoteRow {
    pub uuid: Option<String>,
    pub demographic_no: Option<String>,
    pub provider_no: Option<String>,
    pub signing_provider_no: Option<String>,
    pub observation_date: Option<String>,
    pub update_date: Option<String>,
    pub encounter_type: Option<String>,
    pub note: Option<String>,
    pub signed: bool,
    pub archived: bool,
    pub appointment_no: Option<String>,
}

/// Maps a FHIR `DocumentReference` (the note content) to a `casemgmt_note`
/// `NoteRow`.
///
/// The returned `uuid` is the note's identity.  A missing
/// `oscar-note-document` identifier means a brand-new note; a present value
/// means a new revision of that existing note.
///
/// `observation_date` comes from `DocumentReference.date`.  `update_date` is
/// intentionally left empty; the sink fills it with the current time when it
/// appends the new `casemgmt_note` row.
pub fn fhir_document_reference_to_row(
    doc_ref: &Value,
    oscar_note_document_system: &str,
    demographic_no: Option<String>,
    provider_no: Option<String>,
    signing_provider_no: Option<String>,
    appointment_no: Option<String>,
    tz: &Tz,
) -> Result<(Option<String>, NoteRow), MappingError> {
    let mut row = NoteRow::default();

    let uuid = identifier_value(doc_ref, oscar_note_document_system);

    let demographic_no = demographic_no.ok_or(MappingError::NoDemographic)?;
    if demographic_no == "0" {
        return Err(MappingError::PlaceholderPatient);
    }
    row.demographic_no = Some(demographic_no);
    row.provider_no = provider_no;
    row.signing_provider_no = signing_provider_no;

    match doc_ref.get("docStatus").and_then(Value::as_str) {
        Some("final") => row.signed = true,
        Some("preliminary") | None => row.signed = false,
        Some(other) => {
            return Err(MappingError::InvalidValue {
                field: "docStatus".to_string(),
                value: other.to_string(),
            });
        }
    }

    match doc_ref.get("status").and_then(Value::as_str) {
        Some("entered-in-error") => row.archived = true,
        Some("current") | None => row.archived = false,
        Some(other) => {
            return Err(MappingError::InvalidValue {
                field: "status".to_string(),
                value: other.to_string(),
            });
        }
    }

    if let Some(date) = doc_ref.get("date").and_then(Value::as_str) {
        row.observation_date = Some(parse_instant_to_local(date, tz)?);
    }

    let data = doc_ref
        .get("content")
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(|c| c.get("attachment"))
        .and_then(|a| a.get("data"))
        .and_then(Value::as_str)
        .ok_or_else(|| MappingError::MissingField("note".to_string()))?;
    let decoded = Base64::decode_vec(data).map_err(|_| MappingError::InvalidValue {
        field: "note".to_string(),
        value: data.to_string(),
    })?;
    row.note = Some(String::from_utf8(decoded).map_err(|_| MappingError::InvalidValue {
        field: "note".to_string(),
        value: "not utf-8".to_string(),
    })?);

    row.appointment_no = appointment_no;

    Ok((uuid, row))
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

fn parse_instant_to_local(s: &str, tz: &Tz) -> Result<String, MappingError> {
    let dt = DateTime::parse_from_rfc3339(s).map_err(|_| MappingError::InvalidValue {
        field: "instant".to_string(),
        value: s.to_string(),
    })?;
    let local = dt.with_timezone(tz);
    Ok(local.naive_local().format("%Y-%m-%d %H:%M:%S").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vancouver() -> Tz {
        "America/Vancouver".parse().unwrap()
    }

    fn doc_ref() -> Value {
        serde_json::json!({
            "resourceType": "DocumentReference",
            "identifier": [
                { "system": "https://arsmedicatech.com/fhir/sid/oscar-note-document", "value": "aaaaaaaa-0000-4000-8000-000000000001" }
            ],
            "status": "current",
            "docStatus": "preliminary",
            "subject": {
                "reference": "Patient?identifier=https://arsmedicatech.com/fhir/sid/oscar-demographic|101"
            },
            "date": "2026-08-06T09:20:00-07:00",
            "author": [
                { "reference": "Practitioner?identifier=https://arsmedicatech.com/fhir/sid/oscar-provider|100001" }
            ],
            "authenticator": {
                "reference": "Practitioner?identifier=https://arsmedicatech.com/fhir/sid/oscar-provider|100003"
            },
            "context": {
                "encounter": [
                    { "reference": "Appointment?identifier=https://arsmedicatech.com/fhir/sid/oscar-appointment|5" }
                ]
            },
            "content": [
                {
                    "attachment": {
                        "contentType": "text/plain",
                        "data": "SGVsbG8="
                    }
                }
            ]
        })
    }

    #[test]
    fn maps_document_reference() {
        let (uuid, row) = fhir_document_reference_to_row(
            &doc_ref(),
            "https://arsmedicatech.com/fhir/sid/oscar-note-document",
            Some("101".to_string()),
            Some("100001".to_string()),
            Some("100003".to_string()),
            Some("5".to_string()),
            &vancouver(),
        )
        .unwrap();
        assert_eq!(uuid, Some("aaaaaaaa-0000-4000-8000-000000000001".to_string()));
        assert_eq!(row.demographic_no, Some("101".to_string()));
        assert_eq!(row.provider_no, Some("100001".to_string()));
        assert_eq!(row.signing_provider_no, Some("100003".to_string()));
        assert_eq!(row.note, Some("Hello".to_string()));
        assert!(!row.signed);
        assert!(!row.archived);
        assert_eq!(row.observation_date, Some("2026-08-06 09:20:00".to_string()));
        assert_eq!(row.appointment_no, Some("5".to_string()));
        assert!(row.update_date.is_none());
    }

    #[test]
    fn entered_in_error_archives() {
        let mut d = doc_ref();
        d["status"] = "entered-in-error".into();
        let (_, row) = fhir_document_reference_to_row(
            &d,
            "https://arsmedicatech.com/fhir/sid/oscar-note-document",
            Some("101".to_string()),
            Some("100001".to_string()),
            Some("100003".to_string()),
            Some("5".to_string()),
            &vancouver(),
        )
        .unwrap();
        assert!(row.archived);
        assert!(!row.signed);
    }

    #[test]
    fn final_doc_status_means_signed() {
        let mut d = doc_ref();
        d["docStatus"] = "final".into();
        let (_, row) = fhir_document_reference_to_row(
            &d,
            "https://arsmedicatech.com/fhir/sid/oscar-note-document",
            Some("101".to_string()),
            Some("100001".to_string()),
            Some("100003".to_string()),
            Some("5".to_string()),
            &vancouver(),
        )
        .unwrap();
        assert!(row.signed);
        assert!(!row.archived);
    }

    #[test]
    fn missing_note_data_errors() {
        let mut d = doc_ref();
        d["content"] = serde_json::json!([{ "attachment": { "contentType": "text/plain" } }]);
        let err = fhir_document_reference_to_row(
            &d,
            "https://arsmedicatech.com/fhir/sid/oscar-note-document",
            Some("101".to_string()),
            Some("100001".to_string()),
            Some("100003".to_string()),
            Some("5".to_string()),
            &vancouver(),
        )
        .unwrap_err();
        assert_eq!(err, MappingError::MissingField("note".to_string()));
    }
}

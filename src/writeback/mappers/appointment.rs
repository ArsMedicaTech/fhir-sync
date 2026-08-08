use std::collections::HashMap;

use chrono::{DateTime, LocalResult, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use serde_json::Value;
use tracing::warn;

use super::MappingError;

#[derive(Debug, Default)]
pub struct AppointmentRow {
    pub demographic_no: Option<String>,
    pub provider_no: Option<String>,
    pub appointment_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub booking_source: Option<String>,
    pub lastupdateuser: Option<String>,
}

/// Maps a FHIR `Appointment` resource to an Oscar `appointment` row.
///
/// `appointment_no` is returned when the resource carries the
/// `oscar-appointment-no` identifier, signalling an UPDATE.  A missing
/// identifier means INSERT.
pub fn fhir_appointment_to_row(
    appointment: &Value,
    demographic_no: Option<String>,
    provider_no: Option<String>,
    oscar_appointment_system: &str,
    status_map: &HashMap<String, String>,
    tz: &Tz,
) -> Result<(Option<String>, AppointmentRow), MappingError> {
    let mut row = AppointmentRow::default();
    row.booking_source = Some("OSCAR".to_string());

    let appointment_no = appointment
        .get("identifier")
        .and_then(Value::as_array)
        .and_then(|ids| {
            ids.iter()
                .find(|i| {
                    i.get("system").and_then(Value::as_str) == Some(oscar_appointment_system)
                })
                .and_then(|i| i.get("value").and_then(Value::as_str))
        })
        .map(String::from);

    let demographic_no = demographic_no.ok_or(MappingError::NoDemographic)?;
    if demographic_no == "0" {
        return Err(MappingError::PlaceholderPatient);
    }

    let start = appointment
        .get("start")
        .and_then(Value::as_str)
        .ok_or_else(|| MappingError::MissingField("start".to_string()))?;
    let end = appointment
        .get("end")
        .and_then(Value::as_str)
        .ok_or_else(|| MappingError::MissingField("end".to_string()))?;

    let start_dt = parse_local_datetime(start, tz)?;
    let end_dt = parse_local_datetime(end, tz)?;

    row.appointment_date = Some(start_dt.date_naive().format("%Y-%m-%d").to_string());
    row.start_time = Some(start_dt.time().format("%H:%M:%S%.f").to_string());
    row.end_time = Some(end_dt.time().format("%H:%M:%S%.f").to_string());

    let status = appointment
        .get("status")
        .and_then(Value::as_str)
        .ok_or_else(|| MappingError::MissingField("status".to_string()))?;
    row.status = Some(
        status_map
            .get(status)
            .cloned()
            .ok_or_else(|| MappingError::UnmappedAppointmentStatus(status.to_string()))?,
    );

    row.reason = reason_text(appointment);
    row.demographic_no = Some(demographic_no);
    row.provider_no = provider_no;

    Ok((appointment_no, row))
}



fn reason_text(appointment: &Value) -> Option<String> {
    let from_reason_code = appointment
        .get("reasonCode")
        .and_then(Value::as_array)
        .and_then(|arr| {
            arr.iter().find_map(|rc| {
                rc.get("text")
                    .and_then(Value::as_str)
                    .map(String::from)
                    .or_else(|| {
                        rc.get("coding")
                            .and_then(Value::as_array)
                            .and_then(|c| c.first())
                            .and_then(|c| c.get("display").and_then(Value::as_str))
                            .map(String::from)
                    })
            })
        });

    from_reason_code
        .or_else(|| appointment.get("description").and_then(Value::as_str).map(String::from))
        .or_else(|| appointment.get("comment").and_then(Value::as_str).map(String::from))
}

fn parse_local_datetime(s: &str, tz: &Tz) -> Result<DateTime<Tz>, MappingError> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(tz));
    }

    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .map_err(|_| MappingError::InvalidValue {
            field: "datetime".to_string(),
            value: s.to_string(),
        })?;

    match tz.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::Ambiguous(earliest, _latest) => {
            warn!(
                "ambiguous local time {s} in {tz}: using earliest occurrence ({})",
                earliest.format("%Y-%m-%dT%H:%M:%S%:z")
            );
            Ok(earliest)
        }
        LocalResult::None => Err(MappingError::NonexistentLocalTime(s.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn appt(start: &str, end: &str) -> Value {
        serde_json::json!({
            "resourceType": "Appointment",
            "id": "1014",
            "identifier": [
                { "system": "https://arsmedicatech.com/fhir/sid/oscar-appointment", "value": "1" }
            ],
            "status": "booked",
            "start": start,
            "end": end,
            "participant": [
                { "actor": { "identifier": { "system": "https://arsmedicatech.com/fhir/sid/oscar-demographic", "value": "101" } } },
                { "actor": { "identifier": { "system": "https://arsmedicatech.com/fhir/sid/oscar-provider", "value": "100001" } } }
            ],
            "reasonCode": [{ "text": "Follow-up" }]
        })
    }

    fn vancouver() -> Tz {
        "America/Vancouver".parse().unwrap()
    }

    fn status_map() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("booked".to_string(), "t".to_string());
        m.insert("cancelled".to_string(), "C".to_string());
        m
    }

    #[test]
    fn maps_full_appointment() {
        let (id, row) = fhir_appointment_to_row(
            &appt("2026-08-10T09:00:00", "2026-08-10T09:15:00"),
            Some("101".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap();
        assert_eq!(id, Some("1".to_string()));
        assert_eq!(row.demographic_no, Some("101".to_string()));
        assert_eq!(row.provider_no, Some("100001".to_string()));
        assert_eq!(row.appointment_date, Some("2026-08-10".to_string()));
        assert_eq!(row.start_time, Some("09:00:00".to_string()));
        assert_eq!(row.end_time, Some("09:15:00".to_string()));
        assert_eq!(row.status, Some("t".to_string()));
        assert_eq!(row.reason, Some("Follow-up".to_string()));
        assert_eq!(row.booking_source, Some("OSCAR".to_string()));
    }

    #[test]
    fn handles_offset_datetime() {
        let (id, row) = fhir_appointment_to_row(
            &appt("2026-08-10T09:00:00-07:00", "2026-08-10T09:15:00-07:00"),
            Some("101".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap();
        assert_eq!(id, Some("1".to_string()));
        assert_eq!(row.appointment_date, Some("2026-08-10".to_string()));
        assert_eq!(row.start_time, Some("09:00:00".to_string()));
    }

    #[test]
    fn spring_forward_gap_dead_letters() {
        let err = fhir_appointment_to_row(
            &appt("2026-03-08T02:30:00", "2026-03-08T03:00:00"),
            Some("101".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap_err();
        assert_eq!(
            err,
            MappingError::NonexistentLocalTime("2026-03-08T02:30:00".to_string())
        );
    }

    #[test]
    fn fall_back_ambiguous_resolves_to_first() {
        let (id, row) = fhir_appointment_to_row(
            &appt("2026-11-01T01:30:00", "2026-11-01T01:45:00"),
            Some("101".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap();
        assert_eq!(id, Some("1".to_string()));
        assert_eq!(row.appointment_date, Some("2026-11-01".to_string()));
        assert_eq!(row.start_time, Some("01:30:00".to_string()));
        assert_eq!(row.end_time, Some("01:45:00".to_string()));
    }

    #[test]
    fn demographic_zero_is_rejected() {
        let err = fhir_appointment_to_row(
            &appt("2026-08-10T09:00:00", "2026-08-10T09:15:00"),
            Some("0".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap_err();
        assert_eq!(err, MappingError::PlaceholderPatient);
    }

    #[test]
    fn unknown_status_dead_letters() {
        let mut a = appt("2026-08-10T09:00:00", "2026-08-10T09:15:00");
        a["status"] = "proposed".into();
        let err = fhir_appointment_to_row(
            &a,
            Some("101".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap_err();
        assert_eq!(
            err,
            MappingError::UnmappedAppointmentStatus("proposed".to_string())
        );
    }

    #[test]
    fn missing_identifier_means_insert() {
        let mut a = appt("2026-08-10T09:00:00", "2026-08-10T09:15:00");
        a.as_object_mut().unwrap().remove("identifier");
        let (id, row) = fhir_appointment_to_row(
            &a,
            Some("101".to_string()),
            Some("100001".to_string()),
            "https://arsmedicatech.com/fhir/sid/oscar-appointment",
            &status_map(),
            &vancouver(),
        )
        .unwrap();
        assert_eq!(id, None);
        assert_eq!(row.appointment_date, Some("2026-08-10".to_string()));
    }
}

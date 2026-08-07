use serde_json::Value;

use super::MappingError;

/// The subset of `demographic` columns that AMT is allowed to write.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct DemographicRow {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_names: Option<String>,
    pub pref_name: Option<String>,
    pub title: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal: Option<String>,
    pub phone: Option<String>,
    pub phone2: Option<String>,
    pub email: Option<String>,
    pub year_of_birth: Option<String>,
    pub month_of_birth: Option<String>,
    pub date_of_birth: Option<String>,
    pub sex: Option<String>,
}

/// Maps a FHIR `Patient` resource to an Oscar `demographic` row.
///
/// Returns the existing `demographic_no` identifier value (if present) and the
/// mapped row. A missing `oscar-demographic-no` identifier means the patient
/// should be INSERTed; a present value means UPDATE. Any value other than the
/// allowlisted fields is ignored.
pub fn fhir_patient_to_row(
    patient: &Value,
    oscar_demographic_system: &str,
) -> Result<(Option<String>, DemographicRow), MappingError> {
    let mut row = DemographicRow::default();

    let mut demographic_no: Option<String> = None;
    if let Some(ids) = patient.get("identifier").and_then(Value::as_array) {
        for id in ids {
            let system = id.get("system").and_then(Value::as_str).unwrap_or("");
            if system == oscar_demographic_system {
                demographic_no = id.get("value").and_then(Value::as_str).map(String::from);
            }
        }
    }

    let names: Vec<Value> = patient
        .get("name")
        .and_then(Value::as_array)
        .map(|a| a.to_vec())
        .unwrap_or_default();
    let name = names
        .iter()
        .find(|n| n.get("family").is_some())
        .or_else(|| names.first());
    if let Some(name) = name {
        let given: Vec<String> = name
            .get("given")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        row.first_name = given.first().cloned();
        if given.len() > 1 {
            row.middle_names = Some(given[1..].join(" "));
        }
        row.last_name = name
            .get("family")
            .and_then(Value::as_str)
            .map(String::from);
        row.pref_name = name
            .get("text")
            .and_then(Value::as_str)
            .map(String::from);
        let prefixes: Vec<String> = name
            .get("prefix")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        if !prefixes.is_empty() {
            row.title = Some(prefixes.join(" "));
        }
    }

    let addresses: Vec<Value> = patient
        .get("address")
        .and_then(Value::as_array)
        .map(|a| a.to_vec())
        .unwrap_or_default();
    if let Some(addr) = addresses
        .iter()
        .find(|a| a.get("use").and_then(Value::as_str) == Some("home"))
        .or_else(|| addresses.first())
    {
        let lines: Vec<String> = addr
            .get("line")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        if !lines.is_empty() {
            row.address = Some(lines.join(", "));
        }
        row.city = addr.get("city").and_then(Value::as_str).map(String::from);
        row.province = addr
            .get("state")
            .and_then(Value::as_str)
            .map(String::from);
        row.postal = addr
            .get("postalCode")
            .and_then(Value::as_str)
            .map(String::from);
    }

    if let Some(telecoms) = patient.get("telecom").and_then(Value::as_array) {
        let mut phone_count = 0;
        for t in telecoms {
            let system = t.get("system").and_then(Value::as_str);
            let value = t.get("value").and_then(Value::as_str);
            match (system, value) {
                (Some("email"), Some(v)) => row.email = Some(v.to_string()),
                (Some("phone"), Some(v)) => {
                    if phone_count == 0 {
                        row.phone = Some(v.to_string());
                        phone_count = 1;
                    } else if phone_count == 1 {
                        row.phone2 = Some(v.to_string());
                        phone_count = 2;
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(dob) = patient.get("birthDate").and_then(Value::as_str) {
        // FHIR birthDate is YYYY, YYYY-MM, or YYYY-MM-DD. For Oscar we only
        // write a complete, valid date; partial or missing dates are omitted.
        let parts: Vec<&str> = dob.split('-').collect();
        if parts.len() == 3
            && parts[0].len() == 4
            && parts[1].len() == 2
            && parts[2].len() == 2
        {
            let year = parts[0];
            let month = parts[1];
            let day = parts[2];
            if year.parse::<u32>().is_ok()
                && month.parse::<u32>().is_ok()
                && day.parse::<u32>().is_ok()
                && year != "0000"
            {
                row.year_of_birth = Some(year.to_string());
                row.month_of_birth = Some(month.to_string());
                row.date_of_birth = Some(day.to_string());
            }
        }
    }

    if let Some(gender) = patient.get("gender").and_then(Value::as_str) {
        row.sex = match gender {
            "male" => Some("M".to_string()),
            "female" => Some("F".to_string()),
            g => return Err(MappingError::UnmappedGender(g.to_string())),
        };
    }

    if demographic_no.as_deref() == Some("0") {
        return Err(MappingError::PlaceholderPatient);
    }

    Ok((demographic_no, row))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patient(dob: &str, gender: &str) -> Value {
        serde_json::json!({
            "resourceType": "Patient",
            "identifier": [
                { "system": "https://arsmedicatech.com/fhir/sid/oscar-demographic", "value": "101" }
            ],
            "name": [
                { "use": "official", "family": "Smith", "given": ["Jane", "A."], "prefix": ["Dr"] }
            ],
            "address": [
                { "use": "home", "line": ["123 Main St"], "city": "Vancouver", "state": "BC", "postalCode": "V6B1A1" }
            ],
            "telecom": [
                { "system": "phone", "value": "555-0100" },
                { "system": "phone", "value": "555-0101" },
                { "system": "email", "value": "jane@example.com" }
            ],
            "birthDate": dob,
            "gender": gender
        })
    }

    #[test]
    fn maps_full_patient() {
        let (id, row) = fhir_patient_to_row(
            &patient("1990-03-15", "female"),
            "https://arsmedicatech.com/fhir/sid/oscar-demographic",
        )
        .unwrap();
        assert_eq!(id, Some("101".to_string()));
        assert_eq!(row.first_name, Some("Jane".to_string()));
        assert_eq!(row.middle_names, Some("A.".to_string()));
        assert_eq!(row.last_name, Some("Smith".to_string()));
        assert_eq!(row.title, Some("Dr".to_string()));
        assert_eq!(row.address, Some("123 Main St".to_string()));
        assert_eq!(row.city, Some("Vancouver".to_string()));
        assert_eq!(row.province, Some("BC".to_string()));
        assert_eq!(row.postal, Some("V6B1A1".to_string()));
        assert_eq!(row.phone, Some("555-0100".to_string()));
        assert_eq!(row.phone2, Some("555-0101".to_string()));
        assert_eq!(row.email, Some("jane@example.com".to_string()));
        assert_eq!(row.year_of_birth, Some("1990".to_string()));
        assert_eq!(row.month_of_birth, Some("03".to_string()));
        assert_eq!(row.date_of_birth, Some("15".to_string()));
        assert_eq!(row.sex, Some("F".to_string()));
    }

    #[test]
    fn other_gender_dead_letters() {
        let err = fhir_patient_to_row(
            &patient("1990-03-15", "other"),
            "https://arsmedicatech.com/fhir/sid/oscar-demographic",
        )
        .unwrap_err();
        assert_eq!(err, MappingError::UnmappedGender("other".to_string()));
    }

    #[test]
    fn partial_dob_is_omitted() {
        let mut p = patient("1990-03-15", "female");
        p["birthDate"] = "1990-03".into();
        let (_, row) = fhir_patient_to_row(
            &p,
            "https://arsmedicatech.com/fhir/sid/oscar-demographic",
        )
        .unwrap();
        assert!(row.year_of_birth.is_none());
        assert!(row.month_of_birth.is_none());
        assert!(row.date_of_birth.is_none());
    }

    #[test]
    fn missing_identifier_means_insert() {
        let mut p = patient("1990-03-15", "female");
        p.as_object_mut().unwrap().remove("identifier");
        let (id, row) = fhir_patient_to_row(
            &p,
            "https://arsmedicatech.com/fhir/sid/oscar-demographic",
        )
        .unwrap();
        assert_eq!(id, None);
        assert_eq!(row.first_name, Some("Jane".to_string()));
    }

    #[test]
    fn demographic_zero_is_rejected() {
        let mut p = patient("1990-03-15", "female");
        p["identifier"] = serde_json::json!([
            { "system": "https://arsmedicatech.com/fhir/sid/oscar-demographic", "value": "0" }
        ]);
        let err = fhir_patient_to_row(
            &p,
            "https://arsmedicatech.com/fhir/sid/oscar-demographic",
        )
        .unwrap_err();
        assert_eq!(err, MappingError::PlaceholderPatient);
    }
}

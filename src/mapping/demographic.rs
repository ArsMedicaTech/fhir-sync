use std::collections::HashMap;

use crate::domain::patient::{AddressKind, AddressUse, DomainAddress, DomainPatient};
use crate::sources::RowChange;

/// Column name -> zero-based index in `RowChange.after`, as resolved from
/// `information_schema.columns` at startup (D3). Never hardcode indexes.
pub type ColumnMap = HashMap<String, usize>;

pub(crate) fn lookup<'a>(change: &'a RowChange, columns: &ColumnMap, name: &str) -> Option<&'a str> {
    let idx = *columns.get(name)?;
    change
        .after
        .get(idx)
        .and_then(|v| v.as_deref())
        .filter(|s| !s.is_empty())
}

fn lookup_any<'a>(change: &'a RowChange, columns: &ColumnMap, names: &[&str]) -> Option<&'a str> {
    names.iter().find_map(|n| lookup(change, columns, n))
}

/// Composes an ISO `YYYY-MM-DD` birth date from Oscar's three DOB columns
/// (F5). Returns `None` unless all three parts are present and valid.
fn compose_birth_date(change: &RowChange, columns: &ColumnMap) -> Option<String> {
    let year = lookup(change, columns, "year_of_birth")?;
    let month = lookup(change, columns, "month_of_birth")?;
    let day = lookup(change, columns, "date_of_birth")?;

    let year: u32 = year.trim().parse().ok()?;
    let month: u32 = month.trim().parse().ok()?;
    let day: u32 = day.trim().parse().ok()?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || year == 0 {
        return None;
    }

    Some(format!("{:04}-{:02}-{:02}", year, month, day))
}

/// Builds a domain address if at least one of the named components is present.
/// Country is intentionally not mapped here (P1); the sink sets "CA".
fn compose_address(
    change: &RowChange,
    columns: &ColumnMap,
    line_names: &[&str],
    city_names: &[&str],
    province_names: &[&str],
    postal_names: &[&str],
    use_: AddressUse,
    kind: AddressKind,
) -> Option<DomainAddress> {
    let line = lookup_any(change, columns, line_names);
    let city = lookup_any(change, columns, city_names);
    let province = lookup_any(change, columns, province_names);
    let postal = lookup_any(change, columns, postal_names);

    if line.is_none() && city.is_none() && province.is_none() && postal.is_none() {
        return None;
    }

    Some(DomainAddress {
        line: line.map(str::to_string),
        city: city.map(str::to_string),
        province: province.map(str::to_string),
        postal: postal.map(str::to_string),
        use_,
        kind,
    })
}

fn compose_addresses(change: &RowChange, columns: &ColumnMap) -> Vec<DomainAddress> {
    let mut addresses = Vec::new();

    if let Some(a) = compose_address(
        change,
        columns,
        &["address"],
        &["city"],
        &["province", "state"],
        &["postal", "postal_code"],
        AddressUse::Home,
        AddressKind::Postal,
    ) {
        addresses.push(a);
    }

    if let Some(a) = compose_address(
        change,
        columns,
        &["residentialAddress"],
        &["residentialCity"],
        &["residentialProvince", "residentialState"],
        &["residentialPostal", "residentialPostalCode"],
        AddressUse::Home,
        AddressKind::Physical,
    ) {
        addresses.push(a);
    }

    addresses
}

/// Maps a normalized `RowChange` from the `demographic` table into a
/// `DomainPatient`. Returns `None` if the row has no `demographic_no`
/// (nothing to identify the patient by).
///
/// Never emits dummy literals for missing fields (F4) — optional fields are
/// `None` unless the corresponding column resolved to a non-empty value.
pub fn row_to_domain_patient(change: &RowChange, columns: &ColumnMap) -> Option<DomainPatient> {
    let demographic_no = lookup(change, columns, "demographic_no")?.to_string();

    Some(DomainPatient {
        demographic_no,
        first_name: lookup(change, columns, "first_name").map(str::to_string),
        last_name: lookup(change, columns, "last_name").map(str::to_string),
        date_of_birth: compose_birth_date(change, columns),
        addresses: compose_addresses(change, columns),
        sex: lookup(change, columns, "sex").map(str::to_string),
        phone: lookup_any(change, columns, &["phone", "phone1"]).map(str::to_string),
        email: lookup(change, columns, "email").map(str::to_string),
        hin: lookup(change, columns, "hin").map(str::to_string),
        patient_status: lookup(change, columns, "patient_status").map(str::to_string),
        merged_to: None,
    })
}

/// Maps a `demographic_merged` row into a merge-loser `DomainPatient`.
/// Returns `None` when the merge is marked deleted (unmerged) or the row
/// does not identify the loser record.
pub fn row_to_merged_patient(change: &RowChange, columns: &ColumnMap) -> Option<DomainPatient> {
    let demographic_no = lookup(change, columns, "demographic_no")?.to_string();
    let merged_to = lookup(change, columns, "merged_to").map(str::to_string)?;

    let deleted = lookup(change, columns, "deleted")
        .map(|s| s.trim())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1);
    if deleted != 0 {
        return None;
    }

    Some(DomainPatient {
        demographic_no,
        first_name: None,
        last_name: None,
        date_of_birth: None,
        addresses: Vec::new(),
        sex: None,
        phone: None,
        email: None,
        hin: None,
        patient_status: None,
        merged_to: Some(merged_to),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::{RowOp, SourcePosition};

    fn columns() -> ColumnMap {
        [
            ("demographic_no", 0),
            ("first_name", 1),
            ("last_name", 2),
            ("year_of_birth", 3),
            ("month_of_birth", 4),
            ("date_of_birth", 5),
            ("sex", 6),
            ("email", 7),
            ("phone", 8),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    }

    fn change(after: Vec<Option<&str>>) -> RowChange {
        RowChange {
            schema: "oscar".to_string(),
            table: "demographic".to_string(),
            op: RowOp::Insert,
            after: after.into_iter().map(|v| v.map(str::to_string)).collect(),
            position: SourcePosition::FilePos {
                file: "mysql-bin.000001".to_string(),
                pos: 4,
            },
        }
    }

    #[test]
    fn composes_full_birth_date() {
        let cols = columns();
        let row = change(vec![
            Some("123"),
            Some("Alice"),
            Some("Smith"),
            Some("1990"),
            Some("3"),
            Some("5"),
            Some("F"),
            Some("alice@example.com"),
            Some("555-1234"),
        ]);

        let patient = row_to_domain_patient(&row, &cols).unwrap();
        assert_eq!(patient.demographic_no, "123");
        assert_eq!(patient.date_of_birth, Some("1990-03-05".to_string()));
        assert_eq!(patient.email, Some("alice@example.com".to_string()));
        assert_eq!(patient.phone, Some("555-1234".to_string()));
        assert!(patient.addresses.is_empty());
        assert_eq!(patient.patient_status, None);
    }

    #[test]
    fn missing_dob_part_omits_birth_date() {
        let cols = columns();
        let row = change(vec![
            Some("123"),
            Some("Alice"),
            Some("Smith"),
            Some("1990"),
            None,
            Some("5"),
            Some("F"),
            None,
            None,
        ]);

        let patient = row_to_domain_patient(&row, &cols).unwrap();
        assert_eq!(patient.date_of_birth, None);
        assert_eq!(patient.email, None);
        assert_eq!(patient.phone, None);
    }

    #[test]
    fn missing_demographic_no_yields_none() {
        let cols = columns();
        let row = change(vec![None, Some("Alice"), None, None, None, None, None, None, None]);
        assert!(row_to_domain_patient(&row, &cols).is_none());
    }

    fn address_columns() -> ColumnMap {
        [
            ("demographic_no", 0),
            ("first_name", 1),
            ("last_name", 2),
            ("year_of_birth", 3),
            ("month_of_birth", 4),
            ("date_of_birth", 5),
            ("sex", 6),
            ("email", 7),
            ("phone", 8),
            ("address", 9),
            ("city", 10),
            ("province", 11),
            ("postal", 12),
            ("residentialAddress", 13),
            ("residentialCity", 14),
            ("residentialProvince", 15),
            ("residentialPostal", 16),
            ("patient_status", 17),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    }

    #[test]
    fn composes_two_addresses() {
        let cols = address_columns();
        let row = change(vec![
            Some("101"),
            Some("Bob"),
            Some("Whitfield"),
            Some("1968"),
            Some("7"),
            Some("14"),
            Some("M"),
            None,
            None,
            Some("123 Postal St"),
            Some("Vancouver"),
            Some("BC"),
            Some("V6C1V5"),
            Some("456 Physical Ave"),
            Some("Burnaby"),
            Some("BC"),
            Some("V5A2B3"),
            Some("AC"),
        ]);

        let patient = row_to_domain_patient(&row, &cols).unwrap();
        assert_eq!(patient.addresses.len(), 2);
        assert_eq!(patient.addresses[0].kind, AddressKind::Postal);
        assert_eq!(patient.addresses[1].kind, AddressKind::Physical);
        assert_eq!(patient.patient_status, Some("AC".to_string()));
    }

    #[test]
    fn null_address_line_is_allowed() {
        let cols = address_columns();
        let row = change(vec![
            Some("102"), Some("Kayode"), Some("Adeyemi"), Some("1991"), Some("3"), Some("22"),
            Some("O"), None, None, None, Some("Vancouver"), Some("BC"), Some("V5K0A1"),
            None, None, None, None, None,
        ]);

        let patient = row_to_domain_patient(&row, &cols).unwrap();
        assert_eq!(patient.addresses.len(), 1);
        assert_eq!(patient.addresses[0].line, None);
        assert_eq!(patient.addresses[0].city, Some("Vancouver".to_string()));
    }

    fn merge_columns() -> ColumnMap {
        [("demographic_no", 0), ("merged_to", 1), ("deleted", 2)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }

    #[test]
    fn merge_row_emits_link() {
        let cols = merge_columns();
        let row = RowChange {
            schema: "oscar".to_string(),
            table: "demographic_merged".to_string(),
            op: RowOp::Insert,
            after: vec![Some("106".to_string()), Some("101".to_string()), Some("0".to_string())],
            position: SourcePosition::FilePos { file: "bin".to_string(), pos: 1 },
        };

        let patient = row_to_merged_patient(&row, &cols).unwrap();
        assert_eq!(patient.demographic_no, "106");
        assert_eq!(patient.merged_to, Some("101".to_string()));
    }

    #[test]
    fn merge_row_deleted_is_skipped() {
        let cols = merge_columns();
        let row = RowChange {
            schema: "oscar".to_string(),
            table: "demographic_merged".to_string(),
            op: RowOp::Insert,
            after: vec![Some("106".to_string()), Some("101".to_string()), Some("1".to_string())],
            position: SourcePosition::FilePos { file: "bin".to_string(), pos: 1 },
        };

        assert!(row_to_merged_patient(&row, &cols).is_none());
    }
}

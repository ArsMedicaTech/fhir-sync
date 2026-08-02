use std::collections::HashMap;

use crate::domain::patient::DomainPatient;
use crate::sources::RowChange;

/// Column name -> zero-based index in `RowChange.after`, as resolved from
/// `information_schema.columns` at startup (D3). Never hardcode indexes.
pub type ColumnMap = HashMap<String, usize>;

fn lookup<'a>(change: &'a RowChange, columns: &ColumnMap, name: &str) -> Option<&'a str> {
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

fn compose_location(change: &RowChange, columns: &ColumnMap) -> Option<(String, String, String, String)> {
    let city = lookup_any(change, columns, &["city"])?;
    let province = lookup_any(change, columns, &["province", "state"])?;
    let country = lookup_any(change, columns, &["country"])?;
    let postal = lookup_any(change, columns, &["postal", "postal_code"])?;

    Some((
        city.to_string(),
        province.to_string(),
        country.to_string(),
        postal.to_string(),
    ))
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
        location: compose_location(change, columns),
        sex: lookup(change, columns, "sex").map(str::to_string),
        phone: lookup_any(change, columns, &["phone", "phone1"]).map(str::to_string),
        email: lookup(change, columns, "email").map(str::to_string),
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
        assert_eq!(patient.location, None);
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
}

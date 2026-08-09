use std::collections::HashMap;

use crate::domain::practitioner::DomainPractitioner;
use crate::sources::RowChange;
use crate::mapping::syncable_provider;
use tracing::warn;

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

/// Maps one `provider` row to a `DomainPractitioner`.
///
/// Returns `None` unless `provider_no` is present and non-empty.
pub fn row_to_domain_practitioner(change: &RowChange, columns: &ColumnMap) -> Option<DomainPractitioner> {
    let provider_no = lookup(change, columns, "provider_no")?;

    // D3: the synthetic system actor must never become a Practitioner.
    if syncable_provider(Some(provider_no)).is_none() {
        return None;
    }

    // D3: `ohip_no` is a data-migration artifact in BC builds; warn but do not map.
    if let Some(_ohip) = lookup_any(change, columns, &["ohip_no", "ohip"]) {
        warn!(
            "provider mapping: ohip_no present for provider_no={provider_no}; this is a data-migration artifact and is intentionally not mapped (D-1)"
        );
    }

    Some(DomainPractitioner {
        provider_no: provider_no.to_string(),
        billing_no: lookup_any(change, columns, &["billing_no", "billingno"]).map(str::to_string),
        practitioner_no: lookup_any(change, columns, &["practitioner_no", "practitionerNo"]).map(str::to_string),
        practitioner_no_type: lookup_any(change, columns, &["practitioner_no_type", "practitionerNoType"]).map(str::to_string),
        ohip_no: lookup_any(change, columns, &["ohip_no", "ohip"]).map(str::to_string),
        title: lookup_any(change, columns, &["title"]).map(str::to_string),
        first_name: lookup_any(change, columns, &["first_name", "firstname"]).map(str::to_string),
        last_name: lookup_any(change, columns, &["last_name", "lastname"]).map(str::to_string),
        sex: lookup_any(change, columns, &["sex"]).map(str::to_string),
        date_of_birth: lookup_any(change, columns, &["dob", "date_of_birth", "birth_date"]).map(str::to_string),
        phone: lookup_any(change, columns, &["phone"]).map(str::to_string),
        email: lookup_any(change, columns, &["email"]).map(str::to_string),
        work_phone: lookup_any(change, columns, &["work_phone", "workphone"]).map(str::to_string),
        address: lookup_any(change, columns, &["address"]).map(str::to_string),
        status: lookup_any(change, columns, &["status"]).map(str::to_string),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(values: Vec<Option<&str>>) -> RowChange {
        RowChange {
            schema: "oscar".to_string(),
            table: "provider".to_string(),
            op: crate::sources::RowOp::Insert,
            after: values.into_iter().map(|v| v.map(str::to_string)).collect(),
            position: crate::sources::SourcePosition::FilePos {
                file: String::new(),
                pos: 0,
            },
        }
    }

    fn columns(names: &[&str]) -> ColumnMap {
        names.iter().enumerate().map(|(i, n)| (n.to_string(), i)).collect()
    }

    #[test]
    fn maps_full_provider() {
        let cols = columns(&[
            "provider_no",
            "billing_no",
            "practitionerNo",
            "practitionerNoType",
            "title",
            "first_name",
            "last_name",
            "sex",
            "dob",
            "phone",
            "email",
            "work_phone",
            "address",
            "status",
        ]);
        let p = row_to_domain_practitioner(
            &change(vec![
                Some("1001"),
                Some("B1001"),
                Some("PN-1"),
                Some("College"),
                Some("Dr"),
                Some("Alice"),
                Some("Ng"),
                Some("F"),
                Some("1980-04-15"),
                Some("604-555-0100"),
                Some("alice@example.com"),
                Some("604-555-0200"),
                Some("123 Main St"),
                Some("1"),
            ]),
            &cols,
        )
        .unwrap();

        assert_eq!(p.provider_no, "1001");
        assert_eq!(p.billing_no, Some("B1001".to_string()));
        assert_eq!(p.practitioner_no, Some("PN-1".to_string()));
        assert_eq!(p.practitioner_no_type, Some("College".to_string()));
        assert_eq!(p.title, Some("Dr".to_string()));
        assert_eq!(p.first_name, Some("Alice".to_string()));
        assert_eq!(p.last_name, Some("Ng".to_string()));
        assert_eq!(p.sex, Some("F".to_string()));
        assert_eq!(p.date_of_birth, Some("1980-04-15".to_string()));
        assert_eq!(p.phone, Some("604-555-0100".to_string()));
        assert_eq!(p.email, Some("alice@example.com".to_string()));
        assert_eq!(p.work_phone, Some("604-555-0200".to_string()));
        assert_eq!(p.address, Some("123 Main St".to_string()));
        assert_eq!(p.status, Some("1".to_string()));
    }

    #[test]
    fn missing_provider_no_is_none() {
        let cols = columns(&["provider_no", "first_name"]);
        assert!(row_to_domain_practitioner(&change(vec![None, Some("Alice")]), &cols).is_none());
    }
}

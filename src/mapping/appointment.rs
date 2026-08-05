use std::collections::HashMap;

use tracing::info;

use crate::domain::appointment::DomainAppointment;
use crate::sources::RowChange;
use crate::mapping::syncable_provider;

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

/// Maps one `appointment` row to a `DomainAppointment`.
///
/// Returns `None` if the row has no `appointment_no` or is a placeholder
/// appointment (`demographic_no` is `0` or absent), logging the reason at
/// `info!` (D7).
pub fn row_to_domain_appointment(change: &RowChange, columns: &ColumnMap) -> Option<DomainAppointment> {
    let appointment_no = lookup(change, columns, "appointment_no")?.to_string();

    let demographic_no = lookup(change, columns, "demographic_no").map(str::to_string);

    // D7: placeholder appointments (blocked time) have demographic_no = 0.
    if demographic_no.as_deref() == Some("0") {
        info!("appointment mapping: skipping placeholder appointment_no={appointment_no} (demographic_no=0)");
        return None;
    }

    // D3: the synthetic system actor must not appear as a participant.
    let provider_no = syncable_provider(lookup(change, columns, "provider_no"));

    Some(DomainAppointment {
        appointment_no,
        demographic_no,
        provider_no,
        appointment_date: lookup(change, columns, "appointment_date").map(str::to_string),
        start_time: lookup(change, columns, "start_time").map(str::to_string),
        end_time: lookup(change, columns, "end_time").map(str::to_string),
        status: lookup(change, columns, "status").map(str::to_string),
        reason: lookup(change, columns, "reason").map(str::to_string),
        notes: lookup(change, columns, "notes").map(str::to_string),
        remarks: lookup(change, columns, "remarks").map(str::to_string),
        urgency: lookup(change, columns, "urgency").map(str::to_string),
        createdatetime: lookup(change, columns, "createdatetime").map(str::to_string),
        location: lookup(change, columns, "location").map(str::to_string),
        booking_source: lookup_any(change, columns, &["bookingSource", "booking_source"]).map(str::to_string),
        type_: lookup(change, columns, "type").map(str::to_string),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(values: Vec<Option<&str>>) -> RowChange {
        RowChange {
            schema: "oscar".to_string(),
            table: "appointment".to_string(),
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
    fn maps_full_appointment() {
        let cols = columns(&[
            "appointment_no",
            "demographic_no",
            "provider_no",
            "appointment_date",
            "start_time",
            "end_time",
            "status",
            "reason",
            "notes",
            "remarks",
            "urgency",
            "createdatetime",
            "location",
            "bookingSource",
            "type",
        ]);
        let a = row_to_domain_appointment(
            &change(vec![
                Some("1"),
                Some("101"),
                Some("100001"),
                Some("2026-08-10"),
                Some("09:00:00"),
                Some("09:15:00"),
                Some("t"),
                Some("Follow-up"),
                Some("note one"),
                Some("note two"),
                Some("3"),
                Some("2026-08-09 16:30:00"),
                Some("Room A"),
                Some("online"),
                Some("Regular"),
            ]),
            &cols,
        )
        .unwrap();

        assert_eq!(a.appointment_no, "1");
        assert_eq!(a.demographic_no, Some("101".to_string()));
        assert_eq!(a.provider_no, Some("100001".to_string()));
        assert_eq!(a.appointment_date, Some("2026-08-10".to_string()));
        assert_eq!(a.start_time, Some("09:00:00".to_string()));
        assert_eq!(a.end_time, Some("09:15:00".to_string()));
        assert_eq!(a.status, Some("t".to_string()));
        assert_eq!(a.reason, Some("Follow-up".to_string()));
        assert_eq!(a.notes, Some("note one".to_string()));
        assert_eq!(a.remarks, Some("note two".to_string()));
        assert_eq!(a.urgency, Some("3".to_string()));
        assert_eq!(a.createdatetime, Some("2026-08-09 16:30:00".to_string()));
        assert_eq!(a.location, Some("Room A".to_string()));
        assert_eq!(a.booking_source, Some("online".to_string()));
        assert_eq!(a.type_, Some("Regular".to_string()));
    }

    #[test]
    fn placeholder_demographic_zero_is_none() {
        let cols = columns(&["appointment_no", "demographic_no", "status"]);
        assert!(row_to_domain_appointment(&change(vec![Some("9"), Some("0"), Some("t")]), &cols).is_none());
    }

    #[test]
    fn system_actor_provider_is_omitted() {
        let cols = columns(&["appointment_no", "demographic_no", "provider_no"]);
        let a = row_to_domain_appointment(&change(vec![Some("12"), Some("101"), Some("-1")]), &cols).unwrap();
        assert_eq!(a.provider_no, None);
    }

    #[test]
    fn missing_appointment_no_is_none() {
        let cols = columns(&["appointment_no", "demographic_no"]);
        assert!(row_to_domain_appointment(&change(vec![None, Some("101")]), &cols).is_none());
    }
}

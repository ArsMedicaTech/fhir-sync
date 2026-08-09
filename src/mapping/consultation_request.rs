use std::collections::HashMap;

use tracing::info;

use crate::domain::service_request::DomainServiceRequest;
use crate::mapping::syncable_provider;
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

/// Maps one `consultationRequests` row to a `DomainServiceRequest`.
///
/// This is the reverse of `writeback/mappers/service_request.rs`: AMT
/// originates the `ServiceRequest`, but Oscar's own consult workflow (a
/// human changing the request's Status in Oscar's UI) needs to resync back
/// into AMT so the corresponding task reflects progress/completion there.
///
/// Deliberately does NOT try to correlate via `source` or
/// `consultationRequestExt`: both were found empirically to be unreliable --
/// Oscar's own "Update Consultation Request" save wipes `source` back to
/// blank and replaces that request's extension rows wholesale (its classic
/// JSP form has no `source` field and does not preserve AMT's tracking ext
/// rows on save), even though it edits the row in place. `requestId` is the
/// one thing that stays stable across such an edit, and it's exactly the
/// value AMT used as the FHIR identifier when it originally created the
/// resource (see `writeback/mappers/service_request.rs`), so it's used
/// directly here rather than resolved via a live correlation query.
///
/// Trade-off: a request that was always Oscar-native (never touched by
/// AMT) will also get synced, producing a new orphan `ServiceRequest` in
/// AMT rather than being filtered out -- there's no reliable signal left to
/// distinguish that case once Oscar has scrubbed `source`. This matches the
/// precedent already set by the `demographic` table sync, which does not
/// filter by origin either.
///
/// Returns `None` if the row has no `requestId` or `demographicNo`.
pub fn row_to_domain_service_request(change: &RowChange, columns: &ColumnMap) -> Option<DomainServiceRequest> {
    let request_id = match lookup(change, columns, "requestId") {
        Some(id) => id.to_string(),
        None => {
            info!("consultationRequests mapping: skipping row with no requestId");
            return None;
        }
    };

    let demographic_no = match lookup(change, columns, "demographicNo") {
        Some(d) => d.to_string(),
        None => {
            info!("consultationRequests mapping: skipping requestId={request_id} (no demographicNo)");
            return None;
        }
    };

    let provider_no = syncable_provider(lookup(change, columns, "providerNo"));

    Some(DomainServiceRequest {
        request_id,
        demographic_no: Some(demographic_no),
        provider_no,
        reason: lookup(change, columns, "reason").map(str::to_string),
        clinical_info: lookup(change, columns, "clinicalInfo").map(str::to_string),
        referal_date: lookup(change, columns, "referalDate").map(str::to_string),
        urgency: lookup(change, columns, "urgency").map(str::to_string),
        status: lookup(change, columns, "status").map(str::to_string),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(values: Vec<Option<&str>>) -> RowChange {
        RowChange {
            schema: "oscar".to_string(),
            table: "consultationRequests".to_string(),
            op: crate::sources::RowOp::Update,
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
    fn maps_request_using_its_own_requestid() {
        let cols = columns(&[
            "requestId",
            "demographicNo",
            "providerNo",
            "reason",
            "clinicalInfo",
            "referalDate",
            "urgency",
            "status",
        ]);
        let r = row_to_domain_service_request(
            &change(vec![
                Some("13"),
                Some("118"),
                Some("999998"),
                Some("Radiology referral"),
                Some("Chest pain"),
                Some("2026-08-09"),
                Some("2"),
                Some("4"),
            ]),
            &cols,
        )
        .unwrap();

        assert_eq!(r.request_id, "13");
        assert_eq!(r.demographic_no, Some("118".to_string()));
        assert_eq!(r.provider_no, Some("999998".to_string()));
        assert_eq!(r.status, Some("4".to_string()));
    }

    #[test]
    fn survives_blank_source_and_missing_ext_rows() {
        // No "source" column at all in this change -- mirrors Oscar wiping
        // it on save. Mapping must still succeed via requestId alone.
        let cols = columns(&["requestId", "demographicNo", "status"]);
        let r = row_to_domain_service_request(&change(vec![Some("13"), Some("118"), Some("4")]), &cols).unwrap();
        assert_eq!(r.request_id, "13");
        assert_eq!(r.status, Some("4".to_string()));
    }

    #[test]
    fn missing_request_id_is_skipped() {
        let cols = columns(&["requestId", "demographicNo"]);
        assert!(row_to_domain_service_request(&change(vec![None, Some("118")]), &cols).is_none());
    }

    #[test]
    fn missing_demographic_no_is_skipped() {
        let cols = columns(&["requestId", "demographicNo"]);
        assert!(row_to_domain_service_request(&change(vec![Some("13"), None]), &cols).is_none());
    }

    #[test]
    fn system_actor_provider_is_omitted() {
        let cols = columns(&["requestId", "demographicNo", "providerNo"]);
        let r = row_to_domain_service_request(&change(vec![Some("13"), Some("118"), Some("-1")]), &cols).unwrap();
        assert_eq!(r.provider_no, None);
    }
}

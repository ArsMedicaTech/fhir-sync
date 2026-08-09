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
/// Only `source = 'AMT-eReferral'` rows are resynced — requests originated
/// natively in Oscar (no AMT `ServiceRequest` to update) are skipped, since
/// there's nothing in AMT for them to correlate against.
///
/// Returns `None` if the row has no `requestId`, no `demographicNo`, or was
/// not originated by AMT.
pub fn row_to_domain_service_request(change: &RowChange, columns: &ColumnMap) -> Option<DomainServiceRequest> {
    let request_id = lookup(change, columns, "requestId")?.to_string();

    let source = lookup(change, columns, "source");
    if source != Some("AMT-eReferral") {
        info!(
            "consultationRequests mapping: skipping requestId={request_id} (source={source:?}, not AMT-originated)"
        );
        return None;
    }

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
    fn maps_amt_originated_request() {
        let cols = columns(&[
            "requestId",
            "demographicNo",
            "providerNo",
            "reason",
            "clinicalInfo",
            "referalDate",
            "urgency",
            "status",
            "source",
        ]);
        let r = row_to_domain_service_request(
            &change(vec![
                Some("7"),
                Some("116"),
                Some("999998"),
                Some("Cardiology referral"),
                Some("Chest pain"),
                Some("2026-08-08"),
                Some("2"),
                Some("4"),
                Some("AMT-eReferral"),
            ]),
            &cols,
        )
        .unwrap();

        assert_eq!(r.request_id, "7");
        assert_eq!(r.demographic_no, Some("116".to_string()));
        assert_eq!(r.provider_no, Some("999998".to_string()));
        assert_eq!(r.status, Some("4".to_string()));
    }

    #[test]
    fn non_amt_source_is_skipped() {
        let cols = columns(&["requestId", "demographicNo", "source"]);
        assert!(row_to_domain_service_request(
            &change(vec![Some("9"), Some("116"), Some("Oscar-native")]),
            &cols
        )
        .is_none());
    }

    #[test]
    fn missing_source_is_skipped() {
        let cols = columns(&["requestId", "demographicNo"]);
        assert!(row_to_domain_service_request(&change(vec![Some("9"), Some("116")]), &cols).is_none());
    }

    #[test]
    fn missing_demographic_no_is_skipped() {
        let cols = columns(&["requestId", "demographicNo", "source"]);
        assert!(row_to_domain_service_request(
            &change(vec![Some("9"), None, Some("AMT-eReferral")]),
            &cols
        )
        .is_none());
    }

    #[test]
    fn system_actor_provider_is_omitted() {
        let cols = columns(&["requestId", "demographicNo", "providerNo", "source"]);
        let r = row_to_domain_service_request(
            &change(vec![Some("7"), Some("116"), Some("-1"), Some("AMT-eReferral")]),
            &cols,
        )
        .unwrap();
        assert_eq!(r.provider_no, None);
    }
}

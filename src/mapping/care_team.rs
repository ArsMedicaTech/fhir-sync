use tracing::info;

use crate::config::OscarConfig;
use crate::domain::care_team::DomainCareTeam;
use crate::mapping::demographic::{lookup, ColumnMap};
use crate::mapping::syncable_provider;
use crate::sources::RowChange;

/// Maps a `demographic` row change into a `DomainCareTeam` for the patient's
/// most-responsible provider (MRP), applying the D3 missing/unusable-MRP rules.
pub fn row_to_domain_care_team(
    change: &RowChange,
    cols: &ColumnMap,
    oscar_cfg: &OscarConfig,
) -> Option<DomainCareTeam> {
    let demographic_no = lookup(change, cols, "demographic_no")?.to_string();

    let provider_no = match syncable_provider(lookup(change, cols, "provider_no")) {
        Some(p) => p,
        None => oscar_cfg.default_mrp_provider_no.clone()?,
    };

    if provider_no.trim().is_empty() || provider_no == "-1" {
        info!(
            "care_team: demographic_no={demographic_no} has no usable MRP and no fallback; skipping CareTeam"
        );
        return None;
    }

    Some(DomainCareTeam {
        demographic_no,
        provider_no,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::{RowChange, RowOp, SourcePosition};

    fn columns() -> ColumnMap {
        [("demographic_no", 0), ("provider_no", 1)]
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

    fn cfg(fallback: Option<&str>) -> OscarConfig {
        OscarConfig {
            timezone: Some("America/Vancouver".to_string()),
            region: None,
            appointment_status_map: OscarConfig::default().appointment_status_map,
            default_mrp_provider_no: fallback.map(str::to_string),
            care_team_enabled: true,
            consult_response_status_map: OscarConfig::default().consult_response_status_map,
        }
    }

    #[test]
    fn valid_mrp_maps_to_participant() {
        let cols = columns();
        let row = change(vec![Some("123"), Some("999998")]);
        let ct = row_to_domain_care_team(&row, &cols, &cfg(None)).unwrap();
        assert_eq!(ct.demographic_no, "123");
        assert_eq!(ct.provider_no, "999998");
    }

    #[test]
    fn opaque_provider_no_preserved() {
        let cols = columns();
        let row = change(vec![Some("123"), Some("ABC12")]);
        let ct = row_to_domain_care_team(&row, &cols, &cfg(None)).unwrap();
        assert_eq!(ct.provider_no, "ABC12");
    }

    #[test]
    fn minus_one_without_fallback_yields_none() {
        let cols = columns();
        let row = change(vec![Some("123"), Some("-1")]);
        assert!(row_to_domain_care_team(&row, &cols, &cfg(None)).is_none());
    }

    #[test]
    fn null_provider_no_without_fallback_yields_none() {
        let cols = columns();
        let row = change(vec![Some("123"), None]);
        assert!(row_to_domain_care_team(&row, &cols, &cfg(None)).is_none());
    }

    #[test]
    fn empty_provider_no_without_fallback_yields_none() {
        let cols = columns();
        let row = change(vec![Some("123"), Some("")]);
        assert!(row_to_domain_care_team(&row, &cols, &cfg(None)).is_none());
    }

    #[test]
    fn minus_one_with_fallback_uses_fallback() {
        let cols = columns();
        let row = change(vec![Some("123"), Some("-1")]);
        let ct = row_to_domain_care_team(&row, &cols, &cfg(Some("999998"))).unwrap();
        assert_eq!(ct.provider_no, "999998");
    }

    #[test]
    fn null_with_fallback_uses_fallback() {
        let cols = columns();
        let row = change(vec![Some("123"), None]);
        let ct = row_to_domain_care_team(&row, &cols, &cfg(Some("999998"))).unwrap();
        assert_eq!(ct.provider_no, "999998");
    }

    #[test]
    fn empty_with_fallback_uses_fallback() {
        let cols = columns();
        let row = change(vec![Some("123"), Some("")]);
        let ct = row_to_domain_care_team(&row, &cols, &cfg(Some("999998"))).unwrap();
        assert_eq!(ct.provider_no, "999998");
    }
}

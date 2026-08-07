//! Classification of HAPI resources for the write-back path.
//!
//! The Oscar → FHIR CDC stamps `meta.source` on every resource it creates.
//! A write-back resource with that source is an echo and must be ignored.
//!
//! The write-back path itself also writes identifiers back to HAPI and stamps
//! those updates with `WRITE_BACK_SOURCE` so the next poll cycle does not try
//! to re-sink them.

use serde_json::Value;

use crate::sink::fhir::META_SOURCE;

/// Source value stamped on HAPI resources when the write-back path updates
/// them with newly-generated Oscar identifiers.
pub const WRITE_BACK_SOURCE: &str = "urn:arsmedicatech:fhir-sync:writeback";

/// Returns `true` when `meta.source` identifies this resource as originating
/// from the Oscar → FHIR CDC, **or when `meta.source` is missing/absent**.
/// Such resources must never be written back; an unknown source is treated
/// as Oscar-origin to avoid corrupting pre-existing records.
pub fn is_oscar_origin(resource: &Value) -> bool {
    resource
        .get("meta")
        .and_then(|m| m.get("source"))
        .and_then(Value::as_str)
        .map(|s| s == META_SOURCE)
        .unwrap_or(true)
}

/// Returns `true` when `meta.source` identifies this resource as a
/// write-back path HAPI update.  These must also be skipped by the poller.
pub fn is_writeback_source(resource: &Value) -> bool {
    resource
        .get("meta")
        .and_then(|m| m.get("source"))
        .and_then(Value::as_str)
        .map(|s| s == WRITE_BACK_SOURCE)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::sink::fhir::META_SOURCE;

    use super::{is_oscar_origin, is_writeback_source, WRITE_BACK_SOURCE};

    #[test]
    fn oscar_origin_is_recognized() {
        let resource = json!({"meta": {"source": META_SOURCE}});
        assert!(is_oscar_origin(&resource));
        assert!(!is_writeback_source(&resource));
    }

    #[test]
    fn writeback_source_is_recognized() {
        let resource = json!({"meta": {"source": WRITE_BACK_SOURCE}});
        assert!(!is_oscar_origin(&resource));
        assert!(is_writeback_source(&resource));
    }

    #[test]
    fn unknown_amt_source_is_not_oscar_or_writeback() {
        let resource = json!({"meta": {"source": "https://example.org/fhir"}});
        assert!(!is_oscar_origin(&resource));
        assert!(!is_writeback_source(&resource));
    }

    #[test]
    fn missing_source_fails_closed_as_oscar_origin() {
        let resource = json!({"id": "no-source"});
        assert!(
            is_oscar_origin(&resource),
            "missing source must be treated as Oscar-origin"
        );
        assert!(!is_writeback_source(&resource));
    }

    #[test]
    fn null_source_fails_closed_as_oscar_origin() {
        let resource = json!({"meta": {"source": null}});
        assert!(
            is_oscar_origin(&resource),
            "null source must be treated as Oscar-origin"
        );
    }
}

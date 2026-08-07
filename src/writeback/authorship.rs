//! Classification of HAPI resources for the write-back path.
//!
//! The Oscar → FHIR CDC stamps `meta.source` on every resource it creates.
//! A write-back resource with that source is an echo and must be ignored.

use serde_json::Value;

use crate::sink::fhir::META_SOURCE;

/// Returns `true` when `meta.source` identifies this resource as originating
/// from the Oscar → FHIR CDC.  Such resources must never be written back.
pub fn is_oscar_origin(resource: &Value) -> bool {
    resource
        .get("meta")
        .and_then(|m| m.get("source"))
        .and_then(Value::as_str)
        .map(|s| s == META_SOURCE)
        .unwrap_or(false)
}

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
/// from the Oscar → FHIR CDC.  Such resources must never be written back.
pub fn is_oscar_origin(resource: &Value) -> bool {
    resource
        .get("meta")
        .and_then(|m| m.get("source"))
        .and_then(Value::as_str)
        .map(|s| s == META_SOURCE)
        .unwrap_or(false)
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

pub mod mariadb_cdc;

/// Identifies a table referenced by a binlog `TableMapEvent`.
/// Table ids are per-connection-session and must be resolved dynamically
/// (F2) rather than hardcoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRef {
    pub schema: String,
    pub table: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowOp {
    Insert,
    Update,
    Delete,
}

/// Resume position for a source, transport-specific.
#[derive(Debug, Clone)]
pub enum SourcePosition {
    FilePos { file: String, pos: u32 },
    MariaGtid(String),
}

/// Transport-agnostic row change, normalized from whatever the underlying
/// CDC connector emits. Column extraction / DOB composition / gender mapping
/// live in `crate::mapping`, not here (D2).
#[derive(Debug, Clone)]
pub struct RowChange {
    pub schema: String,
    pub table: String,
    pub op: RowOp,
    pub after: Vec<Option<String>>,
    pub position: SourcePosition,
}

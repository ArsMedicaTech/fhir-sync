use mysql_binlog_connector_rust::column::column_value::ColumnValue;

pub trait ColumnValueExt {
    fn as_str(&self) -> Option<&str>;
}

impl ColumnValueExt for ColumnValue {
    fn as_str(&self) -> Option<&str> {
        match self {
            ColumnValue::String(bytes) => std::str::from_utf8(bytes).ok(),
            ColumnValue::Decimal(s) => Some(s),
            ColumnValue::Date(s) => Some(s),
            ColumnValue::DateTime(s) => Some(s),
            ColumnValue::Time(s) => Some(s),
            _ => None,
        }
    }
}

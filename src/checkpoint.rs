//! Checkpoint persistence for the MariaDB binlog source (D6).
//!
//! File+position, not GTID: `mysql-binlog-connector-rust`
//! has no representation for MariaDB's GTID format, and GTID-mode routes
//! through the same `SHOW MASTER STATUS` column-4 read that panics against
//! MariaDB (E5). File+position survives process restart; it does not survive
//! source failover, which is out of scope for a single-instance Oscar deployment.

use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Checkpoint {
    pub binlog_filename: String,
    pub binlog_position: u32,
}

/// Loads the checkpoint file if present and non-empty. Returns `None` on any
/// read/parse failure or missing file — the caller falls back to a cold
/// start (`SHOW MASTER STATUS`), never to a bad position.
pub fn load(path: &str) -> Option<Checkpoint> {
    let contents = fs::read_to_string(path).ok()?;
    let cp: Checkpoint = serde_json::from_str(&contents).ok()?;
    if cp.binlog_filename.is_empty() {
        return None;
    }
    Some(cp)
}

/// Atomically persists the checkpoint (write to a temp file, then rename)
/// so a crash mid-write never leaves a corrupt/partial checkpoint on disk.
pub fn save(path: &str, cp: &Checkpoint) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).ok();
    }

    let tmp_path = format!("{path}.tmp");
    let json = serde_json::to_string(cp).context("serializing checkpoint")?;

    {
        let mut file =
            fs::File::create(&tmp_path).with_context(|| format!("creating {tmp_path}"))?;
        file.write_all(json.as_bytes())
            .context("writing checkpoint contents")?;
        file.sync_all().ok();
    }

    fs::rename(&tmp_path, path).with_context(|| format!("renaming {tmp_path} -> {path}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_path(name: &str) -> String {
        std::env::temp_dir()
            .join(format!("fhir-sync-checkpoint-test-{}-{name}", std::process::id()))
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn round_trips_through_save_and_load() {
        let path = tmp_path("roundtrip");
        let cp = Checkpoint {
            binlog_filename: "mysql-bin.000002".to_string(),
            binlog_position: 4,
        };

        save(&path, &cp).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(loaded, cp);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn missing_file_yields_none() {
        let path = tmp_path("missing");
        assert!(load(&path).is_none());
    }

    #[test]
    fn empty_filename_yields_none() {
        let path = tmp_path("empty-filename");
        let cp = Checkpoint {
            binlog_filename: String::new(),
            binlog_position: 0,
        };
        save(&path, &cp).unwrap();
        assert!(load(&path).is_none());

        std::fs::remove_file(&path).ok();
    }
}

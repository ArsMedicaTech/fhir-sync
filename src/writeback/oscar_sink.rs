//! Oscar MariaDB sink for the AMT → Oscar write-back path.
//!
//! This module only performs `INSERT` and `UPDATE` on the allowlisted
//! tables (`demographic`, `appointment`, `casemgmt_note`, `consultationRequests`,
//! `consultationRequestExt`).  `casemgmt_note` is append-only: it never runs
//! `UPDATE`, only `INSERT`.  There are no `DELETE` statements anywhere.
//!
//! Every write is executed inside an explicit transaction.  The caller decides
//! when to `COMMIT` or `ROLLBACK`, which lets the HAPI identifier write-back
//! run while the Oscar row is still uncommitted.

use anyhow::{Context, Result};
use chrono::TimeZone;
use chrono_tz::Tz;
use mysql_async::{prelude::*, Conn, Params, Value};
use tracing::{info, instrument};

use crate::config::WritebackDatabaseConfig;
use crate::writeback::mappers::{AppointmentRow, ConsultationRequestRow, DemographicRow, NoteRow};

/// MariaDB write-back sink.  Holds a connection pool so every transaction
/// gets its own `Conn`.
pub struct OscarSink {
    pool: mysql_async::Pool,
    sentinel: String,
}

impl OscarSink {
    pub fn new(db: &WritebackDatabaseConfig, sentinel: &str) -> Self {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            db.user, db.password, db.host, db.port, db.schema
        );
        Self {
            pool: mysql_async::Pool::new(url.as_str()),
            sentinel: sentinel.to_string(),
        }
    }

    /// Begins a new write-back transaction.
    pub async fn begin(&self) -> Result<OscarTx> {
        let mut conn = self
            .pool
            .get_conn()
            .await
            .context("connecting to Oscar for writeback")?;
        conn.query_drop("START TRANSACTION")
            .await
            .context("starting writeback transaction")?;
        Ok(OscarTx {
            conn,
            sentinel: self.sentinel.clone(),
        })
    }
}

/// An in-flight write-back transaction.  The caller commits or rolls back.
pub struct OscarTx {
    conn: Conn,
    sentinel: String,
}

impl OscarTx {
    /// INSERT or UPDATE a `demographic` row.
    ///
    /// When `existing` is `Some(demographic_no)`, only the allowlisted columns
    /// are touched.  Otherwise a new row is inserted and the generated
    /// `demographic_no` is returned.
    #[instrument(skip(self, row))]
    pub async fn write_demographic(
        &mut self,
        existing: Option<&str>,
        row: &DemographicRow,
        tz: &Tz,
    ) -> Result<String> {
        let now = now_local(tz);
        if let Some(id) = existing {
            let mut sets = Vec::new();
            let mut params: Vec<Value> = Vec::new();
            push_set(&mut sets, &mut params, "first_name", row.first_name.as_deref());
            push_set(&mut sets, &mut params, "last_name", row.last_name.as_deref());

            // middleNames is not-null in Oscar's Demographic.hbm.xml; never write NULL.
            push_set_required(
                &mut sets,
                &mut params,
                "middleNames",
                row.middle_names.as_deref(),
                "",
            );
            
            push_set(&mut sets, &mut params, "pref_name", row.pref_name.as_deref());
            push_set(&mut sets, &mut params, "title", row.title.as_deref());
            push_set(&mut sets, &mut params, "address", row.address.as_deref());
            push_set(&mut sets, &mut params, "city", row.city.as_deref());
            push_set(&mut sets, &mut params, "province", row.province.as_deref());
            push_set(&mut sets, &mut params, "postal", row.postal.as_deref());
            push_set(&mut sets, &mut params, "phone", row.phone.as_deref());
            push_set(&mut sets, &mut params, "phone2", row.phone2.as_deref());
            push_set(&mut sets, &mut params, "email", row.email.as_deref());
            push_set(&mut sets, &mut params, "year_of_birth", row.year_of_birth.as_deref());
            push_set(&mut sets, &mut params, "month_of_birth", row.month_of_birth.as_deref());
            push_set(&mut sets, &mut params, "date_of_birth", row.date_of_birth.as_deref());
            push_set(&mut sets, &mut params, "sex", row.sex.as_deref());

            // Always stamp provenance on an AMT-initiated touch.
            sets.push("lastUpdateDate=?".to_string());
            params.push(Value::Bytes(now.as_bytes().to_vec()));
            sets.push("lastUpdateUser=?".to_string());
            params.push(Value::Bytes(self.sentinel.as_bytes().to_vec()));

            let sql = format!(
                "UPDATE demographic SET {} WHERE demographic_no = ?",
                sets.join(", ")
            );
            params.push(Value::Bytes(id.as_bytes().to_vec()));

            self.conn
                .exec_drop(sql, Params::Positional(params))
                .await
                .context("updating demographic")?;
            info!("demographic updated: demographic_no={id}");
            Ok(id.to_string())
        } else {
            let mut cols = Vec::new();
            let mut params: Vec<Value> = Vec::new();
            push_col(&mut cols, &mut params, "first_name", row.first_name.as_deref());
            push_col(&mut cols, &mut params, "last_name", row.last_name.as_deref());
            
            // middleNames is not-null in Oscar's Demographic.hbm.xml; never write NULL.
            push_col_required(
                &mut cols,
                &mut params,
                "middleNames",
                row.middle_names.as_deref(),
                "",
            );
            
            push_col(&mut cols, &mut params, "pref_name", row.pref_name.as_deref());
            push_col(&mut cols, &mut params, "title", row.title.as_deref());
            push_col(&mut cols, &mut params, "address", row.address.as_deref());
            push_col(&mut cols, &mut params, "city", row.city.as_deref());
            push_col(&mut cols, &mut params, "province", row.province.as_deref());
            push_col(&mut cols, &mut params, "postal", row.postal.as_deref());
            push_col(&mut cols, &mut params, "phone", row.phone.as_deref());
            push_col(&mut cols, &mut params, "phone2", row.phone2.as_deref());
            push_col(&mut cols, &mut params, "email", row.email.as_deref());
            push_col(&mut cols, &mut params, "year_of_birth", row.year_of_birth.as_deref());
            push_col(&mut cols, &mut params, "month_of_birth", row.month_of_birth.as_deref());
            push_col(&mut cols, &mut params, "date_of_birth", row.date_of_birth.as_deref());
            push_col(&mut cols, &mut params, "sex", row.sex.as_deref());
            push_col(&mut cols, &mut params, "patient_status", row.patient_status.as_deref());

            cols.push("lastUpdateDate".to_string());
            params.push(Value::Bytes(now.as_bytes().to_vec()));
            cols.push("lastUpdateUser".to_string());
            params.push(Value::Bytes(self.sentinel.as_bytes().to_vec()));

            let placeholders = std::iter::repeat("?").take(cols.len()).collect::<Vec<_>>();
            let sql = format!(
                "INSERT INTO demographic ({}) VALUES ({})",
                cols.join(", "),
                placeholders.join(", ")
            );

            self.conn
                .exec_drop(sql, Params::Positional(params))
                .await
                .context("inserting demographic")?;

            let id = self.last_insert_id().await?;
            info!("demographic inserted: demographic_no={id}");
            Ok(id.to_string())
        }
    }

    /// INSERT or UPDATE an `appointment` row.
    #[instrument(skip(self, row))]
    pub async fn write_appointment(
        &mut self,
        existing: Option<&str>,
        row: &AppointmentRow,
    ) -> Result<String> {
        let demographic_no = row
            .demographic_no
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("appointment requires demographic_no"))?;

        if let Some(id) = existing {
            let mut sets = Vec::new();
            let mut params: Vec<Value> = Vec::new();
            push_set(&mut sets, &mut params, "demographic_no", Some(demographic_no));
            push_set(&mut sets, &mut params, "provider_no", row.provider_no.as_deref());
            push_set(&mut sets, &mut params, "appointment_date", row.appointment_date.as_deref());
            push_set(&mut sets, &mut params, "start_time", row.start_time.as_deref());
            push_set(&mut sets, &mut params, "end_time", row.end_time.as_deref());
            push_set(&mut sets, &mut params, "status", row.status.as_deref());
            push_set(&mut sets, &mut params, "reason", row.reason.as_deref());

            sets.push("bookingSource=?".to_string());
            params.push(Value::Bytes(b"OSCAR".to_vec()));
            sets.push("lastupdateuser=?".to_string());
            params.push(Value::Bytes(self.sentinel.as_bytes().to_vec()));

            let sql = format!(
                "UPDATE appointment SET {} WHERE appointment_no = ?",
                sets.join(", ")
            );
            params.push(Value::Bytes(id.as_bytes().to_vec()));

            self.conn
                .exec_drop(sql, Params::Positional(params))
                .await
                .context("updating appointment")?;
            info!("appointment updated: appointment_no={id}");
            Ok(id.to_string())
        } else {
            let mut cols = vec!["demographic_no".to_string()];
            let mut params: Vec<Value> = vec![Value::Bytes(demographic_no.as_bytes().to_vec())];
            push_col(&mut cols, &mut params, "provider_no", row.provider_no.as_deref());
            push_col(&mut cols, &mut params, "appointment_date", row.appointment_date.as_deref());
            push_col(&mut cols, &mut params, "start_time", row.start_time.as_deref());
            push_col(&mut cols, &mut params, "end_time", row.end_time.as_deref());
            push_col(&mut cols, &mut params, "status", row.status.as_deref());
            push_col(&mut cols, &mut params, "reason", row.reason.as_deref());

            cols.push("bookingSource".to_string());
            params.push(Value::Bytes(b"OSCAR".to_vec()));
            cols.push("lastupdateuser".to_string());
            params.push(Value::Bytes(self.sentinel.as_bytes().to_vec()));

            let placeholders = std::iter::repeat("?").take(cols.len()).collect::<Vec<_>>();
            let sql = format!(
                "INSERT INTO appointment ({}) VALUES ({})",
                cols.join(", "),
                placeholders.join(", ")
            );

            self.conn
                .exec_drop(sql, Params::Positional(params))
                .await
                .context("inserting appointment")?;

            let id = self.last_insert_id().await?;
            info!("appointment inserted: appointment_no={id}");
            Ok(id.to_string())
        }
    }

    /// INSERT a new `casemgmt_note` revision.
    ///
    /// `casemgmt_note` is append-only; this function intentionally has no
    /// UPDATE equivalent.  If `existing_uuid` is `Some`, that `uuid` is re-used
    /// for the new revision; otherwise a fresh v4 UUID is generated.
    #[instrument(skip(self, row))]
    pub async fn write_note(
        &mut self,
        existing_uuid: Option<&str>,
        row: &NoteRow,
        tz: &Tz,
    ) -> Result<String> {
        let now = now_local(tz);
        let uuid = existing_uuid
            .map(String::from)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let note = row
            .note
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("casemgmt_note requires note text"))?;
        let demographic_no = row
            .demographic_no
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("casemgmt_note requires demographic_no"))?;

        let observation = row.observation_date.clone().unwrap_or(now.clone());
        let encounter_type = row.encounter_type.clone().unwrap_or_else(|| "AMB".to_string());

        let mut cols = vec![
            "uuid".to_string(),
            "demographic_no".to_string(),
            "observation_date".to_string(),
            "update_date".to_string(),
            "encounter_type".to_string(),
            "note".to_string(),
            "signed".to_string(),
            "archived".to_string(),
        ];
        let mut params: Vec<Value> = vec![
            Value::Bytes(uuid.as_bytes().to_vec()),
            Value::Bytes(demographic_no.as_bytes().to_vec()),
            Value::Bytes(observation.as_bytes().to_vec()),
            Value::Bytes(now.as_bytes().to_vec()),
            Value::Bytes(encounter_type.as_bytes().to_vec()),
            Value::Bytes(note.as_bytes().to_vec()),
            Value::Int(if row.signed { 1 } else { 0 }),
            Value::Int(if row.archived { 1 } else { 0 }),
        ];

        if let Some(p) = row.provider_no.as_deref() {
            cols.push("provider_no".to_string());
            params.push(Value::Bytes(p.as_bytes().to_vec()));
        }
        if let Some(p) = row.signing_provider_no.as_deref() {
            cols.push("signing_provider_no".to_string());
            params.push(Value::Bytes(p.as_bytes().to_vec()));
        }
        if let Some(a) = row.appointment_no.as_deref() {
            cols.push("appointmentNo".to_string());
            params.push(Value::Bytes(a.as_bytes().to_vec()));
        }

        let placeholders = std::iter::repeat("?").take(cols.len()).collect::<Vec<_>>();
        let sql = format!(
            "INSERT INTO casemgmt_note ({}) VALUES ({})",
            cols.join(", "),
            placeholders.join(", ")
        );

        self.conn
            .exec_drop(sql, Params::Positional(params))
            .await
            .context("inserting casemgmt_note")?;

        info!("casemgmt_note inserted: uuid={uuid}");
        Ok(uuid)
    }

    /// INSERT or UPDATE a `consultationRequests` row.
    #[instrument(skip(self, row))]
    pub async fn write_consultation_request(
        &mut self,
        existing: Option<&str>,
        row: &ConsultationRequestRow,
        tz: &Tz,
    ) -> Result<String> {
        let demographic_no = row
            .demographic_no
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("consultationRequests requires demographic_no"))?;
        let service_id = row
            .service_id
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("consultationRequests requires service_id"))?;
        let status = row.status.as_deref().unwrap_or("1");
        let source = row.source.as_deref().unwrap_or("AMT-eReferral");
        let now = now_local(tz);

        if let Some(id) = existing {
            let mut sets = Vec::new();
            let mut params: Vec<Value> = Vec::new();
            push_set(&mut sets, &mut params, "demographicNo", Some(demographic_no));
            push_set(&mut sets, &mut params, "providerNo", row.provider_no.as_deref());
            push_set(&mut sets, &mut params, "serviceId", Some(service_id));
            push_set(&mut sets, &mut params, "specId", row.spec_id.as_deref());
            push_set(&mut sets, &mut params, "referalDate", row.referal_date.as_deref());
            push_set(&mut sets, &mut params, "reason", row.reason.as_deref());
            push_set(&mut sets, &mut params, "clinicalInfo", row.clinical_info.as_deref());
            push_set(&mut sets, &mut params, "concurrentProblems", row.concurrent_problems.as_deref());
            push_set(&mut sets, &mut params, "currentMeds", row.current_meds.as_deref());
            push_set(&mut sets, &mut params, "allergies", row.allergies.as_deref());
            push_set(&mut sets, &mut params, "sendTo", row.send_to.as_deref());

            // patientWillBook is a primitive boolean in Oscar's entity;
            // urgency is guarded by a null-deref in the consult list JSP.
            // Both default to safe, non-null values.
            push_set_required(
                &mut sets,
                &mut params,
                "patientWillBook",
                row.patient_will_book.as_deref(),
                "0",
            );
            push_set_required(&mut sets, &mut params, "urgency", row.urgency.as_deref(), "2");

            // status, appointmentDate, appointmentTime, and statusText are owned
            // by the Oscar consult workflow and must never be overwritten.
            sets.push("lastUpdateDate=?".to_string());
            params.push(Value::Bytes(now.as_bytes().to_vec()));

            let sql = format!(
                "UPDATE consultationRequests SET {} WHERE requestId = ?",
                sets.join(", ")
            );
            params.push(Value::Bytes(id.as_bytes().to_vec()));

            self.conn
                .exec_drop(sql, Params::Positional(params))
                .await
                .context("updating consultationRequests")?;
            info!("consultationRequests updated: requestId={id}");
            Ok(id.to_string())
        } else {
            let mut cols = Vec::new();
            let mut params: Vec<Value> = Vec::new();
            push_col(&mut cols, &mut params, "demographicNo", Some(demographic_no));
            push_col(&mut cols, &mut params, "providerNo", row.provider_no.as_deref());
            push_col(&mut cols, &mut params, "serviceId", Some(service_id));
            push_col(&mut cols, &mut params, "specId", row.spec_id.as_deref());
            push_col(&mut cols, &mut params, "referalDate", row.referal_date.as_deref());
            push_col(&mut cols, &mut params, "reason", row.reason.as_deref());
            push_col(&mut cols, &mut params, "clinicalInfo", row.clinical_info.as_deref());
            push_col(&mut cols, &mut params, "concurrentProblems", row.concurrent_problems.as_deref());
            push_col(&mut cols, &mut params, "currentMeds", row.current_meds.as_deref());
            push_col(&mut cols, &mut params, "allergies", row.allergies.as_deref());
            push_col(&mut cols, &mut params, "status", Some(status));
            push_col(&mut cols, &mut params, "source", Some(source));
            push_col(&mut cols, &mut params, "sendTo", row.send_to.as_deref());

            // patientWillBook is a primitive boolean in Oscar's entity;
            // urgency is guarded by a null-deref in the consult list JSP.
            // Both default to safe, non-null values.
            push_col_required(
                &mut cols,
                &mut params,
                "patientWillBook",
                row.patient_will_book.as_deref(),
                "0",
            );
            push_col_required(&mut cols, &mut params, "urgency", row.urgency.as_deref(), "2");

            cols.push("lastUpdateDate".to_string());
            params.push(Value::Bytes(now.as_bytes().to_vec()));

            let placeholders = std::iter::repeat("?").take(cols.len()).collect::<Vec<_>>();
            let sql = format!(
                "INSERT INTO consultationRequests ({}) VALUES ({})",
                cols.join(", "),
                placeholders.join(", ")
            );

            self.conn
                .exec_drop(sql, Params::Positional(params))
                .await
                .context("inserting consultationRequests")?;

            let id = self.last_insert_id().await?;
            info!("consultationRequests inserted: requestId={id}");
            Ok(id.to_string())
        }
    }

    /// Upserts a `consultationRequestExt` row keyed on `(requestId, name)`.
    pub async fn upsert_consultation_request_ext(
        &mut self,
        request_id: &str,
        name: &str,
        value: &str,
    ) -> Result<()> {
        let existing: Vec<(u64,)> = self
            .conn
            .exec(
                "SELECT id FROM consultationRequestExt WHERE requestId = ? AND name = ?",
                Params::Positional(vec![
                    Value::Bytes(request_id.as_bytes().to_vec()),
                    Value::Bytes(name.as_bytes().to_vec()),
                ]),
            )
            .await
            .context("selecting consultationRequestExt")?;

        if existing.is_empty() {
            self.conn
                .exec_drop(
                    "INSERT INTO consultationRequestExt (requestId, name, value, dateCreated) VALUES (?, ?, ?, NOW())",
                    Params::Positional(vec![
                        Value::Bytes(request_id.as_bytes().to_vec()),
                        Value::Bytes(name.as_bytes().to_vec()),
                        Value::Bytes(value.as_bytes().to_vec()),
                    ]),
                )
                .await
                .context("inserting consultationRequestExt")?;
        } else {
            self.conn
                .exec_drop(
                    "UPDATE consultationRequestExt SET value = ? WHERE id = ?",
                    Params::Positional(vec![
                        Value::Bytes(value.as_bytes().to_vec()),
                        Value::Bytes(existing[0].0.to_string().as_bytes().to_vec()),
                    ]),
                )
                .await
                .context("updating consultationRequestExt")?;
        }
        info!("consultationRequestExt upserted: requestId={request_id} name={name}");
        Ok(())
    }

    /// Commits the transaction.
    pub async fn commit(mut self) -> Result<()> {
        self.conn
            .query_drop("COMMIT")
            .await
            .context("committing writeback transaction")?;
        Ok(())
    }

    /// Rolls the transaction back.
    pub async fn rollback(mut self) -> Result<()> {
        self.conn
            .query_drop("ROLLBACK")
            .await
            .context("rolling back writeback transaction")?;
        Ok(())
    }

    async fn last_insert_id(&mut self) -> Result<u64> {
        let rows: Vec<(u64,)> = self
            .conn
            .query("SELECT LAST_INSERT_ID()")
            .await
            .context("fetching LAST_INSERT_ID")?;
        rows.into_iter()
            .next()
            .map(|r| r.0)
            .ok_or_else(|| anyhow::anyhow!("LAST_INSERT_ID() returned no rows"))
    }
}

fn now_local(tz: &Tz) -> String {
    chrono::Utc::now()
        .with_timezone(tz)
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

fn push_set(sets: &mut Vec<String>, params: &mut Vec<Value>, col: &str, val: Option<&str>) {
    if let Some(v) = val {
        sets.push(format!("{col}=?"));
        params.push(Value::Bytes(v.as_bytes().to_vec()));
    }
}

fn push_col(cols: &mut Vec<String>, params: &mut Vec<Value>, col: &str, val: Option<&str>) {
    if let Some(v) = val {
        cols.push(col.to_string());
        params.push(Value::Bytes(v.as_bytes().to_vec()));
    }
}

/// Always writes `col`, substituting `default` when `val` is `None`.
///
/// Use for columns Oscar's Hibernate mapping treats as mandatory — either a
/// primitive-typed field (cannot hold NULL) or `not-null="true"` in the
/// entity's .hbm.xml. MySQL accepts NULL for these; Oscar cannot load the
/// row. See HIBERNATE_MANDATORY_COLS.md.
fn push_col_required(
    cols: &mut Vec<String>,
    params: &mut Vec<Value>,
    col: &str,
    val: Option<&str>,
    default: &str,
) {
    let v = val.unwrap_or(default);
    cols.push(col.to_string());
    params.push(Value::Bytes(v.as_bytes().to_vec()));
}

/// `push_col_required` counterpart for `UPDATE ... SET` clauses.
fn push_set_required(
    sets: &mut Vec<String>,
    params: &mut Vec<Value>,
    col: &str,
    val: Option<&str>,
    default: &str,
) {
    let v = val.unwrap_or(default);
    sets.push(format!("{col}=?"));
    params.push(Value::Bytes(v.as_bytes().to_vec()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_col_required_fills_default() {
        let mut cols = Vec::new();
        let mut params = Vec::new();
        push_col_required(&mut cols, &mut params, "urgency", None, "2");
        assert_eq!(cols, vec!["urgency"]);
        assert_eq!(params, vec![Value::Bytes(b"2".to_vec())]);
    }

    #[test]
    fn push_col_required_uses_provided_value() {
        let mut cols = Vec::new();
        let mut params = Vec::new();
        push_col_required(&mut cols, &mut params, "urgency", Some("1"), "2");
        assert_eq!(cols, vec!["urgency"]);
        assert_eq!(params, vec![Value::Bytes(b"1".to_vec())]);
    }

    #[test]
    fn push_set_required_fills_default() {
        let mut sets = Vec::new();
        let mut params = Vec::new();
        push_set_required(&mut sets, &mut params, "patientWillBook", None, "0");
        assert_eq!(sets, vec!["patientWillBook=?"]);
        assert_eq!(params, vec![Value::Bytes(b"0".to_vec())]);
    }

    #[test]
    fn push_col_skips_none() {
        let mut cols = Vec::new();
        let mut params = Vec::new();
        push_col(&mut cols, &mut params, "specId", None);
        assert!(cols.is_empty());
        assert!(params.is_empty());
    }
}

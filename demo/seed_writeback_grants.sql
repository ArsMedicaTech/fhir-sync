-- Least-privilege grants for the AMT -> Oscar write-back user.
-- Runs automatically via MariaDB's /docker-entrypoint-initdb.d/ in the
-- docker-compose.writeback.yml harness.
--
-- This user is intentionally limited to INSERT/UPDATE on demographic and
-- appointment and INSERT-only on casemgmt_note. No DELETE grant exists.
-- Do not apply this seed to a production Oscar database.

CREATE USER IF NOT EXISTS 'amt_writeback'@'%' IDENTIFIED BY 'amtpw';

GRANT SELECT, INSERT, UPDATE ON oscar.demographic   TO 'amt_writeback'@'%';
GRANT SELECT, INSERT, UPDATE ON oscar.appointment   TO 'amt_writeback'@'%';
GRANT SELECT, INSERT           ON oscar.casemgmt_note TO 'amt_writeback'@'%';

FLUSH PRIVILEGES;

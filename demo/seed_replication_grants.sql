-- Least-privilege grant for the Oscar -> AMT binlog source user (fhirsync).
-- Distinct from amt_writeback -- this user only reads, never writes.

CREATE USER IF NOT EXISTS 'fhirsync'@'%' IDENTIFIED BY 'somesecurepassword';

-- Binlog streaming itself. These are global privileges in MariaDB;
-- they cannot be scoped to a single schema.
GRANT REPLICATION SLAVE, REPLICATION CLIENT ON *.* TO 'fhirsync'@'%';

-- information_schema.columns lookups at startup, plus the live
-- consultationRequests/consultationRequestExt correlation query.
GRANT SELECT ON oscar.* TO 'fhirsync'@'%';

FLUSH PRIVILEGES;

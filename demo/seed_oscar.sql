-- Oscar-shaped `demographic` table + sample data for the demo
-- harness (docker-compose.demo.yml). Runs automatically via MariaDB's
-- /docker-entrypoint-initdb.d/ on first container start.

-- The binlog replication user needs REPLICATION SLAVE/CLIENT, which
-- MYSQL_USER (set via env) does not get by default (only ALL PRIVILEGES
-- on MYSQL_DATABASE). Grant them explicitly (spec §8 Known Risks).
GRANT REPLICATION SLAVE, REPLICATION CLIENT ON *.* TO 'oscar'@'%';
FLUSH PRIVILEGES;

USE oscar;

-- Column names match what src/mapping/demographic.rs resolves via
-- information_schema (D3) — see F5 for the 3-part DOB split.
CREATE TABLE IF NOT EXISTS demographic (
    demographic_no INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    first_name     VARCHAR(100),
    last_name      VARCHAR(100),
    year_of_birth  VARCHAR(4),
    month_of_birth VARCHAR(2),
    date_of_birth  VARCHAR(2),
    sex            VARCHAR(20),
    email          VARCHAR(255),
    phone          VARCHAR(50),
    hin            VARCHAR(20),
    city           VARCHAR(100),
    province       VARCHAR(10),
    postal         VARCHAR(10)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 120 seeded rows (Phase 3 acceptance: "--backfill over 100+ seeded rows
-- produces exactly that many Patients").
INSERT INTO demographic
    (first_name, last_name, year_of_birth, month_of_birth, date_of_birth,
     sex, email, phone, city, province, postal)
WITH RECURSIVE seq (n) AS (
    SELECT 1
    UNION ALL
    SELECT n + 1 FROM seq WHERE n < 120
)
SELECT
    CONCAT('Demo', n),
    CONCAT('Patient', n),
    CAST(1950 + (n % 60) AS CHAR),
    CAST(1 + (n % 12) AS CHAR),
    CAST(1 + (n % 28) AS CHAR),
    IF(n % 2 = 0, 'M', 'F'),
    CONCAT('demo.patient', n, '@example.invalid'),
    CONCAT('555-01', LPAD(n, 2, '0')),
    'Toronto',
    'ON',
    'M5V1A1'
FROM seq;

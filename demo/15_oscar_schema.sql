-- demo/15_oscar_schema.sql
--
-- Brings the demo harness tables in line with the REAL Oscar 19.x schema.
--
-- Runs after 10_seed_oscar.sql (minimal `demographic` + 120 synthetic rows,
-- which the Phase 3 backfill acceptance test depends on) and before
-- 20_fixtures.sql.
--
-- WHY THIS FILE EXISTS
-- --------------------
-- 10_seed_oscar.sql creates a 14-column `demographic` shaped to match what
-- src/mapping/demographic.rs reads. That is backwards: a fixture schema built
-- to match the mapper can only ever confirm what the mapper already does.
--
-- Concretely, it declares a `country` column. Real Oscar's `demographic` has
-- NO country column -- verified against a live Oscar 19.x instance, full
-- information_schema dump: the address columns are address / city / province /
-- postal plus the residential* quartet, and nothing else.
--
-- Because compose_location() does `lookup_any(&["country"])?`, the `?`
-- short-circuits on every real Oscar row and Patient.address is NEVER emitted.
-- The demo schema hides this. Dropping the column here makes the harness
-- reproduce the production bug (spec defect P1).

USE oscar;

-- ---------------------------------------------------------------- demographic
-- Remove the column real Oscar does not have. This is deliberate: it makes
-- P1 reproducible in CI.
ALTER TABLE demographic DROP COLUMN country;

-- Add the columns real Oscar has that the minimal demo table lacks.
-- Types and nullability follow the live 19.x schema.
ALTER TABLE demographic
  ADD COLUMN title                    VARCHAR(5)   NULL,
  ADD COLUMN middleNames              VARCHAR(255) NULL,
  ADD COLUMN alias                    VARCHAR(60)  NULL,
  ADD COLUMN pref_name                VARCHAR(255) NULL,
  ADD COLUMN address                  VARCHAR(255) NULL,
  ADD COLUMN phone2                   VARCHAR(50)  NULL,
  ADD COLUMN residentialAddress       VARCHAR(255) NULL,
  ADD COLUMN residentialCity          VARCHAR(255) NULL,
  ADD COLUMN residentialProvince      VARCHAR(255) NULL,
  ADD COLUMN residentialPostal        VARCHAR(255) NULL,
  ADD COLUMN ver                      CHAR(3)      NULL,
  ADD COLUMN roster_status            VARCHAR(20)  NULL,
  ADD COLUMN patient_status           VARCHAR(20)  NULL,
  ADD COLUMN patient_status_date      DATE         NULL,
  ADD COLUMN date_joined              DATE         NULL,
  ADD COLUMN chart_no                 VARCHAR(20)  NULL,
  ADD COLUMN official_lang            VARCHAR(80)  NULL,
  ADD COLUMN spoken_lang              VARCHAR(80)  NULL,
  ADD COLUMN provider_no              VARCHAR(6)   NULL,
  ADD COLUMN end_date                 DATE         NULL,
  ADD COLUMN eff_date                 DATE         NULL,
  ADD COLUMN hc_type                  VARCHAR(20)  NULL,
  ADD COLUMN family_doctor            VARCHAR(80)  NULL,
  ADD COLUMN citizenship              VARCHAR(80)  NULL,
  ADD COLUMN sin                      VARCHAR(30)  NULL,
  ADD COLUMN country_of_origin        CHAR(4)      NULL,
  ADD COLUMN consentToUseEmailForCare TINYINT(1)   NULL,
  ADD COLUMN lastUpdateUser           VARCHAR(6)   NULL,
  ADD COLUMN lastUpdateDate           DATETIME     NULL;

-- ------------------------------------------------------------------ provider
-- provider_no is VARCHAR(6) and application-assigned, NOT auto_increment.
CREATE TABLE IF NOT EXISTS provider (
    provider_no        VARCHAR(6)   NOT NULL PRIMARY KEY,
    last_name          VARCHAR(255) NOT NULL DEFAULT '',
    first_name         VARCHAR(255) NOT NULL DEFAULT '',
    provider_type      VARCHAR(20)  NOT NULL DEFAULT '',
    specialty          VARCHAR(255) NOT NULL DEFAULT '',
    team               VARCHAR(20)  NULL,
    sex                VARCHAR(2)   NOT NULL DEFAULT '',
    dob                DATE         NULL,
    address            VARCHAR(255) NULL,
    phone              VARCHAR(50)  NULL,
    work_phone         VARCHAR(50)  NULL,
    ohip_no            VARCHAR(20)  NULL,
    billing_no         VARCHAR(20)  NULL,
    practitionerNo     VARCHAR(20)  NULL,
    practitionerNoType VARCHAR(20)  NULL,
    status             CHAR(1)      NULL,
    email              VARCHAR(255) NULL,
    title              VARCHAR(10)  NULL,
    init               VARCHAR(10)  NULL,
    job_title          VARCHAR(255) NULL,
    lastUpdateUser     VARCHAR(6)   NULL,
    lastUpdateDate     DATETIME     NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Oscar ships these two on every install. -1 is the system actor and MUST NOT
-- sync (spec D3); seeding it here is what makes that assertion meaningful.
INSERT IGNORE INTO provider
    (provider_no, last_name, first_name, provider_type, specialty, sex, status, lastUpdateDate)
VALUES
    ('-1',     'system',   'system', 'system', 'system', 'M', '1', NOW()),
    ('999998', 'oscardoc', 'doctor', 'doctor', '',       'M', '1', NOW());

-- --------------------------------------------------------------- appointment
-- NOTE the collations. In real Oscar, appointment.status is utf8mb3_bin
-- (case-SENSITIVE) while appointment_status.status is utf8mb4_general_ci
-- (case-INSENSITIVE). Joining them without an explicit COLLATE silently
-- matches 't'/'T', 'B'/'b', 'C'/'c'. Reproduced here on purpose.
CREATE TABLE IF NOT EXISTS appointment (
    appointment_no   INT(12)      NOT NULL AUTO_INCREMENT PRIMARY KEY,
    provider_no      VARCHAR(6)   NOT NULL DEFAULT '',
    appointment_date DATE         NOT NULL,
    start_time       TIME         NOT NULL,
    end_time         TIME         NOT NULL,
    name             VARCHAR(50)  NULL,
    demographic_no   INT(10)      NULL,
    notes            TEXT         NULL,
    reason           VARCHAR(255) NULL,
    location         VARCHAR(30)  NULL,
    type             VARCHAR(255) NULL,
    style            VARCHAR(10)  NULL,
    billing          VARCHAR(10)  NULL,
    status           CHAR(2)      CHARACTER SET utf8mb3 COLLATE utf8mb3_bin NULL,
    createdatetime   DATETIME     NULL,
    updatedatetime   DATETIME     NULL,
    creator          VARCHAR(20)  NULL,
    lastupdateuser   VARCHAR(6)   NULL,
    remarks          VARCHAR(255) NULL,
    urgency          VARCHAR(20)  NULL,
    bookingSource    VARCHAR(50)  NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS appointment_status (
    id            INT(10)      NOT NULL AUTO_INCREMENT PRIMARY KEY,
    status        CHAR(2)      CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    description   VARCHAR(255) NULL,
    color         VARCHAR(20)  NULL,
    icon          VARCHAR(255) NULL,
    active        TINYINT(1)   NULL,
    editable      TINYINT(1)   NULL,
    short_letters VARCHAR(2)   NULL,
    short_letter_colour VARCHAR(20) NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- The 14 stock codes. Case matters: 't' != 'T', 'h' != 'H', 'B' != 'b'.
INSERT IGNORE INTO appointment_status (status, description, active, editable, short_letters) VALUES
    ('t','To Do',1,0,'t'),            ('T','Daysheet Printed',1,0,'T'),
    ('h','Confirmed',1,0,'h'),        ('H','Here',1,0,'H'),
    ('P','Picked',1,0,'P'),           ('E','Empty Room',1,0,'E'),
    ('B','Billed',1,0,'B'),           ('C','Cancelled',1,0,'C'),
    ('N','No Show',1,0,'N'),          ('a','Customized 1',1,1,'a'),
    ('b','Customized 2',1,1,'b'),     ('c','Customized 3',1,1,'c'),
    ('d','Customized 4',1,1,'d'),     ('e','Customized 5',1,1,'e');

-- --------------------------------------------------------- demographic_merged
CREATE TABLE IF NOT EXISTS demographic_merged (
    id             INT(10)    NOT NULL AUTO_INCREMENT PRIMARY KEY,
    demographic_no INT(10)    NOT NULL,
    merged_to      INT(10)    NOT NULL,
    deleted        TINYINT(1) NOT NULL DEFAULT 0,
    lastUpdateUser VARCHAR(6) NULL,
    lastUpdateDate DATE       NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Demo data: insert one new
-- patient into the live `demographic` table. fhir-sync's binlog listener
-- picks up the WriteRows event and the patient should appear in HAPI
-- within seconds.
--
-- Run with:
--   docker compose -f docker-compose.demo.yml exec -T mariadb \
--     mysql -uoscar -poscarpw oscar < demo/insert_patient.sql
--
-- Then check:
--   curl "http://localhost:8082/fhir/Patient?identifier=https://arsmedicatech.com/fhir/sid/oscar-demographic-no|<demographic_no printed below>"

USE oscar;

INSERT INTO demographic
    (first_name, last_name, year_of_birth, month_of_birth, date_of_birth,
     sex, email, phone, city, province, country, postal)
VALUES
    ('Ada', 'Lovelace', '1985', '12', '10', 'F',
     'ada.lovelace@example.invalid', '555-0199',
     'Toronto', 'ON', 'Canada', 'M5V1A1');

SELECT LAST_INSERT_ID() AS demographic_no;

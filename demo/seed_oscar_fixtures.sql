-- demo/seed_oscar_fixtures.sql
--
-- Oscar fixture set for fhir-sync integration testing.
-- Mirrors the rows seeded into the local Oscar 19.x instance on 2026-08-05.
-- Every row exists to exercise a specific mapping branch; see the comment
-- on each. Do not "tidy" the odd-looking values -- they are the tests.
--
-- Assumes the Oscar schema is already present (this file seeds data only).
-- Idempotent: explicit PKs + REPLACE semantics where the PK is assigned.
--
-- Ranges reserved for fixtures so they are trivially identifiable and
-- deletable:
--   provider_no     100001-100003
--   demographic_no  101-107
--   appointment_no  auto (cleaned by demographic_no range)

-- ---------------------------------------------------------------- providers
REPLACE INTO provider
 (provider_no, last_name, first_name, provider_type, specialty, team, sex, dob,
  address, phone, work_phone, ohip_no, billing_no, practitionerNo,
  practitionerNoType, status, email, title, init, job_title,
  lastUpdateUser, lastUpdateDate)
VALUES
 -- full identifier set: oscar-provider + BC MSP billing_no + CPSID
 ('100001','Chen','Sarah','doctor','Family Practice','TeamA','F','1979-04-12',
  '1200 Burrard St, Vancouver BC','604-555-0111','604-555-0112',NULL,'A1234','91234','CPSID',
  '1','schen@clinic.example','Dr.','SC','Family Physician','999998',NOW()),
 -- sparse identifiers; apostrophe in surname (escaping test)
 ('100002','O''Brien','Maureen','nurse','Nursing','TeamA','F','1985-09-30',
  '1200 Burrard St, Vancouver BC','604-555-0121',NULL,NULL,NULL,NULL,'',
  '1','mobrien@clinic.example','RN','MO','Registered Nurse','999998',NOW()),
 -- status '0' -> Practitioner.active = false
 ('100003','Ngo','Thanh','doctor','Internal Medicine',NULL,'M','1972-01-05',
  '88 W Broadway, Vancouver BC','604-555-0131',NULL,NULL,'B5678','88221','CPSID',
  '0','tngo@clinic.example','Dr.','TN','Internist','999998',NOW());

-- NOTE: provider_no '-1' (system actor) ships with Oscar and must NOT sync.
-- It is deliberately not seeded here; it already exists on any install.

-- ------------------------------------------------------------ demographics
REPLACE INTO demographic
 (demographic_no, title, last_name, first_name, middleNames, alias, pref_name,
  address, city, province, postal, phone, phone2, email,
  residentialAddress, residentialCity, residentialProvince, residentialPostal,
  year_of_birth, month_of_birth, date_of_birth, hin, ver,
  roster_status, patient_status, patient_status_date, date_joined, chart_no,
  official_lang, spoken_lang, provider_no, sex, end_date, eff_date,
  hc_type, family_doctor, citizenship, sin, country_of_origin,
  consentToUseEmailForCare, lastUpdateUser, lastUpdateDate)
VALUES
 -- complete record: two addresses (postal + residential), prefix/middle/nickname
 (101,'Mr.','Whitfield','Robert','James Alan','Bobby','Bob',
  '450 Granville St Apt 12','Vancouver','BC','V6C1V5','604-555-0201','778-555-0202','rwhitfield@example.com',
  '77 Seaview Rd','Burnaby','BC','V5A2B3',
  '1968','07','14','9123456781','','RO','AC','2020-01-15','2019-11-02','CH-0101',
  'English','English','100001','M',NULL,'2019-11-02','BC','Chen, Sarah','Canada','123456789','CA',
  1,'999998',NOW()),
 -- sex 'O' -> gender MUST be "other", not "unknown"; NULL street line
 (102,'Mx.','Adeyemi','Kayode',NULL,NULL,NULL,
  NULL,'Surrey','BC','V3T0A1','604-555-0203',NULL,'kadeyemi@example.com',
  NULL,NULL,NULL,NULL,
  '1991','03','22','9123456782','','RO','AC','2021-06-01','2021-06-01','CH-0102',
  'English','Yoruba','100001','O',NULL,'2021-06-01','BC',NULL,'Canada',NULL,'NG',
  1,'999998',NOW()),
 -- NULL month_of_birth -> birthDate must be ABSENT, other fields still map
 (103,'Ms.','Mueller','Annika',NULL,NULL,NULL,
  '19 Lonsdale Ave','North Vancouver','BC','V7M2E5','604-555-0204',NULL,NULL,
  NULL,NULL,NULL,NULL,
  '1955',NULL,'09','9123456783','','RO','AC','2018-02-11','2018-02-11','CH-0103',
  'English','German','100003','F',NULL,'2018-02-11','BC',NULL,'Germany',NULL,'DE',
  0,'999998',NOW()),
 -- patient_status 'DE' -> active=false + deceasedBoolean
 (104,'Mr.','Tremblay','Luc',NULL,NULL,NULL,
  '3300 Oak St','Vancouver','BC','V6H3N1','604-555-0205',NULL,NULL,
  NULL,NULL,NULL,NULL,
  '1943','11','30','9123456784','','RO','DE','2025-12-04','2010-05-20','CH-0104',
  'French','French','100001','M','2025-12-04','2010-05-20','BC',NULL,'Canada',NULL,'CA',
  0,'999998',NOW()),
 -- patient_status 'IN' -> active=false, NO deceasedBoolean
 (105,'Ms.','Kaur','Simran',NULL,NULL,'Sim',
  '8800 120 St','Delta','BC','V4C6R2','604-555-0206',NULL,'skaur@example.com',
  NULL,NULL,NULL,NULL,
  '1988','05','02','9123456785','','TE','IN','2024-03-30','2015-08-14','CH-0105',
  'English','Punjabi','100001','F','2024-03-30','2015-08-14','BC',NULL,'Canada',NULL,'IN',
  1,'999998',NOW()),
 -- merged away into 101 -> active=false + link[replaced-by]
 (106,'Mr.','Whitfield','Rob',NULL,NULL,NULL,
  '450 Granville St','Vancouver','BC','V6C1V5','604-555-0201',NULL,NULL,
  NULL,NULL,NULL,NULL,
  '1968','07','14','9123456781','','RO','AC','2019-11-02','2019-11-02','CH-0106',
  'English','English','100001','M',NULL,'2019-11-02','BC',NULL,'Canada',NULL,'CA',
  0,'999998',NOW()),
 -- maximally sparse: names + sex only; provider_no = system actor
 (107,NULL,'Doe','Unknown',NULL,NULL,NULL,
  NULL,NULL,NULL,NULL,NULL,NULL,NULL,
  NULL,NULL,NULL,NULL,
  NULL,NULL,NULL,NULL,'',NULL,'AC',NULL,NULL,NULL,
  NULL,NULL,'-1','U',NULL,NULL,NULL,NULL,NULL,NULL,NULL,
  NULL,'999998',NOW());

DELETE FROM demographic_merged WHERE demographic_no = 106;
INSERT INTO demographic_merged (demographic_no, merged_to, deleted, lastUpdateUser, lastUpdateDate)
VALUES (106, 101, 0, '999998', CURDATE());

-- ------------------------------------------------------------- appointments
DELETE FROM appointment WHERE demographic_no BETWEEN 100 AND 107 OR demographic_no = 0;
INSERT INTO appointment
 (provider_no, appointment_date, start_time, end_time, name, demographic_no,
  notes, reason, location, type, style, billing, status,
  createdatetime, updatedatetime, creator, lastupdateuser, remarks, urgency, bookingSource)
VALUES
 -- baseline booked
 ('100001','2026-08-10','09:00:00','09:15:00','Whitfield, Robert',101,
  'Routine follow-up','Annual physical','Main Clinic','Regular','','','t',
  '2026-07-20 10:04:00','2026-07-20 10:04:00','100001','100001',NULL,'routine','front_desk'),
 -- 'h' Confirmed -> booked (distinct from 'H'; collation trap)
 ('100001','2026-08-10','09:15:00','09:30:00','Adeyemi, Kayode',102,
  NULL,'BP recheck','Main Clinic','Regular','','','h',
  '2026-07-21 11:00:00','2026-08-05 08:30:00','100001','100001',NULL,NULL,'online'),
 -- 'H' Here -> checked-in; 45 min duration
 ('100003','2026-08-10','10:00:00','10:45:00','Mueller, Annika',103,
  'Bring med list','Medication review','Main Clinic','Long','','','H',
  '2026-07-22 09:12:00','2026-08-05 09:55:00','100003','100003',NULL,NULL,'front_desk'),
 -- 'B' Billed -> fulfilled (distinct from 'b' Customized 2)
 ('100001','2026-07-28','13:00:00','13:15:00','Kaur, Simran',105,
  NULL,'Lab results','Main Clinic','Regular','','','B',
  '2026-07-10 14:00:00','2026-07-28 13:20:00','100001','100001',NULL,NULL,'front_desk'),
 -- 'C' Cancelled (distinct from 'c' Customized 3)
 ('100001','2026-07-29','14:00:00','14:15:00','Whitfield, Robert',101,
  'Patient called to cancel','Sore throat','Main Clinic','Regular','','','C',
  '2026-07-15 09:00:00','2026-07-28 16:40:00','100001','100001','cancelled by pt',NULL,'front_desk'),
 -- 'N' No Show -> noshow
 ('100003','2026-07-30','11:30:00','11:45:00','Adeyemi, Kayode',102,
  NULL,'Rash','Satellite Office','Regular','','','N',
  '2026-07-16 10:00:00','2026-07-30 12:00:00','100003','100003',NULL,NULL,'online'),
 -- 'a' Customized 1 -> UNMAPPED, must dead-letter
 ('100001','2026-08-12','08:30:00','08:45:00','Tremblay, Luc',104,
  NULL,'Chart closure','Main Clinic','Regular','','','a',
  '2026-07-25 15:00:00','2026-07-25 15:00:00','100001','100001',NULL,NULL,'front_desk'),
 -- 'P' Picked -> checked-in; nurse provider
 ('100002','2026-08-11','15:00:00','15:30:00','Kaur, Simran',105,
  'Nurse visit','Wound care','Main Clinic','Nurse','','','P',
  '2026-07-26 08:00:00','2026-08-05 10:10:00','100002','100002',NULL,'urgent','front_desk'),
 -- demographic_no = 0 -> blocked time, must NOT emit an Appointment
 ('100001','2026-08-13','12:00:00','13:00:00','LUNCH',0,
  'Blocked - lunch',NULL,'Main Clinic','Blocked','','','t',
  '2026-01-05 08:00:00','2026-01-05 08:00:00','100001','100001',NULL,NULL,'front_desk'),
 -- DST spring-forward: 02:30 does not exist in America/Vancouver -> dead-letter
 ('100001','2026-03-08','02:30:00','02:45:00','Whitfield, Robert',101,
  'DST spring-forward: local time does not exist','Early appt','Main Clinic','Regular','','','t',
  '2026-02-20 09:00:00','2026-02-20 09:00:00','100001','100001',NULL,NULL,'front_desk'),
 -- DST fall-back: 01:30 occurs twice -> take first (-07:00), warn
 ('100001','2026-11-01','01:30:00','01:45:00','Adeyemi, Kayode',102,
  'DST fall-back: local time occurs twice','Early appt','Main Clinic','Regular','','','t',
  '2026-10-15 09:00:00','2026-10-15 09:00:00','100001','100001',NULL,NULL,'front_desk'),
 -- system actor as provider -> patient participant only, no practitioner
 ('-1','2026-08-14','07:00:00','07:15:00','SYSTEM BATCH',107,
  'Created by system actor',NULL,'Main Clinic','Regular','','','t',
  '2026-08-01 03:00:00','2026-08-01 03:00:00','-1','-1',NULL,NULL,'batch');

-- ---------------------------------------------------------- consultations
-- Exercises the D1 correlation heuristic in
-- TASK_FEATURES_SPEC_OSCAR_CONSULT_RESPONSE_WRITEBACK.md: for each new
-- consultationResponse row, match consultationRequests on
-- (demographicNo, source='AMT-eReferral', referalDate/referralDate), and
-- dead-letter on 0 or >1 matches rather than guessing (A5/A6).
--
-- Reuses demographic_no 101 (Whitfield, full identifier set) and
-- demographic_no 102/103 (Adeyemi/Mueller) from the seed above.
--
-- Reserved ID ranges so fixtures stay trivially identifiable/deletable:
--   requestId               9001-9004
--   responseId               9101-9104
--   consultationRequestExt   filtered by requestId, not id, on cleanup

DELETE FROM consultationRequestExt WHERE requestId BETWEEN 9001 AND 9004;
DELETE FROM consultationResponse WHERE responseId BETWEEN 9101 AND 9104;
DELETE FROM consultationRequests WHERE requestId BETWEEN 9001 AND 9004;

-- ---------------------------------------------------------- consultationRequests
INSERT INTO consultationRequests
 (requestId, referalDate, serviceId, specId, demographicNo, providerNo,
  status, source, urgency, reason, lastUpdateDate)
VALUES
 -- 9001: happy path. Exactly one AMT-sourced request for (101, 2026-08-01).
 -- Paired with response 9101 below -> D1 resolves unambiguously (A3).
 (9001,'2026-08-01',53,NULL,101,'100001',
  '1','AMT-eReferral',NULL,'Cardiology referral -- routine follow-up','2026-08-01 09:00:00'),
 -- 9002 & 9003: deliberately collide. Same demographicNo, same source,
 -- same referalDate as each other -- mirrors the real requestId 7/8
 -- collision noted in the Phase 2 spec's D1 discussion. Paired with
 -- response 9102 below -> D1 must find 2 rows and dead-letter (A6), not
 -- pick one.
 (9002,'2026-08-05',54,NULL,101,'100001',
  '1','AMT-eReferral',NULL,'Dermatology referral -- suspicious lesion','2026-08-05 10:00:00'),
 (9003,'2026-08-05',55,NULL,101,'100001',
  '1','AMT-eReferral',NULL,'Neurology referral -- recurring headaches','2026-08-05 10:05:00'),
 -- 9004: a native Oscar referral, NOT authored by AMT (source is Oscar's
 -- own default, not 'AMT-eReferral'). Paired with response 9103 below,
 -- which shares demographicNo/date but must still be excluded by the
 -- source filter -> D1 finds 0 eligible rows and dead-letters (A5), even
 -- though a same-day/same-patient request technically exists.
 (9004,'2026-08-06',56,NULL,102,'100001',
  '1',NULL,NULL,'Radiology referral -- entered directly in Oscar','2026-08-06 11:00:00');

-- ------------------------------------------------------- consultationRequestExt
-- Phase 1's three EAV rows per AMT-sourced request (EREF_SPEC.md E3),
-- only present for the genuinely AMT-authored requests (9001-9003; 9004 is
-- the native-Oscar request and correctly has none of these).
-- amt.fhirServiceRequestId is what D1's happy path resolves basedOn to.
INSERT INTO consultationRequestExt (requestId, name, value, dateCreated)
VALUES
 (9001,'amt.placerOrderId','AMT-9f2c1b7e-0001','2026-08-01'),
 (9001,'amt.fhirServiceRequestId','sr-9f2c1b7e-0001','2026-08-01'),
 (9001,'amt.sourceNode','clinic-a','2026-08-01'),
 (9002,'amt.placerOrderId','AMT-9f2c1b7e-0002','2026-08-05'),
 (9002,'amt.fhirServiceRequestId','sr-9f2c1b7e-0002','2026-08-05'),
 (9002,'amt.sourceNode','clinic-a','2026-08-05'),
 (9003,'amt.placerOrderId','AMT-9f2c1b7e-0003','2026-08-05'),
 (9003,'amt.fhirServiceRequestId','sr-9f2c1b7e-0003','2026-08-05'),
 (9003,'amt.sourceNode','clinic-a','2026-08-05');

-- ------------------------------------------------------- consultationResponse
-- `status` here is a PLACEHOLDER ('1') pending V1 (the mandatory empirical
-- gate): the real Oscar status codes for this table are
-- still unverified (R2 -- do not infer them from values seen in a table;
-- derive from Oscar's own source once V1 runs). Until
-- [oscar.consult_response_status_map] is populated with verified codes,
-- EVERY one of these rows will additionally dead-letter on status,
-- which is correct per D4 but means these fixtures alone cannot yet prove
-- A3/A5/A6 end-to-end -- they prove the correlation branch only once the
-- status map is real. Revisit this comment and the status value once V1
-- is done.
INSERT INTO consultationResponse
 (responseId, responseDate, referralDate, demographicNo, providerNo,
  status, examination, impression, plan)
VALUES
 -- 9101: answers 9001. Single match -> D1 resolves, basedOn ->
 -- sr-9f2c1b7e-0001 via 9001's amt.fhirServiceRequestId ext row.
 (9101,'2026-08-08','2026-08-01',101,'100001',
  '1','Chest auscultation clear, no murmurs.','Non-cardiac chest pain, likely musculoskeletal.',
  'Reassurance; return to referring physician for primary care follow-up.'),
 -- 9102: answers 9002/9003 (same demographicNo + date). Two eligible
 -- matches -> D1 must dead-letter AmbiguousOriginatingRequest, logging
 -- both 9002 and 9003, and must NOT guess.
 (9102,'2026-08-08','2026-08-05',101,'100001',
  '1','Skin exam performed.','Benign seborrheic keratosis, no biopsy indicated.',
  'Reassurance; routine skin checks going forward.'),
 -- 9103: answers 9004 (native-Oscar request) by demographicNo/date, but
 -- 9004 is excluded by the source filter -> D1 finds 0 eligible rows and
 -- dead-letters NoOriginatingRequest, even though a request "exists".
 (9103,'2026-08-09','2026-08-06',102,'100001',
  '1','Chest X-ray reviewed.','No acute findings.',
  'No follow-up required.'),
 -- 9104: answers nobody. demographicNo 103 has zero consultationRequests
 -- rows of any kind in this fixture set -> unambiguous 0-match case,
 -- dead-letters NoOriginatingRequest, simplest possible instance.
 (9104,'2026-08-08','2026-08-07',103,'100003',
  '1','Medication list reviewed.','Polypharmacy, no acute concerns.',
  'Continue current regimen; reassess in 6 months.');

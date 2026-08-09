-- One-shot remediation for AMT-authored consultationRequests rows written
-- before the Hibernate-mandatory column fix.
--
-- Scope: only rows authored by fhir-sync (source = 'AMT-eReferral') that
-- currently have NULL in the two columns Oscar cannot load/render safely.
--
-- patientWillBook is a primitive boolean in Oscar's entity; a NULL there
-- causes IllegalArgumentException at entity materialization.
-- urgency is dereferenced without a null guard in DisplayDemographicConsultationRequests.jsp.
--
-- Running this more than once is idempotent: a second run should report 0 rows affected.

UPDATE consultationRequests
   SET patientWillBook = COALESCE(patientWillBook, 0),
       urgency         = COALESCE(urgency, '2')
 WHERE source = 'AMT-eReferral'
   AND (patientWillBook IS NULL OR urgency IS NULL);

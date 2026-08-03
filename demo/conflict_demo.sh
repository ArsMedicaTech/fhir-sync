#!/usr/bin/env bash
set -euo pipefail

BASE_A="http://hapi-a:8080/fhir"
BASE_B="http://hapi-b:8080/fhir"
PATIENT_ID="p1"

update_patient() {
  local base=$1
  local given=$2
  local family=$3
  cat <<EOF
{
  "resourceType": "Patient",
  "id": "$PATIENT_ID",
  "identifier": [{"system": "https://arsmedicatech.com/fhir/sid/demo", "value": "demo-1"}],
  "active": true,
  "name": [{"family": "$family", "given": ["$given"]}],
  "gender": "unknown",
  "birthDate": "1990-01-01"
}
EOF
}

echo "1. Update Patient $PATIENT_ID on A..."
curl -s -X PUT -H "Content-Type: application/fhir+json" \
  -d "$(update_patient "$BASE_A" "Alice" "A-version")" \
  "$BASE_A/Patient/$PATIENT_ID" >/dev/null

echo "2. Wait for A -> B sync..."
sleep 12

echo "3. Update the same Patient on B (independent edit)..."
curl -s -X PUT -H "Content-Type: application/fhir+json" \
  -d "$(update_patient "$BASE_B" "Bob" "B-version")" \
  "$BASE_B/Patient/$PATIENT_ID" >/dev/null

echo "4. Update the same Patient on A again..."
curl -s -X PUT -H "Content-Type: application/fhir+json" \
  -d "$(update_patient "$BASE_A" "Alice2" "A-version2")" \
  "$BASE_A/Patient/$PATIENT_ID" >/dev/null

echo "5. Wait for B -> A to detect the conflict..."
sleep 12

echo "6. Conflict record(s) on the b-to-a link:"
docker compose -f docker-compose.replication.yml exec -T fhir-sync \
  cat "/var/lib/fhir-sync/replication/b-to-a/conflicts.jsonl" 2>/dev/null || echo "no conflict file yet"

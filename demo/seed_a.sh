#!/usr/bin/env bash
set -euo pipefail

BASE_A="${FHIR_A:-http://hapi-a:8080/fhir}"

create_patient() {
  local n=$1
  cat <<EOF
{
  "resourceType": "Patient",
  "id": "p$n",
  "identifier": [{"system": "https://arsmedicatech.com/fhir/sid/demo", "value": "demo-$n"}],
  "active": true,
  "name": [{"family": "Demo$n", "given": ["Test$n"]}],
  "gender": "unknown",
  "birthDate": "1990-01-01"
}
EOF
}

if command -v fhir-populate >/dev/null 2>&1; then
  fhir-populate --base "$BASE_A" --count 50
else
  # Fallback: post a minimal bundle of 50 Patients.
  echo "fhir-populate not found; posting 50 demo Patients via curl..."
  for i in $(seq 1 50); do
    curl -s -X PUT -H "Content-Type: application/fhir+json" -d "$(create_patient $i)" "$BASE_A/Patient/p$i" >/dev/null || true
  done
  echo "done"
fi

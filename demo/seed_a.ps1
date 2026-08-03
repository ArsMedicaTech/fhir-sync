$base = "http://localhost:8082/fhir"
1..50 | ForEach-Object {
  $body = @{
    resourceType = "Patient"
    id = "p$_"
    identifier = @(@{ system = "https://arsmedicatech.com/fhir/sid/demo"; value = "demo-$_" })
    active = $true
    name = @(@{ family = "Demo$_"; given = @("Test$_") })
    gender = "unknown"
    birthDate = "1990-01-01"
  } | ConvertTo-Json -Depth 6
  Invoke-RestMethod -Method Put -Uri "$base/Patient/p$_" `
    -ContentType "application/fhir+json" -Body $body | Out-Null
}
"seeded 50"

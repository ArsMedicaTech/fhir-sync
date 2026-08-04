$ErrorActionPreference = "Stop"

$populateHome = if ($env:FHIR_POPULATE_HOME) { $env:FHIR_POPULATE_HOME } else { Join-Path $PSScriptRoot "..\..\fhir-populate" }
$mainPy = Join-Path $populateHome "main.py"
$configFile = Join-Path $populateHome "config_low_volume.json"

if (-not (Test-Path $configFile)) {
    throw "Low-volume fhir-populate config not found: $configFile"
}

$command = $null
$arguments = @()
if (Get-Command fhir-populate -ErrorAction SilentlyContinue) {
    $command = "fhir-populate"
} elseif (Test-Path $mainPy) {
    $command = "python"
    $arguments = @($mainPy)
} else {
    throw "fhir-populate not found on PATH and `$env:FHIR_POPULATE_HOME` is not set to a valid directory"
}

$env:FHIR_CONFIG = $configFile

$targets = @(
    @{ Host = "localhost"; Port = 8082; Path = "fhir" },
    @{ Host = "localhost"; Port = 8083; Path = "fhir" }
)

foreach ($t in $targets) {
    $base = "http://$($t.Host):$($t.Port)/$($t.Path)"
    $count = (Invoke-RestMethod -Uri "$base/Patient?_summary=count" -UseBasicParsing).total
    if ($count -ge 3) {
        Write-Output "Node $($t.Host):$($t.Port) already has $count Patient(s); skipping"
        continue
    }

    $env:FHIR_HOST = $t.Host
    $env:FHIR_PORT = [string]$t.Port
    $env:FHIR_PATH = "/$($t.Path)"

    & $command @arguments
}

Write-Output "seeded ereferral"

param(
  [Parameter(Mandatory=$true)]
  [ValidateSet("python", "dart", "go", "js", "ts", "cpp", "rust")]
  [string]$lang
)

$base = (Get-Location).Path -replace '\\', '/'

# Include all *.proto except those in proto/google/protobuf/*
$protoFiles = Get-ChildItem -Recurse -Filter *.proto -Path ./proto |
  Where-Object {
    $_.FullName -notmatch 'proto[\\/]+google[\\/]+protobuf[\\/]+'
  } |
  ForEach-Object {
    $_.FullName.Replace('\', '/').Replace($base, '/work')
  }

Write-Host "Proto files: $protoFiles"

# Create output dir if needed
# reusable function
function Create-OutputDir {
  param (
    [string]$lang
  )
  $outDir = "gen/$lang/client"
  if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir | Out-Null
  }
}

# Set language-specific output option
switch ($lang) {
  "python" {
    $outFlag = "--python_out=/work/gen/python/client --pyi_out=/work/gen/python/client"
    $outFlagService = "--grpc_python_out=/work/gen/python/client"
    Create-OutputDir -lang "python"
  }
  "dart" {
    $outFlag = "--dart_out=grpc:/work/gen/dart/client"
    Create-OutputDir -lang "dart"
  }
  "go" {
    $outFlag = "--go_out=/work/gen/go/client --go-grpc_out=/work/gen/go/client"
    Create-OutputDir -lang "go"
  }
  "js" {
    $outFlag = "--js_out=import_style=commonjs:/work/gen/js/client"
    Create-OutputDir -lang "js"
  }
  "ts" {
    $outFlag = "--plugin=protoc-gen-ts=/usr/bin/protoc-gen-ts --ts_out=/work/gen/ts/client"
    Create-OutputDir -lang "ts"
  }
  "cpp" {
    $outFlag = "--cpp_out=/work/gen/cpp/client"
    Create-OutputDir -lang "cpp"
  }
  "rust" {
    $outFlag = "--plugin=protoc-gen-rust=/usr/bin/protoc-gen-rust --rust_out=/work/gen/rust/client"
    Create-OutputDir -lang "rust"
  }
  default {
    Write-Error "Unsupported language: $lang"
    exit 1
  }
}

docker run --rm -v "${PWD}:/work" -w /work protoc `
  --proto_path=/work/proto `
  --proto_path=/usr/include `
  $outFlag `
  $protoFiles

$hostPath = $pwd.Path -replace '\\','/'
if ($lang -eq "python") {
  $protoPath = "proto"
  $outDir = "gen/python/client"
  $protoFile = "proto/arsmedicatech/fhir_sync.proto"

  docker run --rm -v "${hostPath}:/work" -w /work python:3.11 bash -c "pip install grpcio-tools && python -m grpc_tools.protoc --proto_path=$protoPath --python_out=$outDir --grpc_python_out=$outDir $protoFile"
}

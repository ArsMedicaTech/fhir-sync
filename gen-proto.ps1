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


# Create output dir if needed
$outDir = "gen/client"
if (-not (Test-Path $outDir)) {
  New-Item -ItemType Directory -Path $outDir | Out-Null
}

# Set language-specific output option
switch ($lang) {
  "python" {
    $outFlag = "--python_out=/work/gen/client"
  }
  "dart" {
    $outFlag = "--dart_out=grpc:/work/gen/client"
  }
  "go" {
    $outFlag = "--go_out=/work/gen/client --go-grpc_out=/work/gen/client"
  }
  "js" {
    $outFlag = "--js_out=import_style=commonjs:/work/gen/client"
  }
  "ts" {
    $outFlag = "--plugin=protoc-gen-ts=/usr/bin/protoc-gen-ts --ts_out=/work/gen/client"
  }
  "cpp" {
    $outFlag = "--cpp_out=/work/gen/client"
  }
  "rust" {
    $outFlag = "--plugin=protoc-gen-rust=/usr/bin/protoc-gen-rust --rust_out=/work/gen/client"
  }
  default {
    Write-Error "Unsupported language: $lang"
    exit 1
  }
}

docker run --rm -v "${PWD}:/work" -w /work rvolosatovs/protoc `
  --proto_path=/work/proto `
  --proto_path=/usr/include `
  $outFlag `
  $protoFiles

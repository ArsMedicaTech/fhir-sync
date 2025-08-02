$base = (Get-Location).Path -replace '\\', '/'

# Include all *.proto except those in proto/google/protobuf/*
$protoFiles = Get-ChildItem -Recurse -Filter *.proto -Path ./proto |
  Where-Object {
    $_.FullName -notmatch 'proto[\\/]+google[\\/]+protobuf[\\/]+'
  } |
  ForEach-Object {
    $_.FullName.Replace('\', '/').Replace($base, '/work')
  }

if (-not (Test-Path "gen/client")) {
  New-Item -ItemType Directory -Path "gen/client" | Out-Null
}

docker run --rm -v "${PWD}:/work" -w /work rvolosatovs/protoc `
  --proto_path=/work/proto `
  --proto_path=/usr/include `
  --python_out=/work/gen/client `
  $protoFiles

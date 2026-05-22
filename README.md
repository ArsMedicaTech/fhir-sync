# fhir-sync

## gRPC

Generate client stubs for Python: `docker run --rm -v "${PWD}:/work" -w /work rvolosatovs/protoc --python_out=gen/client -I/work/proto /work/proto`.

Generate client stubs for Dart: `docker run --rm -v "${PWD}:/work" -w /work rvolosatovs/protoc --dart_out=grpc:/work/gen/client -I/work/proto /work/proto/feed.proto`.

### TODO

Generate client stubs for the following:
* JS.
* Typescript.
* Rust.



arsmedicatech_fhir_sync




Enable binlog_format=ROW in your MariaDB config (/etc/mysql/my.cnf or my.cnf.d/mariadb.cnf).

Use a binlog reader in your FHIR Sync service (or a sidecar service).

Important: Create a read-only cdc_user with REPLICATION SLAVE and REPLICATION CLIENT privileges.




# DevContainer

rustup update stable




docker create --name temp-logs arsmedicatech_fhir_sync
docker cp temp-logs:/logs ./build-logs
docker rm temp-logs

RUN cp -r build-logs /tmp/logs || echo "no logs found"
# cp: cannot stat 'build-logs': No such file or directory

Step 22/24 : COPY --from=builder /tmp/logs /logs
COPY failed: stat tmp/logs: file does not exist



# Gotchas

ALWAYS REMEMBER TO ENSURE THERE IS A `src/proto` DIRECTORY WHEN RUNNING `build.rs`!!!

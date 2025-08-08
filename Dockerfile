FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app

# Install MUSL target and other build deps
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools protobuf-compiler libprotobuf-dev

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

ENV PROTOBUF_LOCATION=/usr
ENV PROTOC=$PROTOBUF_LOCATION/bin/protoc
ENV PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
#ENV PATH="/usr/bin:${PATH}"
ENV PATH="/app/target/release:$PATH"

# Cache dependencies
COPY Cargo.toml ./

# Create a dummy src file so `cargo build` doesn't fail
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo generate-lockfile

# Compile only dependencies (caches this layer if only src/ changes later)
RUN cargo build --release --target x86_64-unknown-linux-musl && rm -rf src

COPY ./build.rs ./build.rs

# Copy actual source files and recompile only your crate
COPY ./src ./src
COPY ./proto ./proto

RUN protoc --proto_path=proto --proto_path=proto/google/fhir/proto --proto_path=/usr/include --descriptor_set_out=/dev/null proto/fhir_sync.proto

RUN mkdir -p src/proto
ENV TONIC_BUILD_VERBOSE=1

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN cp -r build-logs /tmp/logs || echo "no logs found"

# ---- LOG COPY STAGE ----
FROM busybox AS logs
COPY --from=builder /tmp/logs /logs


FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fhir-sync /fhir-sync
ENTRYPOINT ["/fhir-sync"]

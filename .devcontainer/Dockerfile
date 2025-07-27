FROM rust:1.82-slim

WORKDIR /workspaces/fhir-sync

RUN apt-get update && apt-get install -y git

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

ENV PROTOBUF_LOCATION=/usr
ENV PROTOC=$PROTOBUF_LOCATION/bin/protoc
ENV PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
#ENV PATH="/usr/bin:${PATH}"
ENV PATH="/app/target/release:$PATH"

# Cache dependencies
COPY Cargo-copy.toml ./Cargo.toml

# Create a dummy src file so `cargo build` doesn't fail
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo generate-lockfile

# Compile only dependencies (caches this layer if only src/ changes later)
RUN cargo build --release && rm -rf src

# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:1.64 AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasip2
EOT

FROM buildbase AS build
COPY Cargo.toml .
COPY src ./src

# Build the Wasm binary
RUN cargo build --target wasm32-wasip2 --release

FROM scratch
ENTRYPOINT [ "/webserver.wasm" ]
COPY --link --from=build /target/wasm32-wasip2/release/webserver.wasm /webserver.wasm
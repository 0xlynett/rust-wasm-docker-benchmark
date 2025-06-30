# syntax=docker/dockerfile:1
# Docker + WASM doesn't work on my machine, but kept in the hopes someone will make it work. -Lyn

FROM --platform=$BUILDPLATFORM rust:1.88 AS buildbase
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
ENTRYPOINT [ "/fib.wasm" ]
COPY --link --from=build /src/target/wasm32-wasip2/release/fib.wasm /fib.wasm
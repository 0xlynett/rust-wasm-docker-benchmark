# Benchmarking WebAssembly, Docker & Native Execution

- HTTP server with JSON parsing
- Fibonacci 47 (recursion)

## Commands

### Fibonacci (run inside the directory)

```
docker buildx build -t fibonacci-regular .

# Doesn't work on my machine. -Lyn
docker buildx build -t fibonacci-wasm . -f wasm.dockerfile --platform wasi/wasm

cargo build --release --target=x86_64-unknown-linux-musl

# Requires you to install the wasm32-wasip2 toolchain
cargo build --release --target=wasm32-wasip2
```

### Webserver (run inside the directory)

```
docker buildx build -t webserver-regular .

# Doesn't work on my machine. -Lyn
docker buildx build -t webserver-wasm . -f wasm.dockerfile --platform wasi/wasm

cargo build --release --target=x86_64-unknown-linux-musl

# Requires you to install the wasm32-wasip2 toolchain
cargo build --release --target=wasm32-wasip2
```

## Benchmarking (Fibonacci)

```
docker buildx build -t fibonacci-regular ./fibonacci && \
cargo build --release -p fib --target x86_64-unknown-linux-musl --target wasm32-wasip2 && \
hyperfine "docker run --rm fibonacci-regular" "./target/x86_64-unknown-linux-musl/release/fib" "wasmtime ./target/wasm32-wasip2/release/fib.wasm"
```
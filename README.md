# Benchmarking WebAssembly, Docker & Native Execution

- HTTP server with JSON parsing
- Fibonacci 47 (recursion)

## Commands

### Fibonacci (run inside the directory)

```
docker buildx build -t fibonacci-regular .
docker buildx build -t fibonacci-wasm . -f wasm.dockerfile

cargo build --release

# Requires you to install the wasm32-wasip2 toolchain
cargo build --release --target=wasm32-wasip2
```

### Webserver (run inside the directory)

```
docker buildx build -t webserver-regular .
docker buildx build -t webserver-wasm . -f wasm.dockerfile

cargo build --release

# Requires you to install the wasm32-wasip2 toolchain
cargo build --release --target=wasm32-wasip2
```
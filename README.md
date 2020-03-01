A simple Game of Life implementation for the browser in Rust, compiled to WASM!

To install & run:

```
cargo install wasm-pack
make
```

and visit http://localhost:8080

Can also be ran using WASI with wasmtime:
```
cargo build --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/wasm-gameoflife.wasm
```

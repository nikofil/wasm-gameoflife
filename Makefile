run:
	wasm-pack build
	cd www && npm i && npm run build && npm run start

wasi:
	cargo build --target wasm32-wasi
	wasmtime target/wasm32-wasi/debug/wasm-gameoflife.wasm

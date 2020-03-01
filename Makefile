run:
	RUSTFLAGS='-C target-feature=+simd128' wasm-pack build
	cd www && npm i && npm run build && npm run start

wasi:
	RUSTFLAGS='-C target-feature=+simd128' cargo +nightly build --target wasm32-wasi
	wasmer --enable-simd --backend llvm target/wasm32-wasi/debug/wasm-gameoflife.wasm

brave:
	brave-browser --js-flags="--experimental-wasm-simd"

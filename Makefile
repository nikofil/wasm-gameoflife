run:
	wasm-pack build
	cd www && npm i && npm run build && npm run start

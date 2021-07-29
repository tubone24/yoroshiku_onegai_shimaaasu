install-wasm:
	cargo install wasm-pack

build:
	wasm-pack build --target web

run-local:
	python -m http.server 8080

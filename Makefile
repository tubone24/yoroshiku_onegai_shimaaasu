install-wasm:
	cargo install wasm-pack

build:
	wasm-pack build --target web

run-local:
	cargo run --bin simple_server

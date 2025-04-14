.PHONY: auto-gen build run

default: auto-gen build run

auto-gen:
	@cargo run --manifest-path auto-gen/Cargo.toml

dev-build:
	@cd frontend && wasm-pack build --target web

build:
	@cd frontend && wasm-pack build --release --target web

run:
	@cd frontend && python3 -m http.server

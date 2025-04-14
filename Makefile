.PHONY: auto-gen build run

default: auto-gen build run

auto-gen:
	@cargo run --manifest-path auto-gen/Cargo.toml

build:
	@cd frontend && wasm-pack build --target web

run:
	@cd frontend && python3 -m http.server

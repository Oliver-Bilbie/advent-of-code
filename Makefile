.PHONY: auto-gen build run

default: auto-gen build run

auto-gen:
	@cargo run --manifest-path auto-gen/Cargo.toml

dev-build:
	@cd frontend && wasm-pack build --target web

terraform:
	@echo "[INFO] Deploying infrastructure"
	@cd ./frontend/terraform && terraform init
	@cd ./frontend/terraform && terraform apply
	@echo "[INFO] Infrastructure deployed 🚀"

build:
	@cd frontend && wasm-pack build --release --target web

run:
	@cd frontend && python3 -m http.server

.PHONY: build run terraform

default: build terraform deploy

run:
	@cd frontend && npx http-server . -c-1

terraform:
	@cd ./terraform && terraform init && terraform apply

build:
	@cargo run --manifest-path build_utils/Cargo.toml

deploy:
	@echo "[INFO] Uploading client files"
	@aws s3 sync --delete ./frontend s3://aoc-solver-host-bucket
	@echo "[INFO] Resetting CDN cache"
	@aws cloudfront create-invalidation --distribution-id $(shell cd terraform && terraform output -raw cloudfront_distribution) --paths "/*"
	@echo "[INFO] Client deployed 🚀"

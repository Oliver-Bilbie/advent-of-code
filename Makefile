.PHONY: build run terraform

default: build minify terraform deploy

new:
	@cd init_utils && go run .

run:
	@cd frontend && npx http-server . -c-1

terraform:
	@cd ./terraform && terraform init && terraform apply

build:
	@cargo run --manifest-path build_utils/Cargo.toml
	@rm -rf build
	@cp -r frontend build
	@rm -r build/package.json build/package-lock.json build/node_modules

minify:
	find build -path "build/node_modules" -prune -o -type f -name "*.js" -exec sh -c '\
	  for file do \
	    npx terser "$$file" -c -m -o "$$file"; \
	    echo "Minified: $$file"; \
	  done \
	' sh {} +

deploy:
	@echo "[INFO] Uploading client files"
	@aws s3 sync --delete ./build s3://aoc-solver-host-bucket
	@echo "[INFO] Resetting CDN cache"
	@aws cloudfront create-invalidation --distribution-id $(shell cd terraform && terraform output -raw cloudfront_distribution) --paths "/*"
	@echo "[INFO] Client deployed 🚀"

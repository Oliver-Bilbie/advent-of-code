.PHONY: auto-gen build run

default: auto-gen terraform build deploy

start: auto-gen dev-build run

auto-gen:
	@cargo run --manifest-path auto-gen/Cargo.toml

dev-build:
	@cd frontend && wasm-pack build --target web

run:
	@cd frontend && python3 -m http.server

terraform:
	@echo "[INFO] Deploying infrastructure"
	@cd ./frontend/terraform && terraform init
	@cd ./frontend/terraform && terraform apply
	@echo "[INFO] Infrastructure deployed 🚀"

build:
	@echo "[INFO] Compiling solutions to WASM"
	@cd frontend && wasm-pack build --release --target web

	@echo "[INFO] Building client files"
	@rm -rf ./build
	@mkdir -p ./build
	@cp -r ./frontend/* ./build
	@rm -rf ./build/terraform ./build/src ./build/Cargo.toml

	@echo "[INFO] Compressing client files"
	@find build -type f \( -name "*.js" -o -name "*.wasm" -o -name "*.css" -o -name "*.html" \) | while read file; do \
		brotli -f -q 11 "$$file" -o "$$file.br"; \
		gzip -f -k "$$file"; \
	done

	@echo "[INFO] Client has been built"

deploy:
	@for enc in br gz; do \
		echo "[INFO] Uploading $$enc encoded files individually with correct headers"; \
		find build -type f -name "*.$$enc" | while read file; do \
			origfile=$${file%.$$enc}; \
			relpath=$${origfile#build/}; \
			mime=$$(file --mime-type -b "$$origfile"); \
			encoding=$$( [ "$$enc" = "br" ] && echo br || echo gzip ); \
			aws s3 cp "$$file" "s3://aoc-solver-host-bucket/$$relpath" \
				--content-encoding "$$encoding" \
				--content-type "$$mime" \
				--metadata-directive REPLACE; \
		done; \
	done

	@echo "[INFO] Uploading client files"
	@aws s3 sync --delete ./build s3://aoc-solver-host-bucket

	@echo "[INFO] Resetting CDN cache"
	@aws cloudfront create-invalidation --distribution-id $(shell cd frontend/terraform && terraform output -raw cloudfront_distribution) --paths "/*"

	@echo "[INFO] Client deployed 🚀"

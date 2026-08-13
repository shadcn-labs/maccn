.PHONY: help dev build-wasm build clean install watch-wasm

help: ## Show help information
	@echo "maccn WASM demo - Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

install: ## Install Rust WASM target
	@echo "Checking Rust WASM target..."
	@rustup target add wasm32-unknown-unknown || true
	@echo "Checking wasm-bindgen-cli..."
	@cargo install wasm-bindgen-cli || true

build-wasm: ## Build WASM (release mode)
	@./scripts/build.sh --release

build-wasm-dev: ## Build WASM (debug mode)
	@./scripts/build.sh

build: build-wasm ## Build WASM (release) into docs/public/examples/

dev: build-wasm-dev ## Build WASM (debug) into docs/public/examples/
	@echo "WASM built. Run 'pnpm dev' in the docs/ directory to start Next.js."

clean: ## Clean build artifacts
	@rm -rf ../../../../docs/public/examples/wasm
	@rm -f ../../../../docs/public/examples/index.html
	@rm -f ../../../../docs/public/examples/main.js
	@cargo clean

watch-wasm: ## Watch Rust code changes and auto-rebuild WASM
	@cargo watch -x 'build --target wasm32-unknown-unknown' -s './scripts/build.sh'

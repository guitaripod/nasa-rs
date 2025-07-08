.PHONY: help build test deploy clean dev lint format

# Default target
help:
	@echo "NASA-RS Development Commands"
	@echo "============================"
	@echo "  make build         - Build both worker and CLI"
	@echo "  make build-worker  - Build worker only"
	@echo "  make build-cli     - Build CLI only"
	@echo "  make test          - Run all tests"
	@echo "  make test-unit     - Run unit tests"
	@echo "  make test-worker   - Test worker endpoints"
	@echo "  make test-cli      - Test CLI commands"
	@echo "  make dev           - Start local development server"
	@echo "  make deploy        - Deploy to Cloudflare Workers"
	@echo "  make lint          - Run clippy linter"
	@echo "  make format        - Format code with rustfmt"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make install-cli   - Install CLI locally"

# Build targets
build: build-worker build-cli

build-worker:
	@echo "Building worker..."
	@cargo build --release

build-cli:
	@echo "Building CLI..."
	@cargo build --release --features cli

# Test targets
test: test-unit test-worker test-cli

test-unit:
	@echo "Running unit tests..."
	@cargo test --all-features

test-worker:
	@echo "Testing worker endpoints..."
	@./scripts/test_endpoints.sh

test-cli: build-cli
	@echo "Testing CLI commands..."
	@./scripts/test_cli.sh

# Development targets
dev:
	@echo "Starting local development server..."
	@wrangler dev

deploy:
	@echo "Deploying to Cloudflare Workers..."
	@wrangler deploy

# Code quality targets
lint:
	@echo "Running clippy..."
	@cargo clippy --all-features -- -D warnings

format:
	@echo "Formatting code..."
	@cargo fmt

format-check:
	@echo "Checking code formatting..."
	@cargo fmt -- --check

# Installation
install-cli: build-cli
	@echo "Installing NASA CLI..."
	@cargo install --path . --features cli

# Cleanup
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target/

# Quick commands for common tasks
quick-test-apod:
	@echo "Quick test: APOD endpoint"
	@curl -s "http://localhost:8787/api/apod?api_key=DEMO_KEY" | jq .

quick-test-cli-apod: build-cli
	@echo "Quick test: CLI APOD command"
	@./target/release/nasa-cli apod today -o pretty

# Development setup
setup:
	@echo "Setting up development environment..."
	@npm install -g wrangler
	@cargo install cargo-watch
	@echo "Development environment ready!"

# Watch for changes and rebuild
watch:
	@cargo watch -x "build --all-features"

# Generate documentation
docs:
	@echo "Generating documentation..."
	@cargo doc --all-features --no-deps --open

# Check everything before committing
pre-commit: format-check lint test
	@echo "All checks passed! Ready to commit."
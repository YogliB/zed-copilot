.PHONY: fmt clippy test check-all build release clean help

help:
	@echo "Zed Copilot - Available commands:"
	@echo ""
	@echo "  make fmt          - Format code with rustfmt"
	@echo "  make clippy       - Lint code with clippy"
	@echo "  make test         - Run all tests"
	@echo "  make check-all    - Run fmt, clippy, and test (quality checks)"
	@echo "  make build        - Build debug binary"
	@echo "  make release      - Build optimized release binary"
	@echo "  make clean        - Remove build artifacts"
	@echo ""

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

test:
	cargo test

check-all: fmt clippy test
	@echo "✅ All quality checks passed!"

build:
	cargo build --target wasm32-unknown-unknown

release:
	cargo build --release --target wasm32-unknown-unknown

clean:
	cargo clean
	rm -rf target/

.DEFAULT_GOAL := help

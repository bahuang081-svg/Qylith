# Qylith Makefile
# Common build and development tasks

.PHONY: help build check test fmt clippy clean lint benchmark docker-build docker-run
.PHONY: genesis dev spec prune purge clean-all

# Default target
help:
	@echo "Qylith Blockchain - Available targets:"
	@echo ""
	@echo "  build          - Build the Qylith node (release)"
	@echo "  check          - Check the code (faster than build)"
	@echo "  test           - Run tests"
	@echo "  fmt            - Format code"
	@echo "  clippy         - Run linter"
	@echo "  lint           - Run all linting checks"
	@echo "  clean          - Clean build artifacts"
	@echo "  benchmark      - Run benchmarks"
	@echo "  docker-build   - Build Docker image"
	@echo "  docker-run     - Run Docker container"
	@echo "  genesis        - Generate genesis specification"
	@echo "  dev            - Run in development mode"
	@echo "  purge          - Purge local chain data"

# Build the node
build:
	@echo "Building Qylith node..."
	cargo build --release --bin qylith
	@echo "Build complete: target/release/qylith"

# Check the code (faster than build)
check:
	@echo "Checking Qylith code..."
	cargo check --workspace
	@echo "Check complete"

# Run tests
test:
	@echo "Running tests..."
	cargo test --workspace --all-features
	@echo "Tests complete"

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all
	@echo "Format complete"

# Run clippy linter
clippy:
	@echo "Running Clippy..."
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "Clippy complete"

# Run all linting
lint: fmt clippy
	@echo "All linting complete"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Clean complete"

# Run benchmarks
benchmark:
	@echo "Running benchmarks..."
	cargo bench --workspace
	@echo "Benchmarks complete"

# Build Docker image
docker-build:
	@echo "Building Docker image..."
	docker build -t qylith:latest .
	@echo "Docker build complete"

# Run Docker container
docker-run:
	@echo "Running Qylith node in Docker..."
	docker run -p 30333:30333 -p 9933:9933 -p 9944:9944 -v qylith-data:/qylith/data qylith:latest

# Generate genesis specification
genesis:
	@echo "Generating genesis specification..."
	./target/release/qylith build-spec --chain development > scripts/genesis-spec.json
	@echo "Genesis specification generated"

# Run in development mode
dev:
	@echo "Starting Qylith in development mode..."
	RUST_LOG=debug ./target/release/qylith --dev --tmp

# Purge local chain data
purge:
	@echo "Purging local chain data..."
	./target/release/qylith purge-chain --dev
	@echo "Purge complete"

# Clean everything including target
clean-all: clean
	@echo "Removing target directory..."
	rm -rf target/
	@echo "Clean all complete"

# Install prerequisites
install-deps:
	@echo "Installing prerequisites..."
	@if ! command -v rustup &> /dev/null; then \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
	fi
	@echo "Setting up Rust..."
	@source ~/.cargo/env 2>/dev/null || true
	rustup default nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
	@echo "Prerequisites installed"

# Verify WASM build
check-wasm:
	@echo "Checking WASM build..."
	cargo build --release -p qylith-runtime
	@echo "WASM build check complete"

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps
	@echo "Documentation generated in target/doc/"

# Run with specific chain
run-local:
	@echo "Starting Qylith with local chain..."
	./target/release/qylith --chain local

# Update Substrate dependencies
update-deps:
	@echo "Updating dependencies..."
	cargo update
	@echo "Dependencies updated"

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit
	@echo "Audit complete"

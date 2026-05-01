#!/bin/sh

# Qylith build script
# Builds the Qylith node

set -e

echo "Building Qylith node..."

# Check for Rust
if ! command -v rustup &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Ensure we have nightly for wasm
rustup default nightly

# Add wasm target
rustup target add wasm32-unknown-unknown --toolchain nightly

# Build the node
echo "Compiling Qylith node..."
cargo build --release --bin qylith 2>&1 | head -50

echo "Build complete!"
echo ""
echo "Binary location: target/release/qylith"

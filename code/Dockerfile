# Qylith Node Docker Image
# Multi-stage build for optimal image size

# Stage 1: Build
FROM rust:1.75 as builder

WORKDIR /qylith

# Install dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libssl-dev \
    pkg-config \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Set up Rust
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

# Copy source
COPY . .

# Build Qylith runtime
RUN cargo build --release -p qylith-runtime

# Build Qylith node
RUN cargo build --release --bin qylith

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /qylith

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=builder /qylith/target/release/qylith /usr/local/bin/
COPY --from=builder /qylith/target/release/wbuild/qylith_runtime.compact.compressed.wasm /qylith/

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash qylith

# Set ownership
RUN chown -R qylith:qylith /qylith

# Switch to non-root user
USER qylith

# Expose ports
EXPOSE 30333 9933 9944 9615

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9933/health || exit 1

# Default command
CMD ["qylith", "--help"]

# Edge Hive - Dockerfile
FROM rust:1.75-bookworm as builder

WORKDIR /app

# Copy workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY edge-hive-admin/ edge-hive-admin/

# Build release
RUN cargo build --release -p edge-hive-core

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/edge-hive-core /usr/local/bin/

# Create data directory
RUN mkdir -p /data/edge-hive

# Environment
ENV EDGE_HIVE_DATA_DIR=/data/edge-hive
ENV RUST_LOG=info

# Expose ports
EXPOSE 8080 4433

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
    CMD curl -f http://localhost:8080/health || exit 1

# Run
CMD ["edge-hive-core", "serve", "--host", "0.0.0.0", "--port", "8080"]
# Multi-stage build for Edge Hive with Astro frontend

# Stage 1: Build Astro frontend
FROM node:20-alpine AS frontend-builder

WORKDIR /app/frontend

# Copy frontend package files
COPY app/package*.json ./
RUN npm ci

# Copy frontend source and build
COPY app/ ./
RUN npm run build

# Stage 2: Build Rust backend
FROM rust:1.90-alpine AS backend-builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /build

# Copy workspace files (excluding Tauri app)
COPY Cargo.toml Cargo.lock ./

# Create placeholder for Tauri crate to satisfy workspace resolver
RUN mkdir -p app/src-tauri/src && \
    cat > app/src-tauri/Cargo.toml <<'EOF'
[package]
name = "edge-hive-tauri"
version = "0.1.0"
edition = "2021"
EOF

RUN echo 'fn main() {}' > app/src-tauri/src/main.rs

# Copy actual source code
COPY src ./src
COPY crates ./crates

# Build release binary (static linking for musl) - only edge-hive binary, not tauri
RUN cargo build --release --package edge-hive --bin edge-hive --target x86_64-unknown-linux-musl

# Stage 3: Runtime image
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    wget \
    curl

# Create non-root user
RUN addgroup -g 1000 edgehive && \
    adduser -D -u 1000 -G edgehive edgehive

WORKDIR /app

# Copy backend binary from builder
COPY --from=backend-builder /build/target/x86_64-unknown-linux-musl/release/edge-hive /usr/local/bin/edge-hive

# Copy frontend build from frontend-builder
COPY --from=frontend-builder /app/frontend/dist ./app/dist

# Set ownership and permissions
RUN chown edgehive:edgehive /usr/local/bin/edge-hive && \
    chmod +x /usr/local/bin/edge-hive && \
    chown -R edgehive:edgehive /app

# Create data directory for persistent storage
RUN mkdir -p /data/.edge-hive && \
    chown -R edgehive:edgehive /data

# Switch to non-root user
USER edgehive
WORKDIR /app

# Expose ports
# 8080: HTTP/HTTPS API (MCP server)
# 4001: libp2p QUIC
# 5353: mDNS (service discovery)
EXPOSE 8080/tcp
EXPOSE 4001/udp
EXPOSE 5353/udp

# Environment variables
ENV RUST_LOG=info,edge_hive=debug

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Default command: start server with discovery
ENTRYPOINT ["/usr/local/bin/edge-hive"]
CMD ["--config-dir", "/data/.edge-hive", "start", "--port", "8080", "--discovery"]

# Multi-stage build for minimal Docker image
FROM rust:1.85-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY app/src-tauri ./app/src-tauri

# Build release binary (static linking)
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage: minimal Alpine image
FROM alpine:3.19

# Install runtime dependencies (only Tor and CA certs)
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# Create non-root user
RUN addgroup -g 1000 edgehive && \
    adduser -D -u 1000 -G edgehive edgehive

# Copy binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/edge-hive /usr/local/bin/edge-hive

# Set ownership
RUN chown edgehive:edgehive /usr/local/bin/edge-hive && \
    chmod +x /usr/local/bin/edge-hive

# Create data directory
RUN mkdir -p /data/.edge-hive && \
    chown -R edgehive:edgehive /data

# Switch to non-root user
USER edgehive
WORKDIR /data

# Expose ports
# 8080: HTTP API
# 4001: libp2p QUIC
EXPOSE 8080 4001/udp

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Start Edge Hive
CMD ["edge-hive", "start", "--data-dir", "/data/.edge-hive"]

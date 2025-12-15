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

# Build release binary (static linking for musl)
RUN cargo build --release --package edge-hive-core --target x86_64-unknown-linux-musl

# Final stage: minimal Alpine image
FROM alpine:3.19

# Install runtime dependencies (only CA certs for HTTPS)
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# Create non-root user
RUN addgroup -g 1000 edgehive && \
    adduser -D -u 1000 -G edgehive edgehive

# Copy binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/edge-hive /usr/local/bin/edge-hive

# Set ownership and permissions
RUN chown edgehive:edgehive /usr/local/bin/edge-hive && \
    chmod +x /usr/local/bin/edge-hive

# Create data directory for persistent storage
RUN mkdir -p /data/.edge-hive && \
    chown -R edgehive:edgehive /data

# Switch to non-root user
USER edgehive
WORKDIR /data

# Expose ports
# 8080: HTTP/HTTPS API (MCP server)
# 4001: libp2p QUIC
# 5353: mDNS (service discovery)
EXPOSE 8080/tcp
EXPOSE 4001/udp
EXPOSE 5353/udp

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ["/usr/local/bin/edge-hive", "status", "--json"] || exit 1

# Environment variables
ENV RUST_LOG=info,edge_hive=debug \
    EDGE_HIVE_DATA_DIR=/data/.edge-hive

# Default command: start server with discovery
ENTRYPOINT ["/usr/local/bin/edge-hive"]
CMD ["serve", "--port", "8080", "--discovery"]
EXPOSE 8080 4001/udp

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Start Edge Hive
CMD ["edge-hive", "start", "--data-dir", "/data/.edge-hive"]

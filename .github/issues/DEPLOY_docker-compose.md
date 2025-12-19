---
title: "[DEPLOY] Docker Container Build & Compose"
labels:
  - deployment
  - docker
  - jules
  - MVP
assignees: []
---

## Description

Create Docker deployment for Edge Hive server components.

## Tasks

- [ ] Create `Dockerfile` for edge-hive-core server
- [ ] Create `docker-compose.yml` with all services
- [ ] Include SurrealDB, Redis, and edge-hive-core
- [ ] Add health checks
- [ ] Create `.env.example` for configuration
- [ ] Test local deployment
- [ ] Push to Docker Hub / GitHub Container Registry

## Docker Architecture

```yaml
# docker-compose.yml
version: '3.8'
services:
  edge-hive:
    build: .
    ports:
      - "8080:8080"
      - "4433:4433"  # QUIC
    environment:
      - DATABASE_URL=ws://surrealdb:8000
      - REDIS_URL=redis://redis:6379
    depends_on:
      - surrealdb
      - redis

  surrealdb:
    image: surrealdb/surrealdb:latest
    command: start --user root --pass root
    ports:
      - "8000:8000"
    volumes:
      - surrealdb_data:/data

  redis:
    image: redis:alpine
    ports:
      - "6379:6379"

volumes:
  surrealdb_data:
```

## Dockerfile

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p edge-hive-core

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/edge-hive-core /usr/local/bin/
EXPOSE 8080 4433
CMD ["edge-hive-core", "serve", "--host", "0.0.0.0"]
```

## Quick Start

```bash
# Clone and run
git clone https://github.com/user/edge-hive.git
cd edge-hive
docker-compose up -d

# Access
# API: http://localhost:8080
# SurrealDB: http://localhost:8000
```

## Acceptance Criteria

- [ ] `docker-compose up` starts all services
- [ ] Health checks pass
- [ ] API responds correctly
- [ ] Data persists across restarts

## Estimated Effort
3-4 hours

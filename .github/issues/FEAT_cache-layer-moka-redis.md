---
title: "Implement Cache Layer with Moka + Mini-Redis"
labels:
  - enhancement
  - performance
  - ai-plan
assignees: []
---

## Description

Implement a high-performance cache layer to replace external caching dependencies and achieve 100x performance improvement over direct database queries.

## Architecture

**Two-tier caching strategy:**

- **L1 Cache (moka)**: In-memory, nanosecond access
  - Max capacity: configurable (default 10,000 entries)
  - TTL: configurable per key
  - LRU eviction policy

- **L2 Cache (mini-redis)**: Redis protocol compatible, microsecond access
  - Optional persistent storage
  - Network-accessible for distributed caching
  - Compatible with existing Redis clients

## Technical Requirements

### Dependencies

```toml
[dependencies]
moka = { version = "0.12", features = ["future"] }
mini-redis = "0.4"
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
```

### Performance Targets

| Operation | Current (DB) | Target (Cache) | Improvement |
|-----------|--------------|----------------|-------------|
| Read hit | 5-50ms | 0.05-0.5ms | 100x |
| Write | 10-100ms | 0.1-1ms | 100x |
| Invalidation | N/A | <1ms | New feature |

## Implementation Tasks

- [ ] Create `crates/edge-hive-cache/` crate structure
- [ ] Implement L1 cache with moka (in-memory)
- [ ] Implement L2 cache with mini-redis (optional)
- [ ] Add cache invalidation strategies (TTL, manual, pattern-based)
- [ ] Integrate with DatabaseService for transparent caching
- [ ] Add cache metrics (hit rate, miss rate, evictions)
- [ ] Write unit tests (cache hit/miss, TTL, eviction)
- [ ] Write integration tests with SurrealDB
- [ ] Add configuration options (max memory, TTL defaults)
- [ ] Document API and usage examples

## Integration Points

- `edge-hive-db`: Transparent cache layer for database queries
- `edge-hive-api`: Cache HTTP responses
- `edge-hive-mcp`: Cache MCP tool results

## Success Criteria

- [ ] Cache hit rate > 80% for typical queries
- [ ] Read latency < 1ms (L1) and < 5ms (L2)
- [ ] Memory usage configurable and bounded
- [ ] All tests passing
- [ ] Documentation complete

## References

- Moka docs: <https://docs.rs/moka>
- Mini-redis: <https://github.com/tokio-rs/mini-redis>
- VPS Migration Plan: `docs/VPS_MIGRATION_PLAN.md`

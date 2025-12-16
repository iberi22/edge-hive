---
title: "Implement Real-time WebSocket with SurrealDB Live Queries"
labels:
  - enhancement
  - real-time
  - ai-plan
assignees: []
---

## Description

Implement real-time pub/sub functionality using SurrealDB Live Queries and WebSocket connections. This replaces Supabase Realtime with a native Rust solution that is 10x faster.

## Architecture

```
Client (Browser/App)
     ↓
[WebSocket Connection]
     ↓
[Real-time Server]
     ├─ Subscribe to topics
     ├─ SurrealDB Live Queries
     └─ Broadcast updates
     ↓
[Client receives events]
```

## Features

### Live Queries

- **Table subscriptions**: `LIVE SELECT * FROM messages`
- **Filtered subscriptions**: `LIVE SELECT * FROM messages WHERE room = $room`
- **Field-level subscriptions**: `LIVE SELECT status, updated_at FROM tasks`

### Pub/Sub Channels

- Custom channels for non-DB events
- Room-based broadcasting
- Private channels (user-specific)

### WebSocket Protocol

```json
// Client → Server (Subscribe)
{
  "type": "subscribe",
  "topic": "messages",
  "filter": { "room": "general" }
}

// Server → Client (Event)
{
  "type": "event",
  "topic": "messages",
  "action": "CREATE",
  "data": { "id": "msg1", "text": "Hello" }
}

// Client → Server (Unsubscribe)
{
  "type": "unsubscribe",
  "topic": "messages"
}
```

## Performance Comparison

| Feature | Supabase Realtime | Edge Hive | Improvement |
|---------|-------------------|-----------|-------------|
| Latency | 100-500ms | 10-50ms | 10x |
| Protocol | PostgreSQL LISTEN/NOTIFY | Native SurrealDB | Native |
| Concurrent Connections | ~1,000 | ~10,000 | 10x |
| Message Throughput | ~10K/sec | ~100K/sec | 10x |

## Technical Requirements

### Dependencies

```toml
[dependencies]
surrealdb = { workspace = true }
tokio = { workspace = true }
tokio-tungstenite = "0.24"
futures = "0.3"
serde = { workspace = true }
serde_json = { workspace = true }
dashmap = "6.1"
```

### Architecture Components

1. **WebSocket Server**: Handles connections
2. **Subscription Manager**: Tracks active subscriptions
3. **Live Query Engine**: Manages SurrealDB live queries
4. **Event Broadcaster**: Distributes events to subscribers

## Implementation Tasks

- [ ] Create `crates/edge-hive-realtime/` crate structure
- [ ] Implement WebSocket server (tokio-tungstenite)
- [ ] Implement subscription manager (DashMap for concurrent access)
- [ ] Integrate SurrealDB Live Queries
- [ ] Add authentication for WebSocket connections
- [ ] Implement pub/sub channels (non-DB events)
- [ ] Add room-based broadcasting
- [ ] Add connection lifecycle management (connect/disconnect)
- [ ] Implement heartbeat/ping-pong
- [ ] Add metrics (active connections, messages/sec)
- [ ] Write unit tests for subscription logic
- [ ] Write integration tests with SurrealDB
- [ ] Create client library (TypeScript for frontend)
- [ ] Document WebSocket protocol
- [ ] Add example usage

## Integration Points

- `edge-hive-db`: SurrealDB Live Queries
- `edge-hive-auth`: WebSocket authentication
- `edge-hive-api`: WebSocket upgrade endpoint (`/api/v1/realtime`)
- Frontend: Client library for subscriptions

## Client Usage Example

```typescript
// app/src/lib/realtime.ts
import { RealtimeClient } from '@edge-hive/realtime-client';

const client = new RealtimeClient('ws://localhost:8443/api/v1/realtime');

// Subscribe to table
client.subscribe('messages', { room: 'general' }, (event) => {
  console.log('New message:', event.data);
});

// Unsubscribe
client.unsubscribe('messages');
```

## Success Criteria

- [ ] WebSocket connections stable for hours
- [ ] Event latency < 50ms (p99)
- [ ] Support 1,000+ concurrent connections
- [ ] Zero message loss
- [ ] Graceful reconnection handling
- [ ] All tests passing
- [ ] Client library documented

## References

- SurrealDB Live Queries: <https://surrealdb.com/docs/surrealql/statements/live>
- tokio-tungstenite: <https://docs.rs/tokio-tungstenite>
- VPS Migration Plan: `docs/VPS_MIGRATION_PLAN.md`

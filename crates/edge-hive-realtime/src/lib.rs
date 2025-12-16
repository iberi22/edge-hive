//! Edge Hive Real-time Server
//!
//! WebSocket server with a simple JSON protocol and integration points
//! for SurrealDB Live Queries.

mod protocol;
mod server;

pub use protocol::{ClientMessage, ServerMessage};
pub use server::{RealtimeServer, RealtimeServerConfig};

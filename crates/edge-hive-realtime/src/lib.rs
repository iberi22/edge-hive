//! Edge Hive Real-time Server
//!
//! WebSocket server with a simple JSON protocol and integration points
//! for SurrealDB Live Queries.

mod protocol;
mod server;
mod system_monitor;

pub use protocol::{ClientMessage, ServerMessage};
pub use server::{RealtimeServer, RealtimeServerConfig};
pub use system_monitor::{SystemMonitor, SystemMetrics};

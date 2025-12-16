//! WebSocket protocol types

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "subscribe")]
    Subscribe {
        topic: String,
        #[serde(default)]
        filter: Option<Value>,
    },

    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        topic: String,
    },

    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "event")]
    Event {
        topic: String,
        action: String,
        data: Value,
    },

    #[serde(rename = "ack")]
    Ack {
        message: String,
    },

    #[serde(rename = "error")]
    Error {
        message: String,
    },

    #[serde(rename = "pong")]
    Pong,
}

use crate::protocol::types::{Long, String};
use macros::Packet;

/// Status response packet with server information
#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct StatusResponse {
    /// Json-formatted string with server information
    pub response: String,
}

/// Pong!
#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct PongResponse {
    /// Timestamp, should be identical with PingRequest
    pub timestamp: Long,
}

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct StatusRequest {}

#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct PingRequest {
    pub timestamp: Long,
}

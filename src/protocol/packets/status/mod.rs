use macros::Packet;
use crate::protocol::types::{Long, String};

#[derive(Debug, Packet)]
pub struct StatusResponse {
    pub response: String,
}

#[derive(Debug, Packet)]
pub struct PongResponse {
    pub timestamp: Long,
}

#[derive(Debug, Packet)]
pub struct StatusRequest {}

#[derive(Debug, Packet)]
pub struct PingRequest {
    pub timestamp: Long,
}

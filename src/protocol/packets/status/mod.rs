use crate::protocol::types::{Long, String};
use macros::Packet;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct StatusResponse {
    pub response: String,
}

#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct PongResponse {
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

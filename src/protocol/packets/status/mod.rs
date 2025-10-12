use macros::Packet;
use crate::protocol::types::{Long, String};

#[derive(Debug, Packet)]
pub struct StatusResponse {
    response: String,
}

#[derive(Debug, Packet)]
pub struct PongResponse {
    timestamp: Long,
}

#[derive(Debug, Packet)]
pub struct StatusRequest {}

#[derive(Debug, Packet)]
pub struct PingRequest {
    timestamp: Long,
}

use macros::Packet;
use crate::protocol::types::{VarInt, String, UnsignedShort};

#[derive(Debug, Packet)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: UnsignedShort,
    pub intent: VarInt,
}

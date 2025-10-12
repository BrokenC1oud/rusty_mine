use crate::protocol::types::{String, UnsignedShort, VarInt};
use macros::Packet;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: UnsignedShort,
    pub intent: VarInt,
}

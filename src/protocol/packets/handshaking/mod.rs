use crate::protocol::types::{String, UnsignedShort, VarInt};
use macros::Packet;

/// Represents the initial handshake packet sent by the client to the server during the handshaking phase.
#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct Handshake {
    /// Protocol version, 1.20.10 -> 773
    pub protocol_version: VarInt,
    /// Server address
    pub server_address: String,
    /// Server port
    pub server_port: UnsignedShort,
    /// Next intent, 1 -> Status, 2 -> Login
    pub intent: VarInt,
}

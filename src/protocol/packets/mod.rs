pub mod handshaking;
pub mod status;

use crate::protocol::packets::handshaking::Handshake;
use crate::protocol::RawPacket;
use eyre::{eyre, Result};
use std::io::Cursor;
use crate::protocol::packets::status::{PingRequest, PongResponse, StatusRequest, StatusResponse};

pub trait Packet: Sized {
    const PACKET_ID: u8;
    fn read(reader: &mut Cursor<&[u8]>) -> Result<Self>;

    fn write(&self, writer: &mut Vec<u8>) -> Result<()>;
}

#[derive(Debug)]
pub enum HandshakingPacket {
    Handshake(Handshake),
}

impl HandshakingPacket {
    pub fn parse(raw_packet: RawPacket) -> Result<Self> {
        if let Some(packet_id) = raw_packet.packet_id() {
            match packet_id {
                Handshake::PACKET_ID => return Ok(HandshakingPacket::Handshake(Handshake::read(&mut Cursor::new(&raw_packet.content()))?)),
                _ => {}
            }
        }

        Err(eyre!("Invalid Packet ID").into())
    }
}

#[derive(Debug)]
pub enum StatusPacket {
    StatusResponse(StatusResponse),
    PongResponse(PongResponse),
    StatusRequest(StatusRequest),
    PingRequest(PingRequest),
}

impl StatusPacket {
    pub fn parse(raw_packet: RawPacket) -> Result<Self> {
        if let Some(packet_id) = raw_packet.packet_id() {
            match packet_id {
                StatusRequest::PACKET_ID => return Ok(StatusPacket::StatusRequest(StatusRequest::read(&mut Cursor::new(&raw_packet.content()))?)),
                PingRequest::PACKET_ID => return Ok(StatusPacket::PingRequest(PingRequest::read(&mut Cursor::new(&raw_packet.content()))?)),
                _ => {},
            }
        }

        Err(eyre!("Invalid Packet ID").into())
    }
}

#[derive(Debug)]
pub enum PacketRegistry {
    Handshaking(HandshakingPacket),
    Status(StatusPacket),
}

impl PacketRegistry {
    pub fn parse_handshaking(raw_packet: RawPacket) -> Result<Self> {
        let packet = HandshakingPacket::parse(raw_packet)?;
        Ok(PacketRegistry::Handshaking(packet))
    }

    pub fn parse_status(raw_packet: RawPacket) -> Result<Self> {
        let packet = StatusPacket::parse(raw_packet)?;
        Ok(PacketRegistry::Status(packet))
    }
}

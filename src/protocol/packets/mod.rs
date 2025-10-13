pub mod configuration;
pub mod handshaking;
pub mod login;
pub mod status;

use crate::protocol::RawPacket;
use crate::protocol::packets::handshaking::Handshake;
use crate::protocol::packets::login::{
    CookieRequest, CookieResponse, DisconnectClient, EncryptionRequest, EncryptionResponse,
    LoginAcknowledged, LoginPluginRequest, LoginPluginResponse, LoginStart, LoginSuccess,
    SetCompression,
};
use crate::protocol::packets::status::{PingRequest, PongResponse, StatusRequest, StatusResponse};
use eyre::{Result, eyre};
use std::io::Cursor;

/// Trait for packets
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
                Handshake::PACKET_ID => {
                    return Ok(HandshakingPacket::Handshake(Handshake::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
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
                StatusRequest::PACKET_ID => {
                    return Ok(StatusPacket::StatusRequest(StatusRequest::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                PingRequest::PACKET_ID => {
                    return Ok(StatusPacket::PingRequest(PingRequest::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                _ => {}
            }
        }

        Err(eyre!("Invalid Packet ID").into())
    }
}

#[derive(Debug)]
pub enum LoginPacket {
    LoginStart(LoginStart),
    EncryptionResponse(EncryptionResponse),
    LoginPluginResponse(LoginPluginResponse),
    LoginAcknowledged(LoginAcknowledged),
    CookieResponse(CookieResponse),

    DisconnectClient(DisconnectClient),
    EncryptionRequest(EncryptionRequest),
    LoginSuccess(LoginSuccess),
    SetCompression(SetCompression),
    LoginPluginRequest(LoginPluginRequest),
    CookieRequest(CookieRequest),
}

impl LoginPacket {
    pub fn parse(raw_packet: RawPacket) -> Result<Self> {
        if let Some(packet_id) = raw_packet.packet_id() {
            match packet_id {
                LoginStart::PACKET_ID => {
                    return Ok(LoginPacket::LoginStart(LoginStart::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                EncryptionResponse::PACKET_ID => {
                    return Ok(LoginPacket::EncryptionResponse(EncryptionResponse::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                LoginPluginResponse::PACKET_ID => {
                    return Ok(LoginPacket::LoginPluginResponse(LoginPluginResponse::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                LoginAcknowledged::PACKET_ID => {
                    return Ok(LoginPacket::LoginAcknowledged(LoginAcknowledged::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                CookieResponse::PACKET_ID => {
                    return Ok(LoginPacket::CookieResponse(CookieResponse::read(
                        &mut Cursor::new(&raw_packet.content()),
                    )?));
                }
                _ => {}
            }
        }

        Err(eyre!("Invalid Packet ID").into())
    }
}

#[derive(Debug)]
pub enum PacketRegistry {
    Handshaking(HandshakingPacket),
    Status(StatusPacket),
    Login(LoginPacket),
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

    pub fn parse_login(raw_packet: RawPacket) -> Result<Self> {
        let packet = LoginPacket::parse(raw_packet)?;
        Ok(PacketRegistry::Login(packet))
    }
}

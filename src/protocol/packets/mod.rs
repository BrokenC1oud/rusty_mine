pub mod configuration;
pub mod handshaking;
pub mod login;
pub mod status;

use crate::protocol::RawPacket;
use crate::protocol::packets::configuration::{
    AcknowledgeFinishConfiguration, AddResourcePack, C2SKeepAlive, C2SKnownPacks, C2SPluginMessage,
    ClearDialog, ClientInformation, CustomClickAction, CustomReportDetails, Disconnect,
    FeatureFlags, FinishConfiguration, Ping, Pong, RegistryData, RemoveResourcePack, ResetChat,
    ResourcePackResponse, S2CKeepAlive, S2CKnownPacks, S2CPluginMessage, ServerLinks, ShowDialog,
    StoreCookie, Transfer, UpdateTags,
};
use crate::protocol::packets::handshaking::Handshake;
use crate::protocol::packets::login::{
    CookieResponse, DisconnectClient, EncryptionRequest, EncryptionResponse, LoginAcknowledged,
    LoginPluginRequest, LoginPluginResponse, LoginStart, LoginSuccess, SetCompression,
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
    CookieRequest(login::CookieRequest),
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
pub enum ConfigurationPacket {
    CookieRequest(configuration::CookieRequest),
    S2CPluginMessage(S2CPluginMessage),
    Disconnect(Disconnect),
    FinishConfiguration(FinishConfiguration),
    S2CKeepAlive(S2CKeepAlive),
    Ping(Ping),
    ResetChat(ResetChat),
    RegistryData(RegistryData),
    RemoveResourcePack(RemoveResourcePack),
    AddResourcePack(AddResourcePack),
    StoreCookie(StoreCookie),
    Transfer(Transfer),
    FeatureFlags(FeatureFlags),
    UpdateTags(UpdateTags),
    S2CKnownPacks(S2CKnownPacks),
    CustomReportDetails(CustomReportDetails),
    ServerLinks(ServerLinks),
    ClearDialog(ClearDialog),
    ShowDialog(ShowDialog),

    ClientInformation(ClientInformation),
    CookieResponse(CookieResponse),
    C2SPluginMessage(C2SPluginMessage),
    AcknowledgeFinishConfiguration(AcknowledgeFinishConfiguration),
    C2SKeepAlive(C2SKeepAlive),
    Pong(Pong),
    ResourcePackResponse(ResourcePackResponse),
    C2SKnownPacks(C2SKnownPacks),
    CustomClickAction(CustomClickAction),
}

impl ConfigurationPacket {
    pub fn parse(raw_packet: RawPacket) -> Result<Self> {
        if let Some(packet_id) = raw_packet.packet_id() {
            match packet_id {
                ClientInformation::PACKET_ID => {
                    return Ok(ConfigurationPacket::ClientInformation(ClientInformation::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                configuration::CookieResponse::PACKET_ID => {
                    return Ok(ConfigurationPacket::CookieResponse(CookieResponse::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                C2SPluginMessage::PACKET_ID => {
                    return Ok(ConfigurationPacket::C2SPluginMessage(C2SPluginMessage::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                AcknowledgeFinishConfiguration::PACKET_ID => {
                    return Ok(ConfigurationPacket::AcknowledgeFinishConfiguration(AcknowledgeFinishConfiguration::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                C2SKeepAlive::PACKET_ID => {
                    return Ok(ConfigurationPacket::C2SKeepAlive(C2SKeepAlive::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                Pong::PACKET_ID => {
                    return Ok(ConfigurationPacket::Pong(Pong::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                ResourcePackResponse::PACKET_ID => {
                    return Ok(ConfigurationPacket::ResourcePackResponse(ResourcePackResponse::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                C2SKnownPacks::PACKET_ID => {
                    return Ok(ConfigurationPacket::C2SKnownPacks(C2SKnownPacks::read(&mut Cursor::new(&raw_packet.content()))?));
                }
                CustomClickAction::PACKET_ID => {
                    return Ok(ConfigurationPacket::CustomClickAction(CustomClickAction::read(&mut Cursor::new(&raw_packet.content()))?));
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
    Configuration(ConfigurationPacket),
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

    pub fn parse_configuration(raw_packet: RawPacket) -> Result<Self> {
        let packet = ConfigurationPacket::parse(raw_packet)?;
        Ok(PacketRegistry::Configuration(packet))
    }
}

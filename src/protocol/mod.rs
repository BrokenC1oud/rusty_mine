use crate::protocol::packets::{
    HandshakingPacket, LoginPacket, Packet, PacketRegistry, StatusPacket,
};
use crate::protocol::types::Type;
use crate::protocol::types::VarInt;
use eyre::{Result, eyre};
use std::io::Cursor;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub mod login;
pub mod packets;
pub mod types;
pub mod utils;

#[derive(Debug)]
pub enum ProtocolState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

#[derive(Debug)]
pub struct RawPacket(Vec<u8>);

impl RawPacket {
    pub fn packet_id(&self) -> Option<u8> {
        // for simplicity, seems good
        self.0.first().copied()
    }

    pub fn content(&self) -> &[u8] {
        &self.0[1..]
    }
}

#[derive(Debug)]
pub struct PacketStream<S>(S)
where
    S: AsyncRead + AsyncWrite + Unpin;

impl<S> PacketStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self(stream)
    }

    pub async fn read_raw_packet(&mut self) -> Result<RawPacket> {
        let packet_length = self.read_packet_length().await?;

        let mut data = vec![0; packet_length];
        self.0.read_exact(&mut data).await?;

        Ok(RawPacket(data))
    }

    pub async fn read_packet(&mut self, protocol_state: &ProtocolState) -> Result<PacketRegistry> {
        let raw_packet = self.read_raw_packet().await?;

        match protocol_state {
            ProtocolState::Handshaking => PacketRegistry::parse_handshaking(raw_packet),
            ProtocolState::Status => PacketRegistry::parse_status(raw_packet),
            ProtocolState::Login => PacketRegistry::parse_login(raw_packet),
            ProtocolState::Configuration => todo!(),
            ProtocolState::Play => todo!(),
        }
    }

    pub async fn write_packet(&mut self, packet: PacketRegistry) -> Result<()> {
        let mut buffer = vec![];
        match packet {
            PacketRegistry::Handshaking(packet) => match packet {
                HandshakingPacket::Handshake(real_packet) => real_packet.write(&mut buffer)?,
            },
            PacketRegistry::Status(packet) => match packet {
                StatusPacket::StatusResponse(real_packet) => real_packet.write(&mut buffer)?,
                StatusPacket::PongResponse(real_packet) => real_packet.write(&mut buffer)?,
                _ => Err(eyre!("Invalid packet not sent"))?,
            },
            PacketRegistry::Login(packet) => match packet {
                LoginPacket::DisconnectClient(real_packet) => real_packet.write(&mut buffer)?,
                LoginPacket::EncryptionRequest(real_packet) => real_packet.write(&mut buffer)?,
                LoginPacket::LoginSuccess(real_packet) => real_packet.write(&mut buffer)?,
                LoginPacket::SetCompression(real_packet) => real_packet.write(&mut buffer)?,
                LoginPacket::LoginPluginRequest(real_packet) => real_packet.write(&mut buffer)?,
                LoginPacket::CookieRequest(real_packet) => real_packet.write(&mut buffer)?,
                _ => Err(eyre!("Invalid packet not sent"))?,
            },
        }

        let mut length_buf: Vec<u8> = Vec::new();
        VarInt(buffer.len() as i32).write(&mut length_buf)?;
        self.0.write_all(&length_buf).await?;
        self.0.write_all(&buffer).await?;

        Ok(())
    }

    async fn read_varint(&mut self) -> Result<VarInt> {
        let mut varint_buf = Vec::with_capacity(5);

        for _ in 0..5 {
            let mut byte = [0u8; 1];
            self.0.read_exact(&mut byte).await?;
            varint_buf.push(byte[0]);

            match <VarInt as Type>::read(&mut Cursor::new(&mut varint_buf)) {
                Ok(length) => return Ok(length),
                Err(_) => {}
            }
        }

        Err(eyre!("VarInt too long"))
    }

    pub async fn read_packet_length(&mut self) -> Result<usize> {
        let packet_length = usize::try_from(self.read_varint().await?)?;

        Ok(packet_length)
    }
}

use crate::protocol::types::Byte;
use crate::protocol::types::Identifier;
use crate::protocol::types::uuid::Uuid;
use crate::protocol::types::{
    Boolean, ByteArray, GameProfile, PrefixedArray, PrefixedOptional, String, VarInt,
};
use macros::Packet;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct DisconnectClient {
    pub reason: String,
}

#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: PrefixedArray<Byte>,
    pub verify_token: PrefixedArray<Byte>,
    pub should_authenticate: Boolean,
}

#[derive(Debug, Packet)]
#[packet(id = 0x02)]
pub struct LoginSuccess {
    pub profile: GameProfile,
}

#[derive(Debug, Packet)]
#[packet(id = 0x03)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Debug, Packet)]
#[packet(id = 0x04)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: Identifier,
    pub data: ByteArray,
}

#[derive(Debug, Packet)]
#[packet(id = 0x05)]
pub struct CookieRequest {
    pub key: Identifier,
}

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid,
}

#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct EncryptionResponse {
    pub shared_secret: PrefixedArray<Byte>,
    pub verify_token: PrefixedArray<Byte>,
}

#[derive(Debug, Packet)]
#[packet(id = 0x02)]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub data: PrefixedOptional<ByteArray>,
}

#[derive(Debug, Packet)]
#[packet(id = 0x03)]
pub struct LoginAcknowledged {}

#[derive(Debug, Packet)]
#[packet(id = 0x04)]
pub struct CookieResponse {
    pub key: Identifier,
    pub payload: PrefixedOptional<PrefixedArray<Byte>>,
}

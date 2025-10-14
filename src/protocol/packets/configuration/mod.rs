use crate::protocol::types::Int;
use crate::protocol::types::{
    Boolean, Byte, ByteArray, Identifier, Long, Nbt, Or, PrefixedArray, PrefixedOptional, String,
    Uuid, VarInt,
};
use macros::Packet;
use serde_json::json;

/// Requests a cookie that was previously stored.
#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct CookieRequest {
    /// The identifier of the cookie
    pub key: Identifier,
}

/// Mods and plugins can use this to send their data
#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct S2CPluginMessage {
    /// Name of the plugin channel used to send the data
    pub channel: Identifier,
    /// Any data, length inferred from packet length
    pub data: ByteArray,
}

/// Disconnect client
#[derive(Debug, Packet)]
#[packet(id = 0x02)]
pub struct Disconnect {
    /// Json Text Component
    pub reason: String,
}

impl Disconnect {
    /// util to get a DisconnectClient with reason
    pub fn text(reason: std::string::String) -> Self {
        Self {
            reason: String(json!({"text": reason}).to_string()),
        }
    }
}

/// Sent by server to notify the client that the configuration process has finished. The client
/// answers with Acknowledge Finish Configuration whenever it is ready to continue.
///
/// This packet switches the connection state to Play
#[derive(Debug, Packet)]
#[packet(id = 0x03)]
pub struct FinishConfiguration {}

/// The server will frequently send out a keep-alive, each containing a random ID. The client must
/// respond with the same payload (see Serverbound Keep Alive). If the client does not respond to a
/// Keep Alive packet within 15 seconds after it was sent, the server kicks the client. Vice versa,
/// if the server does not send any keep-alives for 20 seconds, the client will disconnect and
/// yield a "Timed out" exception.
///
/// The vanilla server uses a system-dependent time in milliseconds to generate the keep alive ID value.
#[derive(Debug, Packet)]
#[packet(id = 0x04)]
pub struct S2CKeepAlive {
    pub id: Long,
}

/// Packet is not used by the vanilla server. When sent to the client, the client responds with a
/// Pong packet with the same ID.
#[derive(Debug, Packet)]
#[packet(id = 0x05)]
pub struct Ping {
    pub id: Int,
}

#[derive(Debug, Packet)]
#[packet(id = 0x06)]
pub struct ResetChat {}

/// Represents certain registries that are sent from the server and are applied on the client.
#[derive(Debug, Packet)]
#[packet(id = 0x07)]
pub struct RegistryData {
    pub registry_id: Identifier,
    pub entries: PrefixedArray<(Identifier, PrefixedOptional<Nbt>)>,
}

#[derive(Debug, Packet)]
#[packet(id = 0x08)]
pub struct RemoveResourcePack {
    /// The UUID of the resource pack to be removed.
    /// If not present, every resource pack will be removed.
    pub uuid: Uuid,
}

#[derive(Debug, Packet)]
#[packet(id = 0x09)]
pub struct AddResourcePack {
    /// The unique identifier of the resource pack.
    pub uuid: Uuid,
    /// The URL to the resource pack.
    pub url: String,
    /// A 40 character hexadecimal, case-insensitive SHA-1 hash of the resource pack file.
    /// If it's not a 40-character hexadecimal string, the client will not use it for hash
    /// verification and likely waste bandwidth.
    pub hash: String,
    /// The vanilla client will be forced to use the resource pack from the server.
    /// If they decline, they will be kicked from the server.
    pub forced: Boolean,
    /// This is shown in the prompt making the client accept or decline the
    /// resource pack (only if present).
    pub prompt_message: PrefixedOptional<Nbt>,
}

/// Stores some arbitrary data on the client, which persists between server transfers.
/// The vanilla client only accepts cookies of up to 5 kiB in size.
#[derive(Debug, Packet)]
#[packet(id = 0x0a)]
pub struct StoreCookie {
    /// The identifier of the cookie.
    pub key: Identifier,
    /// The data of the cookie.
    pub payload: PrefixedArray<Byte>,
}

/// Notifies the client that it should transfer to the given server. Cookies previously stored
/// are preserved between server transfers.
#[derive(Debug, Packet)]
#[packet(id = 0x0b)]
pub struct Transfer {
    /// The hostname or IP of the server.
    pub host: String,
    /// The port of the server.
    pub port: VarInt,
}

/// Used to enable and disable features, generally experimental ones, on the client.
#[derive(Debug, Packet)]
#[packet(id = 0x0c)]
pub struct FeatureFlags {
    pub feature_flags: PrefixedArray<Identifier>,
}

#[derive(Debug, Packet)]
#[packet(id = 0x0d)]
pub struct UpdateTags {
    pub tags: PrefixedArray<(
        Identifier,
        PrefixedArray<(Identifier, PrefixedArray<VarInt>)>,
    )>,
}

/// Informs the client of which data packs are present on the server. The client is expected to
/// respond with its own Serverbound Known Packs packet. The vanilla server does not continue
/// with Configuration until it receives a response.
///
/// The vanilla client requires the minecraft:core pack with version 1.21.8 for a normal
/// login sequence. This packet must be sent before the Registry Data packets.
#[derive(Debug, Packet)]
#[packet(id = 0x0e)]
pub struct S2CKnownPacks {
    pub known_packs: PrefixedArray<(String, String, String)>,
}

/// Contains a list of key-value text entries that are included in any crash or disconnection
/// report generated during connection to the server.
#[derive(Debug, Packet)]
#[packet(id = 0x0f)]
pub struct CustomReportDetails {
    pub details: PrefixedArray<(String, String)>,
}

/// This packet contains a list of links that the vanilla client will display in the menu
/// available from the pause menu. Link labels can be built-in or custom (i.e., any text).
#[derive(Debug, Packet)]
#[packet(id = 0x10)]
pub struct ServerLinks {
    pub links: PrefixedArray<(Or<VarInt, Nbt>, String)>,
}

/// If we're currently in a dialog screen, then this removes the current screen
/// and switches back to the previous one.
#[derive(Debug, Packet)]
#[packet(id = 0x11)]
pub struct ClearDialog {}

/// Show a custom dialog screen to the client.
#[derive(Debug, Packet)]
#[packet(id = 0x12)]
pub struct ShowDialog {
    /// Inline definition as described at Registry_data#Dialog.
    pub dialog: Nbt,
}

/// Sent when the player connects, or when settings are changed.
#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct ClientInformation {
    /// e.g. en_GB.
    pub locale: String,
    /// Client-side render distance, in chunks.
    pub view_distance: Byte,
    /// 0: enabled, 1: commands only, 2: hidden. See Chat#Client chat mode for more information.
    pub chat_mode: VarInt,
    /// “Colors” multiplayer setting. The vanilla server stores this value but does nothing
    /// with it (see MC-64867). Some third-party servers disable all coloring in chat and
    /// system messages when it is false.
    pub chat_colors: Boolean,
    /// Bit mask.
    pub displayed_skin_parts: Byte,
    /// 0: Left, 1: Right.
    pub main_hand: VarInt,
    /// Enables filtering of text on signs and written book titles. The vanilla client sets this
    /// according to the profanityFilterPreferences.profanityFilterOn account attribute
    /// indicated by the /player/attributes Mojang API endpoint. In offline mode, it is always false.
    pub enable_text_filtering: Boolean,
    /// Servers usually list online players; this option should let you not show up in that list.
    pub allow_server_listings: Boolean,
    /// 0: all, 1: decreased, 2: minimal
    pub particle_status: VarInt,
}

/// Response to a Cookie Request (configuration) from the server. The vanilla server only accepts
/// responses of up to 5 kiB in size.
#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct CookieResponse {
    /// The identifier of the cookie.
    pub key: Identifier,
    /// The data of the cookie.
    pub payload: PrefixedOptional<PrefixedArray<Byte>>,
}

/// Mods and plugins can use this to send their data. Minecraft itself uses some plugin channels.
/// These internal channels are in the minecraft namespace.
///
/// More documentation on this: https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/
///
/// Note that the length of Data is known only from the packet length, since the packet has
/// no length field of any kind.
#[derive(Debug, Packet)]
#[packet(id = 0x02)]
pub struct C2SPluginMessage {
    /// Name of the plugin channel used to send the data.
    pub channel: Identifier,
    /// Any data, depending on the channel. minecraft: channels are documented in wiki.
    /// The length of this array must be inferred from the packet length.
    pub data: ByteArray,
}

/// Sent by the client to notify the server that the configuration process has finished.
/// It is sent in response to the server's Finish Configuration.
///
/// This packet switches the connection state to play.
#[derive(Debug, Packet)]
#[packet(id = 0x03)]
pub struct AcknowledgeFinishConfiguration {}

/// The server will frequently send out a keep-alive (see Clientbound Keep Alive),
/// each containing a random ID. The client must respond with the same packet.
#[derive(Debug, Packet)]
#[packet(id = 0x04)]
pub struct C2SKeepAlive {
    pub id: Long,
}

/// Response to the clientbound packet (Ping) with the same id.
#[derive(Debug, Packet)]
#[packet(id = 0x05)]
pub struct Pong {
    pub id: Int,
}

#[derive(Debug, Packet)]
#[packet(id = 0x06)]
pub struct ResourcePackResponse {
    /// The unique identifier of the resource pack received in the
    /// Add Resource Pack (configuration) request.
    pub uuid: Uuid,
    /// Result ID.
    pub result: VarInt,
}

/// Informs the server of which data packs are present on the client. The client sends this
/// in response to Clientbound Known Packs.
///
/// If the client specifies a pack in this packet, the server should omit its contained data
/// from the Registry Data packet.
#[derive(Debug, Packet)]
#[packet(id = 0x07)]
pub struct C2SKnownPacks {
    pub known_packs: PrefixedArray<(String, String, String)>,
}

/// Sent when the client clicks a Text Component with the minecraft:custom click action.
/// This is meant as an alternative to running a command, but it will not have any effect on vanilla servers.
#[derive(Debug, Packet)]
#[packet(id = 0x08)]
pub struct CustomClickAction {
    /// The identifier for the click action.
    pub id: Identifier,
    /// The data to send with the click action. May be a TAG_END (0).
    pub payload: Nbt,
}

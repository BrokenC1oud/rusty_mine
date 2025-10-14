use crate::config::Config;
use crate::protocol::login::{LoginState, MojangAuthenticateResult};
use crate::protocol::packets::login::{DisconnectClient, EncryptionRequest, LoginSuccess};
use crate::protocol::packets::status::{PongResponse, StatusResponse};
use crate::protocol::packets::{ConfigurationPacket, HandshakingPacket, LoginPacket, PacketRegistry, StatusPacket};
use crate::protocol::types::{Boolean, Byte, GameProfile, PrefixedArray};
use crate::protocol::utils::{Description, Players, ServerListPingStatusResponse, Version};
use crate::protocol::{PacketStream, ProtocolState, types};
use eyre::{Result, eyre};
use rsa::pkcs8::EncodePublicKey;
use rsa::rand_core::{OsRng, RngCore};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use sha1::{Digest, Sha1};
use std::io;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct ServerState {
    config: Config,

    protocol_version_number: u32,
    version_name: String,

    private_key: RsaPrivateKey,

    online: usize,
}

impl ServerState {
    pub fn new(config: Config, private_key: RsaPrivateKey) -> Self {
        Self {
            config,

            protocol_version_number: 773,
            version_name: String::from("1.21.10"),

            private_key,

            online: 0,
        }
    }
}

pub struct Server {
    listener: TcpListener,

    state: Arc<RwLock<ServerState>>,
}

#[derive(Debug)]
pub struct PlayerInfo {
    name: String,
    uuid: Uuid,
}

#[derive(Debug)]
pub struct Client {
    stream: PacketStream<TcpStream>,

    protocol_state: ProtocolState,
    login_state: Option<LoginState>,

    player_info: Option<PlayerInfo>,

    server_state: Option<Arc<RwLock<ServerState>>>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client {
            stream: PacketStream::new(stream),

            protocol_state: ProtocolState::Handshaking,
            login_state: None,

            player_info: None,

            server_state: None,
        }
    }

    pub async fn handle_client(mut self, server_state: Arc<RwLock<ServerState>>) -> Result<()> {
        self.server_state = Some(server_state);
        self.server_state.as_ref().unwrap().write().await.online += 1;

        loop {
            let packet = self.stream.read_packet(&self.protocol_state).await;

            let packet = match packet {
                Ok(packet) => packet,
                Err(e) => {
                    if let Some(io_error) = e.downcast_ref::<io::Error>() {
                        if io_error.kind() == ErrorKind::UnexpectedEof {
                            debug!("client disconnected");
                            return Ok(());
                        }
                    }
                    return Err(e);
                }
            };

            match packet {
                PacketRegistry::Handshaking(handshaking_packet) => {
                    self.handle_handshaking_packet(handshaking_packet).await?
                }
                PacketRegistry::Status(status_packet) => {
                    self.handle_status_packet(status_packet).await?
                }
                PacketRegistry::Login(login_packet) => {
                    self.handle_login_packet(login_packet).await?
                }
                PacketRegistry::Configuration(configuration_packet) => {
                    self.handle_configuration_packet(configuration_packet).await?
                }
            }
        }
    }

    pub async fn handle_handshaking_packet(&mut self, packet: HandshakingPacket) -> Result<()> {
        let HandshakingPacket::Handshake(inner) = packet;

        if u32::from(inner.protocol_version)
            != self
                .server_state
                .as_ref()
                .unwrap()
                .read()
                .await
                .protocol_version_number
        {
            Err(eyre!("Unmatched protocol version"))?
        }

        match u32::from(inner.intent) {
            1 => self.protocol_state = ProtocolState::Status,
            2 => self.protocol_state = ProtocolState::Login,
            _ => Err(eyre!("Unknown intent"))?,
        }

        Ok(())
    }

    pub async fn handle_status_packet(&mut self, packet: StatusPacket) -> Result<()> {
        match packet {
            StatusPacket::StatusRequest(_) => {
                let resp = ServerListPingStatusResponse {
                    version: Version {
                        name: self
                            .server_state
                            .as_ref()
                            .unwrap()
                            .read()
                            .await
                            .version_name
                            .clone(),
                        protocol: self
                            .server_state
                            .as_ref()
                            .unwrap()
                            .read()
                            .await
                            .protocol_version_number,
                    },
                    players: Players {
                        max: self
                            .server_state
                            .as_ref()
                            .unwrap()
                            .read()
                            .await
                            .config
                            .max_players,
                        online: 0,
                    },
                    description: Description {
                        text: self
                            .server_state
                            .as_ref()
                            .unwrap()
                            .read()
                            .await
                            .config
                            .motd
                            .clone(),
                    },
                };

                let packet = StatusResponse {
                    response: types::String(serde_json::to_string(&resp)?),
                };
                self.stream
                    .write_packet(PacketRegistry::Status(StatusPacket::StatusResponse(packet)))
                    .await?;
            }
            StatusPacket::PingRequest(packet) => {
                self.stream
                    .write_packet(PacketRegistry::Status(StatusPacket::PongResponse(
                        PongResponse {
                            timestamp: packet.timestamp,
                        },
                    )))
                    .await?;
            }
            _ => Err(eyre!("Invalid packet received"))?,
        }

        Ok(())
    }

    pub async fn handle_login_packet(&mut self, packet: LoginPacket) -> Result<()> {
        match packet {
            LoginPacket::LoginStart(packet) => {
                if let Some(_) = &self.login_state {
                    self.stream
                        .write_packet(PacketRegistry::Login(LoginPacket::DisconnectClient(
                            DisconnectClient {
                                reason: types::String(
                                    r#"{"text": "Invalid Login Sequence"}"#.to_string(),
                                ),
                            },
                        )))
                        .await?;

                    Err(eyre!("Invalid Login Sequence"))?
                }

                let verify_token = OsRng.next_u32().to_be_bytes();
                debug!("verify_token: {:?}", verify_token);

                self.login_state = Some(LoginState::Start(verify_token));
                self.player_info = Some(PlayerInfo {
                    name: packet.name.0,
                    uuid: packet.uuid.0,
                });

                let public_key = self
                    .server_state
                    .as_ref()
                    .unwrap()
                    .read()
                    .await
                    .private_key
                    .to_public_key()
                    .to_public_key_der()?
                    .to_vec();

                self.stream
                    .write_packet(PacketRegistry::Login(LoginPacket::EncryptionRequest(
                        EncryptionRequest {
                            server_id: types::String("".to_string()),
                            public_key: PrefixedArray(
                                public_key.iter().map(|&b| Byte(b)).collect(),
                            ),
                            verify_token: PrefixedArray(
                                verify_token.iter().map(|&b| Byte(b)).collect(),
                            ),
                            should_authenticate: Boolean(true),
                        },
                    )))
                    .await?;
            }
            LoginPacket::EncryptionResponse(packet) => {
                let verify_token = if let Some(LoginState::Start(verify_token)) = self.login_state {
                    verify_token
                } else {
                    Err(eyre!("Invalid Login Sequence"))?
                };

                let deciphered_verify_token = self
                    .server_state
                    .as_ref()
                    .unwrap()
                    .read()
                    .await
                    .private_key
                    .decrypt(
                        Pkcs1v15Encrypt,
                        &packet
                            .verify_token
                            .0
                            .iter()
                            .map(|b| b.0)
                            .collect::<Vec<_>>(),
                    )?;
                debug!("deciphered_verify_token: {:?}", deciphered_verify_token);

                if &deciphered_verify_token == &verify_token {
                    debug!("which is a match");
                    self.login_state = Some(LoginState::Verified);

                    let shared_secret = self
                        .server_state
                        .as_ref()
                        .unwrap()
                        .read()
                        .await
                        .private_key
                        .decrypt(
                            Pkcs1v15Encrypt,
                            &packet
                                .shared_secret
                                .0
                                .iter()
                                .map(|b| b.0)
                                .collect::<Vec<_>>(),
                        )?;

                    debug!("Decrypted shared secret: {:?}", shared_secret);

                    self.stream
                        .enable_encryption(&shared_secret.clone(), &shared_secret);

                    let public_key_der = self
                        .server_state
                        .as_ref()
                        .unwrap()
                        .read()
                        .await
                        .private_key
                        .to_public_key()
                        .to_public_key_der()?
                        .to_vec();

                    let mut hash = Sha1::new();
                    hash.update("".as_bytes());
                    hash.update(&shared_secret);
                    hash.update(&public_key_der);
                    let hash = hash.finalize().to_vec();

                    debug!("serverIdHash: {:?}", hash);

                    let resp = reqwest::get(format!(
                        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
                        self.player_info.as_ref().unwrap().name,
                        num_bigint::BigInt::from_signed_bytes_be(&hash).to_str_radix(16),
                    ))
                        .await?
                        .json::<MojangAuthenticateResult>()
                        .await?;

                    self.stream
                        .write_packet(PacketRegistry::Login(LoginPacket::LoginSuccess(
                            LoginSuccess {
                                profile: GameProfile {
                                    uuid: types::Uuid(resp.id),
                                    username: types::String(resp.name),
                                    properties: PrefixedArray(
                                        resp.properties.iter().map(|p| p.into()).collect(),
                                    ),
                                },
                            },
                        )))
                        .await?;

                    debug!("login success sent")
                } else {
                    self.stream
                        .write_packet(PacketRegistry::Login(LoginPacket::DisconnectClient(
                            DisconnectClient::text("Unable to verify token".into()),
                        )))
                        .await?;
                }
            }
            LoginPacket::LoginPluginResponse(_) => {}
            LoginPacket::LoginAcknowledged(_) => {
                debug!("login acknowledged");
                self.protocol_state = ProtocolState::Configuration;
            }
            LoginPacket::CookieResponse(_) => {}
            _ => Err(eyre!("Invalid packet received"))?,
        }

        Ok(())
    }

    pub async fn handle_configuration_packet(&mut self, packet: ConfigurationPacket) -> Result<()> {
        Ok(())
    }
}

impl Server {
    pub async fn new(config: Config) -> Result<Self> {
        let mut rng = OsRng;
        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;

        let bind = format!("0.0.0.0:{}", config.server_port);
        let listener = TcpListener::bind(&bind).await?;
        info!("listening on {}", &bind);

        let state = Arc::new(RwLock::new(ServerState::new(config, private_key)));

        Ok(Server { listener, state })
    }

    pub async fn run(self) -> Result<()> {
        tokio::select! {
            () = self.handle_connection() => {},
            _ = tokio::signal::ctrl_c() => {
                info!("Received SIGINT, quitting");
            }
        }

        Ok(())
    }

    async fn handle_connection(self) {
        loop {
            match self.listener.accept().await {
                Ok((socket, address)) => {
                    debug!("new connection from {}", &address);
                    let state_clone = self.state.clone();
                    let state_clone_ = self.state.clone();
                    let client = Client::new(socket);
                    tokio::spawn(async move {
                        client.handle_client(state_clone).await.unwrap();
                        state_clone_.write().await.online -= 1;
                    });
                }
                Err(e) => {
                    error!("failed to accept connection: {}", e);
                }
            }
        }
    }
}

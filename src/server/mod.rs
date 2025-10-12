use std::sync::Arc;
use crate::config::Config;
use eyre::{eyre, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use crate::protocol::{types, PacketStream, ProtocolState};
use crate::protocol::packets::{HandshakingPacket, PacketRegistry, StatusPacket};
use crate::protocol::packets::status::{PongResponse, StatusResponse};
use crate::protocol::utils::{Description, Players, ServerListPingStatusResponse, Version};

#[derive(Debug)]
pub struct ServerState {
    config: Config,
    protocol_version_number: u32,
    version_name: String,

    online: usize,
}

impl ServerState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            protocol_version_number: 773,
            version_name: String::from("1.21.10"),

            online: 0,
        }
    }
}

pub struct Server {
    listener: TcpListener,

    state: Arc<RwLock<ServerState>>,
}

#[derive(Debug)]
pub struct Client {
    stream: PacketStream<TcpStream>,

    protocol_state: ProtocolState,

    server_state: Option<Arc<RwLock<ServerState>>>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client {
            stream: PacketStream::new(stream),
            protocol_state: ProtocolState::Handshaking,
            server_state: None,
        }
    }

    pub async fn handle_client(mut self, server_state: Arc<RwLock<ServerState>>) -> Result<()> {
        self.server_state = Some(server_state);
        self.server_state.as_ref().unwrap().write().await.online += 1;

        loop {
            let packet = self.stream.read_packet(&self.protocol_state).await?;

            match packet {
                PacketRegistry::Handshaking(handshaking_packet) => self.handle_handshaking_packet(handshaking_packet).await?,
                PacketRegistry::Status(status_packet) => self.handle_status_packet(status_packet).await?,
            }
        }
    }

    pub async fn handle_handshaking_packet(&mut self, packet: HandshakingPacket) -> Result<()> {
        let HandshakingPacket::Handshake(inner) = packet;

        if u32::from(inner.protocol_version) != self.server_state.as_ref().unwrap().read().await.protocol_version_number {
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
                        name: self.server_state.as_ref().unwrap().read().await.version_name.clone(),
                        protocol: self.server_state.as_ref().unwrap().read().await.protocol_version_number,
                    },
                    players: Players {
                        max: self.server_state.as_ref().unwrap().read().await.config.max_players,
                        online: 0,
                    },
                    description: Description {
                        text: self.server_state.as_ref().unwrap().read().await.config.motd.clone(),
                    },
                };

                let packet = StatusResponse { response: types::String(serde_json::to_string(&resp)?) };
                self.stream.write_packet(PacketRegistry::Status(StatusPacket::StatusResponse(packet))).await?;
                debug!("status response packet sent");
            }
            StatusPacket::PingRequest(packet) => {
                self.stream.write_packet(PacketRegistry::Status(StatusPacket::PongResponse(PongResponse { timestamp: packet.timestamp }))).await?;
                debug!("pong response packet sent");
            }
            _ => Err(eyre!("Invalid packet received"))?,
        }

        Ok(())
    }
}

impl Server {
    pub async fn new(config: Config) -> Result<Self> {
        let bind = format!("0.0.0.0:{}", config.server_port);
        let listener = TcpListener::bind(&bind).await?;
        info!("listening on {}", &bind);

        let state = Arc::new(RwLock::new(ServerState::new(config)));

        Ok(Server {
            listener,
            state,
        })
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

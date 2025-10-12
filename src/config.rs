use std::io::ErrorKind;
use eyre::{Report, Result};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "server-port", default = "Config::default_server_port")]
    pub server_port: usize,
    #[serde(rename = "max-players", default = "Config::default_max_players")]
    pub max_players: usize,
    #[serde(default = "Config::default_motd")]
    pub motd: String,
}

impl Config {
    fn default_server_port() -> usize { 25565 }
    fn default_max_players() -> usize { 20 }
    fn default_motd() -> String { String::from("A Minecraft Server") }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_port: Self::default_server_port(),
            max_players: Self::default_max_players(),
            motd: Self::default_motd(),
        }
    }
}

pub async fn load_config(file: String) -> Result<Config> {
    let file = match File::open(&file).await {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let mut file = File::create_new(file).await?;
                file.write(serde_java_properties::to_string(&Config::default())?.as_bytes()).await?;
                file
            },
            _ => Err(Report::new(e))?,
        }
    };
    let mut reader = BufReader::new(file);
    let mut config = String::new();
    reader.read_to_string(&mut config).await?;

    let config = serde_java_properties::from_str(&config)?;
    Ok(config)
}

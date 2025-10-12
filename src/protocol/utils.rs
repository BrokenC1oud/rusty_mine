use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};

pub fn read_u8(reader: &mut Cursor<&[u8]>) -> eyre::Result<u8> {
    let mut buffer = vec![0u8; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

pub fn write_u8(writer: &mut Vec<u8>, value: u8) -> eyre::Result<()> {
    writer.write((&value).to_be_bytes().as_ref())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ServerListPingStatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: Description,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub max: usize,
    pub online: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub text: String,
}

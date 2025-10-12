use crate::protocol::types::Type;
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub struct Uuid(pub uuid::Uuid);

impl Type for Uuid {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut bytes = [0u8; 16];
        reader.read(&mut bytes)?;

        Ok(Uuid(uuid::Uuid::from_bytes(bytes)))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write(self.0.as_bytes())?;

        Ok(())
    }
}

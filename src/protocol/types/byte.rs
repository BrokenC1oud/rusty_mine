use crate::protocol::types::Type;
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub struct Byte(pub u8);

impl Type for Byte {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(Self(buffer[0]))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write(&[self.0])?;

        Ok(())
    }
}

use std::io::{Cursor, Read, Write};
use crate::protocol::types::Type;

#[derive(Debug)]
pub struct UnsignedShort(pub u16);

impl Type for UnsignedShort {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;

        Ok(Self(u16::from_be_bytes(buffer)))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write_all(&self.0.to_be_bytes())?;
        Ok(())
    }
}

use crate::protocol::types::Type;
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub struct Boolean(pub bool);

impl Type for Boolean {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut buffer = [0u8; 1];
        reader.read(&mut buffer)?;
        Ok(Boolean(buffer[0] == 0x01))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write(&[if self.0 { 0x01 } else { 0x00 }])?;

        Ok(())
    }
}

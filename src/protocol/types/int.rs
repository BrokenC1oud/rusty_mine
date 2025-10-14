use crate::protocol::types::Type;
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub struct Int(i32);

impl Type for Int {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        Ok(Int(i32::from_be_bytes(buffer)))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write_all(&i32::to_be_bytes(self.0))?;
        Ok(())
    }
}

use std::io::{Cursor, Read, Write};
use crate::protocol::types::Type;

#[derive(Debug)]
pub struct Long(i64);

impl Type for Long {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        Ok(Long(i64::from_be_bytes(buffer)))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write_all(self.0.to_be_bytes().as_ref())?;

        Ok(())
    }
}

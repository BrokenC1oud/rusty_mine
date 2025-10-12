use crate::protocol::types::Type;
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub struct ByteArray(pub Vec<u8>);

impl Type for ByteArray {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut buffer = Vec::new();
        // should be inferred from context, but only usage indicates this
        reader.read_to_end(&mut buffer)?;
        Ok(ByteArray(buffer))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        writer.write(&self.0)?;

        Ok(())
    }
}

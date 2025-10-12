use std::io::{Cursor, Read, Write};
use crate::protocol::types::Type;
use crate::protocol::types::varint::VarInt;

#[derive(Debug)]
pub struct String(pub std::string::String);

impl Type for String {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let length = VarInt::read(reader)?.0 as usize;
        let mut buffer = vec![0; length];
        reader.read_exact(&mut buffer)?;

        Ok(String(std::string::String::from_utf8(buffer)?))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        VarInt(self.0.len() as i32).write(writer)?;
        writer.write_all(self.0.as_bytes())?;

        Ok(())
    }
}

use crate::protocol::types::{String, Type};
use std::io::Cursor;

#[derive(Debug)]
pub struct Identifier(pub String);

impl Type for Identifier {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        Ok(Self(String::read(reader)?))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        self.0.write(writer)?;

        Ok(())
    }
}

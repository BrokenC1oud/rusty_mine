use crate::protocol::types::Type;
use std::io::Cursor;

#[derive(Debug)]
pub struct Nbt(simdnbt::owned::Nbt);

impl Type for Nbt {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        Ok(Self(simdnbt::owned::read(reader)?.into()))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        self.0.write(writer);
        Ok(())
    }
}

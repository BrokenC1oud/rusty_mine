use crate::protocol::types::uuid::Uuid;
use crate::protocol::types::{PrefixedArray, Property, String, Type};
use std::io::Cursor;

#[derive(Debug)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: PrefixedArray<Property>,
}

impl Type for GameProfile {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        Ok(Self {
            uuid: Uuid::read(reader)?,
            username: String::read(reader)?,
            properties: PrefixedArray::read(reader)?,
        })
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        self.uuid.write(writer)?;
        self.username.write(writer)?;
        self.properties.write(writer)?;

        Ok(())
    }
}

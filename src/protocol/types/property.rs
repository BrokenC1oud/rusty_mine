use crate::protocol::types::PrefixedOptional;
use crate::protocol::types::{String, Type};
use std::io::Cursor;

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: PrefixedOptional<String>,
}

impl Type for Property {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        Ok(Property {
            name: String::read(reader)?,
            value: String::read(reader)?,
            signature: PrefixedOptional::read(reader)?,
        })
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        self.name.write(writer)?;
        self.value.write(writer)?;
        self.signature.write(writer)?;

        Ok(())
    }
}

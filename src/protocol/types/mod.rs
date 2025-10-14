use eyre::Result;
use std::io::Cursor;

pub mod boolean;
pub mod byte;
pub mod bytearray;
pub mod gameprofile;
pub mod identifier;
pub mod int;
pub mod long;
pub mod nbt;
pub mod or;
pub mod prefixed;
pub mod property;
pub mod string;
pub mod ushort;
pub mod uuid;
pub mod varint;

pub use boolean::Boolean;
pub use byte::Byte;
pub use bytearray::ByteArray;
pub use gameprofile::GameProfile;
pub use identifier::Identifier;
pub use int::Int;
pub use long::Long;
pub use nbt::Nbt;
pub use or::Or;
pub use prefixed::{PrefixedArray, PrefixedOptional};
pub use property::Property;
pub use string::String;
pub use ushort::UnsignedShort;
pub use uuid::Uuid;
pub use varint::VarInt;

pub trait Type: Sized {
    fn read(reader: &mut Cursor<&[u8]>) -> Result<Self>;
    fn write(&self, writer: &mut Vec<u8>) -> Result<()>;
}

impl<A, B> Type for (A, B)
where
    A: Type,
    B: Type,
{
    fn read(reader: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok((A::read(reader)?, B::read(reader)?))
    }

    fn write(&self, writer: &mut Vec<u8>) -> Result<()> {
        self.0.write(writer)?;
        self.1.write(writer)?;

        Ok(())
    }
}

impl<A, B, C> Type for (A, B, C)
where
    A: Type,
    B: Type,
    C: Type,
{
    fn read(reader: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok((A::read(reader)?, B::read(reader)?, C::read(reader)?))
    }

    fn write(&self, writer: &mut Vec<u8>) -> Result<()> {
        self.0.write(writer)?;
        self.1.write(writer)?;
        self.2.write(writer)?;

        Ok(())
    }
}

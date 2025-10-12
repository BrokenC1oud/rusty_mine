use eyre::Result;
use std::io::Cursor;

pub mod boolean;
pub mod byte;
pub mod bytearray;
pub mod gameprofile;
pub mod identifier;
pub mod long;
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
pub use long::Long;
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

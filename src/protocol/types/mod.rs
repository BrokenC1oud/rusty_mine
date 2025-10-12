use eyre::Result;
use std::io::Cursor;

pub mod long;
pub mod varint;
pub mod string;
pub mod ushort;

pub use long::Long as Long;
pub use varint::VarInt as VarInt;
pub use string::String as String;
pub use ushort::UnsignedShort as UnsignedShort;

pub trait Type: Sized {
    fn read(reader: &mut Cursor<&[u8]>) -> Result<Self>;
    fn write(&self, writer: &mut Vec<u8>) -> Result<()>;
}

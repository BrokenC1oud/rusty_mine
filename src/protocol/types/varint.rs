use crate::protocol::types::Type;
use crate::protocol::utils::{read_u8, write_u8};
use std::io::Cursor;

#[derive(Debug)]
pub struct VarInt(pub i32);

impl Type for VarInt {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let mut value = 0i32;
        let mut idx = 0;

        loop {
            let byte = read_u8(reader)?;
            value |= ((byte & 0x7F) as i32) << idx * 7;
            if byte & 0x80 == 0 {
                break;
            }
            idx += 1;
        }

        Ok(VarInt(value))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        let mut value = self.0;
        loop {
            let b = (value as u8) & 0x7F;
            value >>= 7;
            if value != 0 {
                write_u8(writer, b | 0b1000_0000)?;
            } else {
                write_u8(writer, b)?;
                break;
            }
        }

        Ok(())
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}

impl From<VarInt> for usize {
    fn from(value: VarInt) -> Self {
        value.0 as usize
    }
}

impl From<VarInt> for u32 {
    fn from(value: VarInt) -> Self {
        value.0 as u32
    }
}

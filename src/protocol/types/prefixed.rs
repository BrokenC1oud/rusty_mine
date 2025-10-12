use crate::protocol::types::{Boolean, Type, VarInt};
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub struct PrefixedArray<I>(pub Vec<I>);

impl<I: Type> Type for PrefixedArray<I> {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let length = VarInt::read(reader)?;
        let mut result: Vec<I> = Vec::with_capacity(length.0 as usize);
        for _ in 0..length.0 {
            result.push(I::read(reader)?);
        }

        Ok(PrefixedArray(result))
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        VarInt(self.0.len() as i32).write(writer)?;
        for element in &self.0 {
            element.write(writer)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PrefixedOptional<I>(pub Option<I>);

impl<I: Type> Type for PrefixedOptional<I> {
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        let Boolean(present) = Boolean::read(reader)?;

        if present {
            Ok(PrefixedOptional(Some(I::read(reader)?)))
        } else {
            Ok(PrefixedOptional(None))
        }
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        if let Some(v) = &self.0 {
            writer.write(&[0x01])?;
            v.write(writer)?;
        } else {
            writer.write(&[0x00])?;
        }

        Ok(())
    }
}

use crate::protocol::types::{Boolean, Type};
use std::io::Cursor;

#[derive(Debug)]
pub enum Or<A, B>
where
    A: Type,
    B: Type,
{
    A(A),
    B(B),
}

impl<A, B> Type for Or<A, B>
where
    A: Type,
    B: Type,
{
    fn read(reader: &mut Cursor<&[u8]>) -> eyre::Result<Self> {
        if Boolean::read(reader)?.0 {
            Ok(Or::A(A::read(reader)?))
        } else {
            Ok(Or::B(B::read(reader)?))
        }
    }

    fn write(&self, writer: &mut Vec<u8>) -> eyre::Result<()> {
        match self {
            Or::A(a) => {
                Boolean(true).write(writer)?;
                a.write(writer)?;
            }
            Or::B(b) => {
                Boolean(false).write(writer)?;
                b.write(writer)?;
            }
        }

        Ok(())
    }
}

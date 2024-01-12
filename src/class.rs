use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::error::{ParseError, Result};
use std::io::{Read, Write};

use crate::{class_version::ClassVersion, Serializable};

#[derive(Debug, PartialEq)]
pub struct Class {
    pub magic: u32,
    pub version: ClassVersion,
}

const MAGIC: u32 = 0xCAFEBABE;

impl Serializable for Class {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let magic = reader.read_u32::<BigEndian>()?;

        if magic != MAGIC {
            return Err(ParseError::InvalidMagicNumber(magic).into());
        }

        let version = ClassVersion::read(reader)?;
        Ok(Class { magic, version })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32::<BigEndian>(self.magic)?;
        self.version.write(writer)?;
        Ok(())
    }
}

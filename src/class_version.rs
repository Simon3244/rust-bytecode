use crate::error::Result as ParseResult;
use crate::{error::ParseError, Serializable};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use core::convert::TryFrom;
use std::{
    convert::TryInto,
    io::{Read, Write},
};

use derive_try_from_primitive::TryFromPrimitive;

#[derive(Debug, PartialEq)]
pub struct ClassVersion {
    pub major: MajorVersion,
    pub minor: u16,
}

impl Serializable for ClassVersion {
    fn read<R: Read>(reader: &mut R) -> ParseResult<Self> {
        let minor = reader.read_u16::<BigEndian>()?;
        let major = reader.read_u16::<BigEndian>()?;
        let test = MajorVersion::try_from(major)
            .map_err(|_| ParseError::Unrecognized("major version", major.to_string()))?;
        Ok(ClassVersion::new(test, minor))
    }
    fn write<W: Write>(&self, writer: &mut W) -> ParseResult<()> {
        writer.write_u16::<BigEndian>(self.minor)?;
        writer.write_u16::<BigEndian>(self.major.into())?;
        Ok(())
    }
}

impl ClassVersion {
    pub fn new(major: MajorVersion, minor: u16) -> Self {
        Self { major, minor }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u16)]
pub enum MajorVersion {
    JDK_1_1 = 45,
    JDK_1_2 = 46,
    JDK_1_3 = 47,
    JDK_1_4 = 48,
    JDK_5 = 49,
    JDK_6 = 50,
    JDK_7 = 51,
    JDK_8 = 52,
    JDK_9 = 53,
    JDK_10 = 54,
    JDK_11 = 55,
    JDK_12 = 56,
    JDK_13 = 57,
    JDK_14 = 58,
    JDK_15 = 59,
    JDK_16 = 60,
    JDK_17 = 61,
    JDK_18 = 62,
    JDK_19 = 63,
    JDK_20 = 64,
    JDK_21 = 65,
    JDK_22 = 66,
}

impl From<MajorVersion> for u16 {
    fn from(major: MajorVersion) -> u16 {
        major as u16
    }
}

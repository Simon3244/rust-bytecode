use std::convert::TryInto;
use std::io::Read;

use crate::Result;
use crate::{structs::ClassVersion, Readable};
use byteorder::{BigEndian, ReadBytesExt};

impl Readable for ClassVersion {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let minor = reader.read_u16::<BigEndian>()?;
        let major = reader.read_u16::<BigEndian>()?.try_into()?;
        Ok(ClassVersion { major, minor })
    }
}

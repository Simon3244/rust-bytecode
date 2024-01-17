use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::structs::interfaces::{Interface, Interfaces};
use crate::structs::Index;
use crate::{Readable, Result};

impl Readable for Interfaces {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let interfaces_count = reader.read_u16::<BigEndian>()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(Interface {
                index: Index::read(reader)?,
            });
        }
        Ok(interfaces)
    }
}

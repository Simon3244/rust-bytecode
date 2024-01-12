use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::const_type::ConstType;
use crate::error::Result;
use crate::Serializable;

pub struct ConstPool {
    pub entries: Vec<Option<ConstType>>,
}

impl Serializable for ConstPool {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        todo!()
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        todo!()
    }
}

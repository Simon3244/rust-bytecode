use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{
    structs::{access_flags::FieldFlags, attributes::Attributes, Field, Fields, Index},
    Readable, Result,
};

impl Readable for Fields {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let fields_count = reader.read_u16::<BigEndian>()?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(Field::read(reader)?);
        }
        Ok(fields)
    }
}

impl Readable for Field {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let access_flags = FieldFlags::read(reader)?;
        let name_index = Index::read(reader)?;
        let descriptor_index = Index::read(reader)?;
        let attributes = Attributes::read(reader)?;
        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

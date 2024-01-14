use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{
    structs::{access_flags::MethodFlags, attributes::Attributes, Index, Method, Methods},
    Readable, Result,
};

impl Readable for Methods {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let methods_count = reader.read_u16::<BigEndian>()?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Method::read(reader)?);
        }
        Ok(Self { methods })
    }
}

impl Readable for Method {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let access_flags = MethodFlags::read(reader)?;
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

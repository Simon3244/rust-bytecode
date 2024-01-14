use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::structs::access_flags::ClassFlags;
use crate::structs::attributes::Attributes;
use crate::structs::{ClassVersion, ConstPool, Fields, Index, Interfaces, Methods};
use crate::{error::ParseError, Result};

use crate::{structs::Class, Readable};

impl Readable for Class {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let magic = reader.read_u32::<BigEndian>()?;
        if magic != 0xCAFEBABE {
            return Err(ParseError::InvalidMagicNumber(magic));
        }
        let version = ClassVersion::read(reader)?;
        let constant_pool = ConstPool::read(reader)?;
        let access_flags = ClassFlags::read(reader)?;
        let this_class = Index::read(reader)?;
        let super_class = Index::read(reader)?;
        let interfaces = Interfaces::read(reader)?;
        let mut fields = Fields::read(reader)?;
        let mut methods = Methods::read(reader)?;
        let mut attributes = Attributes::read(reader)?;
        fields.classify_attributes(&constant_pool);
        methods.classify_attributes(&constant_pool);
        attributes.classify(&constant_pool);
        Ok(Class {
            magic,
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

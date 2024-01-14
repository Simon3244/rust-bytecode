use std::io::Read;

use byteorder::ReadBytesExt;

use crate::{
    error::ParseError,
    structs::access_flags::{
        ClassFlags, ExportsFlags, InnerClassFlags, MethodFlags, MethodParameterFlags, ModuleFlags,
        RequiresFlags,
    },
    Readable, Result,
};

macro_rules! impl_readable_for_access_flags {
    ($($flag:ident),*) => {
        $(impl Readable for $flag {
            fn read<R: Read>(reader: &mut R) -> Result<Self> {
                let flags = reader.read_u16::<byteorder::BigEndian>()?;
                Ok(Self::from_bits(flags).ok_or(ParseError::InvalidAccessFlags(flags))?)
            }
        })*
    };
}

impl_readable_for_access_flags! {
    ClassFlags,
    MethodFlags,
    ExportsFlags,
    RequiresFlags,
    ModuleFlags,
    InnerClassFlags,
    MethodParameterFlags
}

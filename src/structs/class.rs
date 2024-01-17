use crate::impl_get_pretty;

use super::{
    access_flags::ClassFlags, attributes::Attributes, const_types::Class as ConstClass,
    ClassVersion, ConstPool, Fields, Index, Interfaces, Methods,
};

pub const MAGIC: u32 = 0xCAFEBABE;

impl_get_pretty! {
#[derive(Debug)]
pub struct Class {
    pub magic: u32,
    pub version: ClassVersion,
    pub constant_pool: ConstPool,
    pub access_flags: ClassFlags,
    pub this_class: Index<ConstClass>,
    pub super_class: Index<ConstClass>,
    pub interfaces: Interfaces,
    pub fields: Fields,
    pub methods: Methods,
    pub attributes: Attributes,
}
}

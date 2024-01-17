use super::{ConstItem, Index};
use crate::{
    error::ParseError,
    impl_get_pretty,
    structs::const_types::{
        Class, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef, InvokeDynamic, Long,
        MethodHandle, MethodRef, MethodType, Module, NameAndType, Package, StringJ, Utf8,
    },
    Result,
};

impl_get_pretty! {
#[derive(Debug)]
pub struct ConstPool {
    pub entries: Vec<Option<ConstItem>>,
}
}

impl ConstPool {
    pub fn get<T: TryFromItem>(&self, index: &Index<T>) -> Result<&T> {
        let entry = &self.entries[index.index as usize];
        let entry = entry.as_ref().unwrap();
        T::try_from(entry).ok_or(ParseError::Other(
            "Invalid constant pool entry access".to_string(),
        ))
    }
}

pub trait TryFromItem: Sized {
    fn try_from(item: &ConstItem) -> Option<&Self>;
}

macro_rules! impl_try_from_item {
    ($($name:ident),*) => {
        $(
            impl TryFromItem for $name {
                fn try_from(item: &ConstItem) -> Option<&Self> {
                    if let ConstItem::$name(x) = item {
                        Some(x)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

impl TryFromItem for ConstItem {
    fn try_from(item: &ConstItem) -> Option<&Self> {
        Some(item)
    }
}

impl_try_from_item! {
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    StringJ,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package
}

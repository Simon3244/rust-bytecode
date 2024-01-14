use std::marker::PhantomData;

use crate::Result;

use super::{const_pool::TryFromItem, ConstPool};

#[derive(Debug, Clone, PartialEq)]
pub enum ConstItem {
    Utf8(Utf8),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    Class(Class),
    StringJ(StringJ),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    NameAndType(NameAndType),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    Dynamic(Dynamic),
    InvokeDynamic(InvokeDynamic),
    Module(Module),
    Package(Package),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index<T: TryFromItem> {
    pub index: u16,
    _marker: PhantomData<T>,
}

pub type OptionalIndex<T> = Index<T>;

impl<T: TryFromItem> Index<T> {
    pub fn get<'a>(&self, pool: &'a ConstPool) -> Result<&'a T> {
        pool.get(self)
    }

    pub fn new(index: u16) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }

    pub fn index(&self) -> u16 {
        self.index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8 {
    pub bytes: Vec<u8>,
    pub str: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Long {
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Double {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub name_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringJ {
    pub string_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldRef {
    pub class_index: Index<Class>,
    pub name_and_type_index: Index<NameAndType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodRef {
    pub class_index: Index<Class>,
    pub name_and_type_index: Index<NameAndType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceMethodRef {
    pub class_index: Index<Class>,
    pub name_and_type_index: Index<NameAndType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameAndType {
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodHandle {
    pub reference_kind: MethodHandleReferenceKind,
    // Index into the constant pool table
    // If the reference_kind is 1-4, then this points to a FieldRef
    // If the reference_kind is 5-8, then this points to a MethodRef
    // If the reference_kind is 9, then this points to an InterfaceMethodRef
    pub reference_index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodHandleReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodType {
    pub descriptor_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dynamic {
    // Index into the bootstrap_methods array of the bootstrap method table
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: Index<NameAndType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeDynamic {
    // Index into the bootstrap_methods array of the bootstrap method table
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: Index<NameAndType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name_index: Index<Utf8>,
}

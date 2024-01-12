use std::{
    convert::{TryFrom, TryInto},
    io::{self, Read, Write},
};

use derive_more::Constructor;

use enum_dispatch::enum_dispatch;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{error::ParseError, Result};

#[enum_dispatch]
trait Serializable: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}

// pub struct ConstPool {
//     pub entries: Vec<Option<ConstType>>,
// }

// impl Serializable for ConstPool {
//     fn read<R: Read>(reader: &mut R) -> ParseResult<Self> {
//         let length = reader.read_u16::<BigEndian>()?;
//         let mut entries = Vec::with_capacity(length as usize);
//         entries.push(None);

//         let mut i = 1;
//         while i < length {
//             let tag = reader.read_u8()?;
//             let entry = match tag {
//                 ConstType::CONST_UTF8 => {
//                     let length = reader.read_u16::<BigEndian>()?;
//                     let mut buf = vec![0; length as usize];
//                     reader.read_exact(&mut buf)?;
//                     let str = String::from_utf8(buf)?;
//                     ConstType::Utf8(Utf8Info { str })
//                 }
//                 _ => return Err(ParseError::Unrecognized("tag", tag.to_string()).into()),
//             };
//             entries.push(Some(entry));
//             i += 1;
//         }

//         Ok(ConstPool { entries })
//     }

//     fn write<W: Write>(&self, writer: &mut W) -> ParseResult<()> {
//         writer.write_u16::<BigEndian>(self.entries.len() as u16)?;
//         for entry in self.entries.iter() {
//             match entry {
//                 Some(entry) => match entry {
//                     ConstType::Utf8(utf8_info) => {
//                         writer.write_u8(ConstType::CONST_UTF8)?;
//                         writer.write_u16::<BigEndian>(utf8_info.str.len() as u16)?;
//                         writer.write_all(utf8_info.str.as_bytes())?;
//                     }
//                     _ => {
//                         return Err(ParseError::Unrecognized("const type", entry.to_string()).into())
//                     }
//                 },
//                 None => return Err(ParseError::Unrecognized("const entry", "None".into()).into()),
//             }
//         }
//         Ok(())
//     }
// }

type Index = u16;

// Add a trait to Read and Write to read and write an index.
// This is used in the Constant Pool.

impl<R: io::Read + ?Sized> IndexReadable for R {
    fn read_index(&mut self) -> Result<Index> {
        self.read_u16::<BigEndian>().map_err(Into::into)
    }
}

impl<W: io::Write + ?Sized> IndexWritable for W {
    #[inline]
    fn write_index(&mut self, index: Index) -> Result<()> {
        self.write_u16::<BigEndian>(index).map_err(Into::into)
    }
}

pub trait IndexReadable: io::Read {
    fn read_index(&mut self) -> Result<Index>;
}

pub trait IndexWritable: io::Write {
    fn write_index(&mut self, index: Index) -> Result<()>;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Utf8Data {
    str: String,
}

impl Serializable for Utf8Data {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let length = reader.read_u16::<BigEndian>()?;
        let mut buf = vec![0; length as usize];
        reader.read_exact(&mut buf)?;
        let str = String::from_utf8(buf)?;
        Ok(Utf8Data { str })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let buf = self.str.as_bytes();
        writer.write_u16::<BigEndian>(buf.len() as u16)?;
        writer.write_all(buf)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntegerData {
    value: i32,
}

impl Serializable for IntegerData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value = reader.read_i32::<BigEndian>()?;
        Ok(IntegerData { value })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32::<BigEndian>(self.value)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatData {
    inner: u32,
}

impl FloatData {
    pub fn value(&self) -> f32 {
        f32::from_bits(self.inner)
    }

    pub fn set_value(&mut self, value: f32) {
        self.inner = value.to_bits();
    }

    pub fn new(value: f32) -> Self {
        Self {
            inner: value.to_bits(),
        }
    }
}

impl Serializable for FloatData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let inner = reader.read_u32::<BigEndian>()?;
        Ok(FloatData { inner })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32::<BigEndian>(self.inner)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LongData {
    value: i64,
}

impl Serializable for LongData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value = reader.read_i64::<BigEndian>()?;
        Ok(LongData { value })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i64::<BigEndian>(self.value)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DoubleData {
    inner: u64,
}

impl DoubleData {
    pub fn value(&self) -> f64 {
        f64::from_bits(self.inner)
    }

    pub fn set_value(&mut self, value: f64) {
        self.inner = value.to_bits();
    }

    pub fn new(value: f64) -> Self {
        Self {
            inner: value.to_bits(),
        }
    }
}

impl Serializable for DoubleData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let inner = reader.read_u64::<BigEndian>()?;
        Ok(DoubleData { inner })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u64::<BigEndian>(self.inner)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClassData {
    name_index: Index,
}

impl Serializable for ClassData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = reader.read_index()?;
        Ok(ClassData { name_index })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.name_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StringData {
    string_index: Index,
}

impl Serializable for StringData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let string_index = reader.read_index()?;
        Ok(StringData { string_index })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.string_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FieldRefData {
    class_index: Index,
    name_and_type_index: Index,
}

impl Serializable for FieldRefData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = reader.read_index()?;
        let name_and_type_index = reader.read_index()?;
        Ok(FieldRefData {
            class_index,
            name_and_type_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.class_index)?;
        writer.write_index(self.name_and_type_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodRefData {
    class_index: Index,
    name_and_type_index: Index,
}

impl Serializable for MethodRefData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = reader.read_index()?;
        let name_and_type_index = reader.read_index()?;
        Ok(MethodRefData {
            class_index,
            name_and_type_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.class_index)?;
        writer.write_index(self.name_and_type_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InterfaceMethodRefData {
    class_index: Index,
    name_and_type_index: Index,
}

impl Serializable for InterfaceMethodRefData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = reader.read_index()?;
        let name_and_type_index = reader.read_index()?;
        Ok(InterfaceMethodRefData {
            class_index,
            name_and_type_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.class_index)?;
        writer.write_index(self.name_and_type_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NameAndTypeData {
    name_index: Index,
    descriptor_index: Index,
}

impl Serializable for NameAndTypeData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = reader.read_index()?;
        let descriptor_index = reader.read_index()?;
        Ok(NameAndTypeData {
            name_index,
            descriptor_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.name_index)?;
        writer.write_index(self.descriptor_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodHandleData {
    reference_kind: ReferenceKind,
    reference_index: Index,
}

impl Serializable for MethodHandleData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let reference_kind = reader.read_u8()?.try_into()?;
        let reference_index = reader.read_index()?;
        Ok(MethodHandleData {
            reference_kind,
            reference_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.reference_kind as u8)?;
        writer.write_index(self.reference_index)?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum ReferenceKind {
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

impl TryFrom<u8> for ReferenceKind {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(ReferenceKind::GetField),
            2 => Ok(ReferenceKind::GetStatic),
            3 => Ok(ReferenceKind::PutField),
            4 => Ok(ReferenceKind::PutStatic),
            5 => Ok(ReferenceKind::InvokeVirtual),
            6 => Ok(ReferenceKind::InvokeStatic),
            7 => Ok(ReferenceKind::InvokeSpecial),
            8 => Ok(ReferenceKind::NewInvokeSpecial),
            9 => Ok(ReferenceKind::InvokeInterface),
            _ => Err(ParseError::Unrecognized("reference kind", value.to_string()).into()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodTypeData {
    descriptor_index: Index,
}

impl Serializable for MethodTypeData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let descriptor_index = reader.read_index()?;
        Ok(MethodTypeData { descriptor_index })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.descriptor_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DynamicData {
    bootstrap_method_attr_index: Index,
    name_and_type_index: Index,
}

impl Serializable for DynamicData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let bootstrap_method_attr_index = reader.read_index()?;
        let name_and_type_index = reader.read_index()?;
        Ok(DynamicData {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.bootstrap_method_attr_index)?;
        writer.write_index(self.name_and_type_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InvokeDynamicData {
    bootstrap_method_attr_index: Index,
    name_and_type_index: Index,
}

impl Serializable for InvokeDynamicData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let bootstrap_method_attr_index = reader.read_index()?;
        let name_and_type_index = reader.read_index()?;
        Ok(InvokeDynamicData {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.bootstrap_method_attr_index)?;
        writer.write_index(self.name_and_type_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModuleData {
    name_index: Index,
}

impl Serializable for ModuleData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = reader.read_index()?;
        Ok(ModuleData { name_index })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.name_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PackageData {
    name_index: Index,
}

impl Serializable for PackageData {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = reader.read_index()?;
        Ok(PackageData { name_index })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_index(self.name_index)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(dead_code, non_camel_case_types)]
// #[enum_dispatch(Serializable)]
pub enum ConstType {
    Utf8(Utf8Data),
    Integer(IntegerData),
    Float(FloatData),
    Long(LongData),
    Double(DoubleData),
    Class(ClassData),
    String(StringData),
    FieldRef(FieldRefData),
    MethodRef(MethodRefData),
    InterfaceMethodRef(InterfaceMethodRefData),
    NameAndType(NameAndTypeData),
    MethodHandle(MethodHandleData),
    MethodType(MethodTypeData),
    Dynamic(DynamicData),
    InvokeDynamic(InvokeDynamicData),
    Module(ModuleData),
    Package(PackageData),
}

fn run_write<T>(serializable: &T)
where
    T: Serializable,
{
    let mut buf = Vec::new();
    serializable.write(&mut buf).unwrap();
    println!("{:?}", buf);
}

impl ConstType {
    const CONST_UTF8: u8 = 1;
    const CONST_INTEGER: u8 = 3;
    const CONST_FLOAT: u8 = 4;
    const CONST_LONG: u8 = 5;
    const CONST_DOUBLE: u8 = 6;
    const CONST_CLASS: u8 = 7;
    const CONST_STRING: u8 = 8;
    const CONST_FIELD_REF: u8 = 9;
    const CONST_METHOD_REF: u8 = 10;
    const CONST_INTERFACE_METHOD_REF: u8 = 11;
    const CONST_NAME_AND_TYPE: u8 = 12;
    const CONST_METHOD_HANDLE: u8 = 15;
    const CONST_METHOD_TYPE: u8 = 16;
    const CONST_DYNAMIC: u8 = 17;
    const CONST_INVOKE_DYNAMIC: u8 = 18;
    const CONST_MODULE: u8 = 19;
    const CONST_PACKAGE: u8 = 20;

    fn get_tag(&self) -> u8 {
        match self {
            ConstType::Utf8(_) => ConstType::CONST_UTF8,
            ConstType::Integer(_) => ConstType::CONST_INTEGER,
            ConstType::Float(_) => ConstType::CONST_FLOAT,
            ConstType::Long(_) => ConstType::CONST_LONG,
            ConstType::Double(_) => ConstType::CONST_DOUBLE,
            ConstType::Class(_) => ConstType::CONST_CLASS,
            ConstType::String(_) => ConstType::CONST_STRING,
            ConstType::FieldRef(_) => ConstType::CONST_FIELD_REF,
            ConstType::MethodRef(_) => ConstType::CONST_METHOD_REF,
            ConstType::InterfaceMethodRef(_) => ConstType::CONST_INTERFACE_METHOD_REF,
            ConstType::NameAndType(_) => ConstType::CONST_NAME_AND_TYPE,
            ConstType::MethodHandle(_) => ConstType::CONST_METHOD_HANDLE,
            ConstType::MethodType(_) => ConstType::CONST_METHOD_TYPE,
            ConstType::Dynamic(_) => ConstType::CONST_DYNAMIC,
            ConstType::InvokeDynamic(_) => ConstType::CONST_INVOKE_DYNAMIC,
            ConstType::Module(_) => ConstType::CONST_MODULE,
            ConstType::Package(_) => ConstType::CONST_PACKAGE,
        }
    }
}

impl Serializable for ConstType {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let tag = reader.read_u8()?;

        Ok(match tag {
            ConstType::CONST_UTF8 => ConstType::Utf8(Utf8Data::read(reader)?),
            ConstType::CONST_INTEGER => ConstType::Integer(IntegerData::read(reader)?),
            ConstType::CONST_FLOAT => ConstType::Float(FloatData::read(reader)?),
            ConstType::CONST_LONG => ConstType::Long(LongData::read(reader)?),
            ConstType::CONST_DOUBLE => ConstType::Double(DoubleData::read(reader)?),
            ConstType::CONST_CLASS => ConstType::Class(ClassData::read(reader)?),
            ConstType::CONST_STRING => ConstType::String(StringData::read(reader)?),
            ConstType::CONST_FIELD_REF => ConstType::FieldRef(FieldRefData::read(reader)?),
            ConstType::CONST_METHOD_REF => ConstType::MethodRef(MethodRefData::read(reader)?),
            ConstType::CONST_INTERFACE_METHOD_REF => {
                ConstType::InterfaceMethodRef(InterfaceMethodRefData::read(reader)?)
            }
            ConstType::CONST_NAME_AND_TYPE => {
                ConstType::NameAndType(NameAndTypeData::read(reader)?)
            }
            ConstType::CONST_METHOD_HANDLE => {
                ConstType::MethodHandle(MethodHandleData::read(reader)?)
            }
            ConstType::CONST_METHOD_TYPE => ConstType::MethodType(MethodTypeData::read(reader)?),
            ConstType::CONST_DYNAMIC => ConstType::Dynamic(DynamicData::read(reader)?),
            ConstType::CONST_INVOKE_DYNAMIC => {
                ConstType::InvokeDynamic(InvokeDynamicData::read(reader)?)
            }
            ConstType::CONST_MODULE => ConstType::Module(ModuleData::read(reader)?),
            ConstType::CONST_PACKAGE => ConstType::Package(PackageData::read(reader)?),
            _ => return Err(ParseError::Unrecognized("tag", tag.to_string()).into()),
        })
    }
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.get_tag())?;
        run_write(self);

        Ok(())
    }
}

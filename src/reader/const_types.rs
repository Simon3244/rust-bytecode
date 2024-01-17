use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use cesu8::from_java_cesu8;

use crate::structs::const_types::*;
use crate::{Readable, Result};

use crate::structs::const_pool::TryFromItem;

impl Readable for ConstItem {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let tag = reader.read_u8()?;
        Ok(match tag {
            1 => ConstItem::Utf8(Utf8::read(reader)?),
            3 => ConstItem::Integer(Integer::read(reader)?),
            4 => ConstItem::Float(Float::read(reader)?),
            5 => ConstItem::Long(Long::read(reader)?),
            6 => ConstItem::Double(Double::read(reader)?),
            7 => ConstItem::Class(Class::read(reader)?),
            8 => ConstItem::StringJ(StringJ::read(reader)?),
            9 => ConstItem::FieldRef(FieldRef::read(reader)?),
            10 => ConstItem::MethodRef(MethodRef::read(reader)?),
            11 => ConstItem::InterfaceMethodRef(InterfaceMethodRef::read(reader)?),
            12 => ConstItem::NameAndType(NameAndType::read(reader)?),
            15 => ConstItem::MethodHandle(MethodHandle::read(reader)?),
            16 => ConstItem::MethodType(MethodType::read(reader)?),
            17 => ConstItem::Dynamic(Dynamic::read(reader)?),
            18 => ConstItem::InvokeDynamic(InvokeDynamic::read(reader)?),
            19 => ConstItem::Module(Module::read(reader)?),
            20 => ConstItem::Package(Package::read(reader)?),
            _ => panic!("Invalid constant pool tag"),
        })
    }
}

impl<T: TryFromItem> Readable for Index<T> {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let index = reader.read_u16::<BigEndian>()?;
        Ok(Self::new(index))
    }
}

impl Readable for Utf8 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let length = reader.read_u16::<BigEndian>()?;
        let mut bytes = vec![0; length as usize];
        reader.read_exact(&mut bytes)?;

        let str = match from_java_cesu8(&bytes) {
            Ok(str) => str,
            Err(_) => String::from_utf8_lossy(&bytes),
        }
        .to_string();
        Ok(Utf8 { bytes, str })
    }
}

impl Readable for Integer {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value = reader.read_i32::<BigEndian>()?;
        Ok(Integer { value })
    }
}

impl Readable for Float {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value = reader.read_f32::<BigEndian>()?;
        Ok(Float { value })
    }
}

impl Readable for Long {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value = reader.read_i64::<BigEndian>()?;
        Ok(Long { value })
    }
}

impl Readable for Double {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value = reader.read_f64::<BigEndian>()?;
        Ok(Double { value })
    }
}

impl Readable for Class {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        Ok(Class { name_index })
    }
}

impl Readable for StringJ {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let string_index = Index::read(reader)?;
        Ok(StringJ { string_index })
    }
}

impl Readable for FieldRef {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = Index::read(reader)?;
        let name_and_type_index = Index::read(reader)?;
        Ok(FieldRef {
            class_index,
            name_and_type_index,
        })
    }
}

impl Readable for MethodRef {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = Index::read(reader)?;
        let name_and_type_index = Index::read(reader)?;
        Ok(MethodRef {
            class_index,
            name_and_type_index,
        })
    }
}

impl Readable for InterfaceMethodRef {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = Index::read(reader)?;
        let name_and_type_index = Index::read(reader)?;
        Ok(InterfaceMethodRef {
            class_index,
            name_and_type_index,
        })
    }
}

impl Readable for NameAndType {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        let descriptor_index = Index::read(reader)?;
        Ok(NameAndType {
            name_index,
            descriptor_index,
        })
    }
}

impl Readable for MethodHandle {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let reference_kind = MethodHandleReferenceKind::read(reader)?;
        let reference_index = reader.read_u16::<BigEndian>()?;
        Ok(MethodHandle {
            reference_kind,
            reference_index,
        })
    }
}

impl Readable for MethodHandleReferenceKind {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let reference_kind = reader.read_u8()?;
        Ok(match reference_kind {
            1 => MethodHandleReferenceKind::GetField,
            2 => MethodHandleReferenceKind::GetStatic,
            3 => MethodHandleReferenceKind::PutField,
            4 => MethodHandleReferenceKind::PutStatic,
            5 => MethodHandleReferenceKind::InvokeVirtual,
            6 => MethodHandleReferenceKind::InvokeStatic,
            7 => MethodHandleReferenceKind::InvokeSpecial,
            8 => MethodHandleReferenceKind::NewInvokeSpecial,
            9 => MethodHandleReferenceKind::InvokeInterface,
            _ => panic!("Invalid method handle reference kind"),
        })
    }
}

impl Readable for MethodType {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let descriptor_index = Index::read(reader)?;
        Ok(MethodType { descriptor_index })
    }
}

impl Readable for Dynamic {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let bootstrap_method_attr_index = reader.read_u16::<BigEndian>()?;
        let name_and_type_index = Index::read(reader)?;
        Ok(Dynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
}

impl Readable for InvokeDynamic {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let bootstrap_method_attr_index = reader.read_u16::<BigEndian>()?;
        let name_and_type_index = Index::read(reader)?;
        Ok(InvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
}

impl Readable for Module {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        Ok(Module { name_index })
    }
}

impl Readable for Package {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        Ok(Package { name_index })
    }
}

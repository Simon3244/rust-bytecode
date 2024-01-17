use std::io::Cursor;

use crate::{impl_get_pretty, Classify, Readable, Result};

use super::{
    access_flags::{
        ExportsFlags, InnerClassFlags, MethodParameterFlags, ModuleFlags, OpensFlags, RequiresFlags,
    },
    const_types::{Class, MethodHandle, Module as ModuleConst, NameAndType, Package, Utf8},
    opcodes::Opcode,
    ConstItem, ConstPool, Index, OptionalIndex,
};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Attributes {
//     pub attributes: Vec<Attribute>,
// }

pub type Attributes = Vec<Attribute>;

impl_get_pretty! {

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub attribute_name_index: Index<Utf8>,
    pub info: AttributeInfo,
}
}

impl Classify for Vec<Attribute> {
    fn classify(&mut self, const_pool: &ConstPool) -> Result<()> {
        Ok(for attribute in self.iter_mut() {
            match attribute.classify(const_pool) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error classifying attribute: {:?}", e);
                }
            }
        })
    }
}

impl Attribute {
    pub fn classify(&mut self, pool: &ConstPool) -> Result<()> {
        let bytes = match &self.info {
            AttributeInfo::Unknown(info) => &info.bytes,
            _ => return Ok(()),
        };

        let name = self.attribute_name_index.get(pool)?;
        let mut reader = Cursor::new(bytes);

        let info = AttributeInfo::get(&name.str, &mut reader)?;
        self.info = info;

        if let AttributeInfo::Code(code) = &mut self.info {
            code.attributes.classify(pool)?;
        } else if let AttributeInfo::Record(record) = &mut self.info {
            for component in &mut record.components {
                component.attributes.classify(pool)?;
            }
        }
        Ok(())
    }
}

pub trait Get {
    fn get<R: std::io::Read>(attr_name: &str, reader: &mut R) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! gen_get {
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            $($variant:ident($value:ident),)*
        }
    ) => {
        impl_get_pretty! {
        $(#[$attr])*
        pub enum $name {
            $($variant($value),)*
        }
        }


        impl Get for $name {
            fn get<R: std::io::Read>(attr_name: &str, reader: &mut R) -> Result<Self> {
                match attr_name {
                    $(stringify!($variant) => Ok($name::$variant($value::read(reader)?)),)*
                    _ => Err(crate::error::ParseError::Unrecognized(stringify!($name), attr_name.to_string())),
                }
            }
        }
    };
}
// impl TryFrom<&str> for AttributeTypes {
//     type Error = ParseError;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {}
// }

gen_get! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum AttributeInfo {
        ConstantValue(ConstantValue),
        Code(Code),
        StackMapTable(StackMapTable),
        Exceptions(Exceptions),
        InnerClasses(InnerClasses),
        EnclosingMethod(EnclosingMethod),
        Synthetic(Synthetic),
        Signature(Signature),
        SourceFile(SourceFile),
        SourceDebugExtension(SourceDebugExtension),
        LineNumberTable(LineNumberTable),
        LocalVariableTable(LocalVariableTable),
        LocalVariableTypeTable(LocalVariableTypeTable),
        Deprecated(Deprecated),
        RuntimeVisibleAnnotations(RuntimeVisibleAnnotations),
        RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations),
        RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations),
        RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations),
        AnnotationDefault(AnnotationDefault),
        BootstrapMethods(BootstrapMethods),
        MethodParameters(MethodParameters),
        Module(Module),
        ModulePackages(ModulePackages),
        ModuleMainClass(ModuleMainClass),
        NestHost(NestHost),
        NestMembers(NestMembers),
        Record(Record),
        PermittedSubclasses(PermittedSubclasses),
        Unknown(Unknown),
    }
}

impl_get_pretty! {
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantValue {
    pub constant_value_index: Index<ConstItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Opcode>,
    pub exception_table: Vec<ExceptionTable>,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: OptionalIndex<Class>,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct StackMapTable {
//     pub bytes: Vec<u8>,
// }
// }

pub type StackMapTable = NotImplemented;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exceptions {
    pub exception_index_table: Vec<Index<Class>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerClasses {
    pub classes: Vec<InnerClass>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerClass {
    pub inner_class_info_index: Index<Class>,
    pub outer_class_info_index: OptionalIndex<Class>,
    pub inner_name_index: OptionalIndex<Utf8>,
    pub inner_class_access_flags: InnerClassFlags,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnclosingMethod {
    pub class_index: Index<Class>,
    pub method_index: OptionalIndex<NameAndType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Synthetic();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub signature_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub sourcefile_index: Index<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDebugExtension {
    pub debug_extension: Utf8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineNumberTable {
    pub line_number_table: Vec<LineNumber>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableTable {
    pub local_variable_table: Vec<LocalVariable>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table: Vec<LocalVariableType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: Index<Utf8>,
    pub signature_index: Index<Utf8>,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deprecated();

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RuntimeVisibleAnnotations {
//     // pub annotations: Vec<Annotation>,
//     pub annotations: Vec<u8>,
// }
pub type RuntimeVisibleAnnotations = NotImplemented;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Annotation {
//     pub type_index: Index<Utf8>,
//     pub element_value_pairs: Vec<ElementValuePair>,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct ElementValuePair {
//     pub element_name_index: Index<Utf8>,
//     pub value: ElementValue,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ElementValue {
//     Byte(u8),
//     Char(u16),
//     Double(f64),
//     Float(f32),
//     Int(u32),
//     Long(u64),
//     Short(u16),
//     Boolean(bool),
//     String(Index<Utf8>),
//     EnumConstValue {
//         type_name_index: Index<Utf8>,
//         const_name_index: Index<Utf8>,
//     },
//     ClassInfoIndex(Index<Class>),
//     AnnotationValue(RuntimeAnnotation),
//     ArrayValue(Vec<ElementValue>),
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RuntimeInvisibleAnnotations {
//     // pub annotations: Vec<Annotation>,
//     pub annotations: Vec<u8>,
// }
pub type RuntimeInvisibleAnnotations = NotImplemented;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RuntimeVisibleParameterAnnotations {
//     // pub parameter_annotations: Vec<Vec<Annotation>>,
//     pub parameter_annotations: Vec<u8>,
// }
pub type RuntimeVisibleParameterAnnotations = NotImplemented;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RuntimeInvisibleParameterAnnotations {
//     // pub parameter_annotations: Vec<Vec<Annotation>>,
//     pub parameter_annotations: Vec<u8>,
// }
pub type RuntimeInvisibleParameterAnnotations = NotImplemented;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct AnnotationDefault {
//     // pub default_value: ElementValue,
//     pub default_value: Vec<u8>,
// }
pub type AnnotationDefault = NotImplemented;

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethods {
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: Index<MethodHandle>,
    pub bootstrap_arguments: Vec<Index<ConstItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodParameters {
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodParameter {
    pub name_index: Index<Utf8>,
    pub access_flags: MethodParameterFlags,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name_index: Index<Utf8>,
    pub flags: ModuleFlags,
    pub version_index: OptionalIndex<Utf8>,
    pub requires: Vec<Requires>,
    pub exports: Vec<Exports>,
    pub opens: Vec<Opens>,
    pub uses: Vec<Uses>,
    pub provides: Vec<Provides>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requires {
    pub requires_index: Index<Utf8>,
    pub flags: RequiresFlags,
    pub version_index: OptionalIndex<Utf8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exports {
    pub exports_index: Index<Utf8>,
    pub flags: ExportsFlags,
    pub to_index: Vec<Index<ModuleConst>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opens {
    pub opens_index: Index<Utf8>,
    pub flags: OpensFlags,
    pub to_index: Vec<Index<Utf8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uses {
    pub uses_index: Index<Class>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provides {
    pub provides_index: Index<Class>,
    pub with_index: Vec<Index<Class>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePackages {
    pub package_index: Vec<Index<Package>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleMainClass {
    pub main_class_index: Index<Class>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestHost {
    pub host_class_index: Index<Class>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestMembers {
    pub classes: Vec<Index<Class>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub components: Vec<RecordComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordComponent {
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermittedSubclasses {
    pub classes: Vec<Index<Class>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unknown {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotImplemented {
    pub bytes: Vec<u8>,
}
}

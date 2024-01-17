use std::{convert::TryInto, io::Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{
    structs::{
        access_flags::{
            ExportsFlags, InnerClassFlags, MethodParameterFlags, ModuleFlags, OpensFlags,
            RequiresFlags,
        },
        attributes::*,
        const_types::Utf8,
        Index,
    },
    Readable, Result,
};

impl Readable for Vec<Attribute> {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let attributes_count = reader.read_u16::<BigEndian>()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Attribute::read(reader)?);
        }
        Ok(attributes)
    }
}

impl Readable for Attribute {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let attribute_name_index = Index::read(reader)?;
        let attribute_length = reader.read_u32::<BigEndian>()?;
        let mut data = reader.take(attribute_length as u64);
        let info = AttributeInfo::Unknown(Unknown::read(&mut data)?);
        Ok(Self {
            attribute_name_index,
            info,
        })
    }
}

impl Readable for Unknown {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(Self { bytes })
    }
}

impl Readable for NotImplemented {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(Self { bytes })
    }
}

impl Readable for ConstantValue {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let constant_value_index = Index::read(reader)?;
        Ok(Self {
            constant_value_index,
        })
    }
}

impl Readable for Code {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let max_stack = reader.read_u16::<BigEndian>()?;
        let max_locals = reader.read_u16::<BigEndian>()?;
        let code_length = reader.read_u32::<BigEndian>()?;
        let mut code = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length {
            code.push(reader.read_u8()?.try_into()?);
        }
        let exception_table_length = reader.read_u16::<BigEndian>()?;
        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTable::read(reader)?);
        }
        let attributes = Attributes::read(reader)?;
        Ok(Self {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}

impl Readable for ExceptionTable {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let start_pc = reader.read_u16::<BigEndian>()?;
        let end_pc = reader.read_u16::<BigEndian>()?;
        let handler_pc = reader.read_u16::<BigEndian>()?;
        let catch_type = Index::read(reader)?;
        Ok(Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}

impl Readable for Exceptions {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let number_of_exceptions = reader.read_u16::<BigEndian>()?;
        let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
        for _ in 0..number_of_exceptions {
            exception_index_table.push(Index::read(reader)?);
        }
        Ok(Self {
            exception_index_table,
        })
    }
}

impl Readable for InnerClasses {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let number_of_classes = reader.read_u16::<BigEndian>()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            classes.push(InnerClass::read(reader)?);
        }
        Ok(Self { classes })
    }
}

impl Readable for InnerClass {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let inner_class_info_index = Index::read(reader)?;
        let outer_class_info_index = Index::read(reader)?;
        let inner_name_index = Index::read(reader)?;
        let inner_class_access_flags = InnerClassFlags::read(reader)?;
        Ok(Self {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        })
    }
}

impl Readable for EnclosingMethod {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let class_index = Index::read(reader)?;
        let method_index = Index::read(reader)?;
        Ok(Self {
            class_index,
            method_index,
        })
    }
}

impl Readable for Synthetic {
    fn read<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(Self())
    }
}

impl Readable for Signature {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let signature_index = Index::read(reader)?;
        Ok(Self { signature_index })
    }
}

impl Readable for SourceFile {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let sourcefile_index = Index::read(reader)?;
        Ok(Self { sourcefile_index })
    }
}

impl Readable for SourceDebugExtension {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let debug_extension = Utf8::read(reader)?;
        Ok(Self { debug_extension })
    }
}

impl Readable for LineNumberTable {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let line_number_table_length = reader.read_u16::<BigEndian>()?;
        let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
        for _ in 0..line_number_table_length {
            line_number_table.push(LineNumber::read(reader)?);
        }
        Ok(Self { line_number_table })
    }
}

impl Readable for LineNumber {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let start_pc = reader.read_u16::<BigEndian>()?;
        let line_number = reader.read_u16::<BigEndian>()?;
        Ok(Self {
            start_pc,
            line_number,
        })
    }
}

impl Readable for LocalVariableTable {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let local_variable_table_length = reader.read_u16::<BigEndian>()?;
        let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
        for _ in 0..local_variable_table_length {
            local_variable_table.push(LocalVariable::read(reader)?);
        }
        Ok(Self {
            local_variable_table,
        })
    }
}

impl Readable for LocalVariable {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let start_pc = reader.read_u16::<BigEndian>()?;
        let length = reader.read_u16::<BigEndian>()?;
        let name_index = Index::read(reader)?;
        let descriptor_index = Index::read(reader)?;
        let index = reader.read_u16::<BigEndian>()?;
        Ok(Self {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        })
    }
}

impl Readable for LocalVariableTypeTable {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let local_variable_type_table_length = reader.read_u16::<BigEndian>()?;
        let mut local_variable_type_table =
            Vec::with_capacity(local_variable_type_table_length as usize);
        for _ in 0..local_variable_type_table_length {
            local_variable_type_table.push(LocalVariableType::read(reader)?);
        }
        Ok(Self {
            local_variable_type_table,
        })
    }
}

impl Readable for LocalVariableType {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let start_pc = reader.read_u16::<BigEndian>()?;
        let length = reader.read_u16::<BigEndian>()?;
        let name_index = Index::read(reader)?;
        let signature_index = Index::read(reader)?;
        let index = reader.read_u16::<BigEndian>()?;
        Ok(Self {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        })
    }
}

impl Readable for Deprecated {
    fn read<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(Self())
    }
}

impl Readable for BootstrapMethods {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let num_bootstrap_methods = reader.read_u16::<BigEndian>()?;
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod::read(reader)?);
        }
        Ok(Self { bootstrap_methods })
    }
}

impl Readable for BootstrapMethod {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let bootstrap_method_ref = Index::read(reader)?;
        let num_bootstrap_arguments = reader.read_u16::<BigEndian>()?;
        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
        for _ in 0..num_bootstrap_arguments {
            bootstrap_arguments.push(Index::read(reader)?);
        }
        Ok(Self {
            bootstrap_method_ref,
            bootstrap_arguments,
        })
    }
}

impl Readable for MethodParameters {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let parameters_count = reader.read_u8()?;
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        for _ in 0..parameters_count {
            parameters.push(MethodParameter::read(reader)?);
        }
        Ok(Self { parameters })
    }
}

impl Readable for MethodParameter {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        let access_flags = MethodParameterFlags::read(reader)?;
        Ok(Self {
            name_index,
            access_flags,
        })
    }
}

impl Readable for Module {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        let flags = ModuleFlags::read(reader)?;
        let version_index = Index::read(reader)?;
        let requires_count = reader.read_u16::<BigEndian>()?;
        let mut requires = Vec::with_capacity(requires_count as usize);
        for _ in 0..requires_count {
            requires.push(Requires::read(reader)?);
        }
        let exports_count = reader.read_u16::<BigEndian>()?;
        let mut exports = Vec::with_capacity(exports_count as usize);
        for _ in 0..exports_count {
            exports.push(Exports::read(reader)?);
        }
        let opens_count = reader.read_u16::<BigEndian>()?;
        let mut opens = Vec::with_capacity(opens_count as usize);
        for _ in 0..opens_count {
            opens.push(Opens::read(reader)?);
        }
        let uses_count = reader.read_u16::<BigEndian>()?;
        let mut uses = Vec::with_capacity(uses_count as usize);
        for _ in 0..uses_count {
            uses.push(Uses::read(reader)?);
        }
        let provides_count = reader.read_u16::<BigEndian>()?;
        let mut provides = Vec::with_capacity(provides_count as usize);
        for _ in 0..provides_count {
            provides.push(Provides::read(reader)?);
        }
        Ok(Self {
            name_index,
            flags,
            version_index,
            requires,
            exports,
            opens,
            uses,
            provides,
        })
    }
}

impl Readable for Requires {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let requires_index = Index::read(reader)?;
        let flags = RequiresFlags::read(reader)?;
        let version_index = Index::read(reader)?;
        Ok(Self {
            requires_index,
            flags,
            version_index,
        })
    }
}

impl Readable for Exports {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let exports_index = Index::read(reader)?;
        let flags = ExportsFlags::read(reader)?;
        let exports_to_count = reader.read_u16::<BigEndian>()?;
        let mut to_index = Vec::with_capacity(exports_to_count as usize);
        for _ in 0..exports_to_count {
            to_index.push(Index::read(reader)?);
        }
        Ok(Self {
            exports_index,
            flags,
            to_index,
        })
    }
}

impl Readable for Opens {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let opens_index = Index::read(reader)?;
        let flags = OpensFlags::read(reader)?;
        let opens_to_count = reader.read_u16::<BigEndian>()?;
        let mut to_index = Vec::with_capacity(opens_to_count as usize);
        for _ in 0..opens_to_count {
            to_index.push(Index::read(reader)?);
        }
        Ok(Self {
            opens_index,
            flags,
            to_index,
        })
    }
}

impl Readable for Uses {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let uses_index = Index::read(reader)?;
        Ok(Self { uses_index })
    }
}

impl Readable for Provides {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let provides_index = Index::read(reader)?;
        let provides_with_count = reader.read_u16::<BigEndian>()?;
        let mut with_index = Vec::with_capacity(provides_with_count as usize);
        for _ in 0..provides_with_count {
            with_index.push(Index::read(reader)?);
        }
        Ok(Self {
            provides_index,
            with_index,
        })
    }
}

impl Readable for ModulePackages {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let package_count = reader.read_u16::<BigEndian>()?;
        let mut package_index = Vec::with_capacity(package_count as usize);
        for _ in 0..package_count {
            package_index.push(Index::read(reader)?);
        }
        Ok(Self { package_index })
    }
}

impl Readable for ModuleMainClass {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let main_class_index = Index::read(reader)?;
        Ok(Self { main_class_index })
    }
}

impl Readable for NestHost {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let host_class_index = Index::read(reader)?;
        Ok(Self { host_class_index })
    }
}

impl Readable for NestMembers {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let number_of_classes = reader.read_u16::<BigEndian>()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            classes.push(Index::read(reader)?);
        }
        Ok(Self { classes })
    }
}

impl Readable for Record {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let component_count = reader.read_u16::<BigEndian>()?;
        let mut components = Vec::with_capacity(component_count as usize);
        for _ in 0..component_count {
            components.push(RecordComponent::read(reader)?);
        }
        Ok(Self { components })
    }
}

impl Readable for RecordComponent {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let name_index = Index::read(reader)?;
        let descriptor_index = Index::read(reader)?;
        let attributes = Attributes::read(reader)?;
        Ok(Self {
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl Readable for PermittedSubclasses {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let number_of_classes = reader.read_u16::<BigEndian>()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            classes.push(Index::read(reader)?);
        }
        Ok(Self { classes })
    }
}

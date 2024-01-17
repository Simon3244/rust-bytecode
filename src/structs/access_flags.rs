use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MethodFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ClassFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InnerClassFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ModuleFlags: u16 {
        const OPEN = 0x0020;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MethodParameterFlags: u16 {
        const FINAL = 0x0010;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RequiresFlags: u16 {
        const TRANSITIVE = 0x0020;
        const STATIC_PHASE = 0x0040;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ExportsFlags: u16 {
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

pub type FieldFlags = MethodFlags;
pub type OpensFlags = ExportsFlags;

macro_rules! impl_get_pretty {
    ($($name:ident,)*) => {
        $(
            impl crate::pretty_print::GetPretty for $name {
                fn get_pretty(&self, _pool: &super::ConstPool, tabs: usize) -> crate::error::Result<String> {
                    let mut result = String::new();
                    self.iter_names().for_each(|(name, _)| {
                        result.push_str(&format!("{:indent$}{}", "", name, indent = tabs));
                        result.push_str("\n");
                    });
                    Ok(result)
                }
            }
        )*
    };
}

impl_get_pretty! {
    MethodFlags,
    ClassFlags,
    InnerClassFlags,
    ModuleFlags,
    MethodParameterFlags,
    RequiresFlags,
    ExportsFlags,
}

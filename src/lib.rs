// pub mod class;
// pub mod class_version;
// pub mod const_pool;
// pub mod const_type;
pub mod error;
pub mod pretty_print;
pub mod reader;
pub mod structs;

// use crate::const_type::ConstType;
// use enum_dispatch::enum_dispatch;
use error::Result;
use neon::prelude::*;
use std::io::{Read, Write};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

pub trait Serializable: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}

pub trait Readable: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait Writable {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}

// Macro to generate the TryFrom impl for an enum

#[macro_export]
macro_rules! gen_try_from {
    (
        $(#[$attr:meta])*
        pub enum $Name:ident {
            $($variant:ident = $value:expr,)*
        }
    ) => {
        $(#[$attr])*
        pub enum $Name {
            $($variant = $value,)*
        }

        impl std::convert::TryFrom<u16> for $Name {
            type Error = crate::error::ParseError;
            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($Name::$variant),)*
                    _ => Err(crate::error::ParseError::Unrecognized(stringify!($Name), value.to_string())),
                }
            }
        }
    };
}
// pub use gen_try_from;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::structs::{const_types::MethodRef, ConstItem, Index};

    use super::*;
    use std::process::Command;

    #[test]
    fn test_read_class() {
        // Compile the testdata/HelloWorld.java file to testdata/HelloWorld.class
        Command::new("javac")
            .arg("./testdata/HelloWorld.java")
            .output()
            .expect("Failed to compile testdata/HelloWorld.java");

        let file = std::fs::File::open("testdata/HelloWorld.class").expect("Failed to open file");
        let mut reader = std::io::BufReader::new(file);
        // let class = class::Class::read(&mut reader).expect("Failed to read class file");
        let class = structs::Class::read(&mut reader).expect("Failed to read class file");
        println!("{:#?}", class);

        class.methods.methods.iter().for_each(|method| {
            method
                .attributes
                .attributes
                .iter()
                .filter(|attr| {
                    attr.attribute_name_index
                        .get(&class.constant_pool)
                        .unwrap()
                        .str
                        == "Code"
                })
                .for_each(|code| {
                    println!("{:#?}", code);
                });
        });
    }
}

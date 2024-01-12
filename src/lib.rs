pub mod class;
pub mod class_version;
pub mod const_pool;
pub mod const_type;
pub mod error;

use crate::const_type::ConstType;
use enum_dispatch::enum_dispatch;
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

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_read_class() {
        // Compile the testdata/HelloWorld.java file to testdata/HelloWorld.class
        Command::new("javac")
            .arg("testdata/HelloWorld.java")
            .output()
            .expect("Failed to compile testdata/HelloWorld.java");

        let file = std::fs::File::open("testdata/HelloWorld.class").expect("Failed to open file");
        let mut reader = std::io::BufReader::new(file);
        let class = class::Class::read(&mut reader).expect("Failed to read class file");
        println!("{:#?}", class);
    }
}

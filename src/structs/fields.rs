use crate::{error::Result, impl_get_pretty, Classify};

use super::{
    access_flags::FieldFlags, attributes::Attributes, const_types::Utf8, ConstPool, Index,
};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Fields {
//     pub fields: Vec<Field>,
// }

pub type Fields = Vec<Field>;

impl_get_pretty! {
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub access_flags: FieldFlags,
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
    pub attributes: Attributes,
}
}

impl Classify for Fields {
    fn classify(&mut self, const_pool: &ConstPool) -> Result<()> {
        for field in self.iter_mut() {
            match field.attributes.classify(const_pool) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error classifying field attributes")
                }
            };
        }
        Ok(())
    }
}

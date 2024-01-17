use crate::{error::Result, impl_get_pretty, Classify};

use super::{
    access_flags::MethodFlags, attributes::Attributes, const_types::Utf8, ConstPool, Index,
};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Methods {
//     pub methods: Vec<Method>,
// }
pub type Methods = Vec<Method>;

impl_get_pretty! {

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub access_flags: MethodFlags,
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
    pub attributes: Attributes,
}
}

impl Classify for Methods {
    fn classify(&mut self, const_pool: &ConstPool) -> Result<()> {
        for method in self.iter_mut() {
            match method.attributes.classify(const_pool) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error classifying method attributes")
                }
            };
        }
        Ok(())
    }
}

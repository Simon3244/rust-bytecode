use super::{
    access_flags::MethodFlags, attributes::Attributes, const_types::Utf8, ConstPool, Index,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Methods {
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub access_flags: MethodFlags,
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
    pub attributes: Attributes,
}

impl Methods {
    pub fn classify_attributes(&mut self, const_pool: &ConstPool) {
        for method in &mut self.methods {
            method.attributes.classify(const_pool);
        }
    }
}

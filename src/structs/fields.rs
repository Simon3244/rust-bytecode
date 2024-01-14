use super::{
    access_flags::FieldFlags, attributes::Attributes, const_types::Utf8, ConstPool, Index,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Fields {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub access_flags: FieldFlags,
    pub name_index: Index<Utf8>,
    pub descriptor_index: Index<Utf8>,
    pub attributes: Attributes,
}

impl Fields {
    pub fn classify_attributes(&mut self, const_pool: &ConstPool) {
        for field in &mut self.fields {
            field.attributes.classify(const_pool);
        }
    }
}

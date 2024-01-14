use super::{const_types::Class, Index};

#[derive(Debug)]
pub struct Interfaces {
    pub interfaces: Vec<Interface>,
}

#[derive(Debug)]
pub struct Interface {
    pub index: Index<Class>,
}

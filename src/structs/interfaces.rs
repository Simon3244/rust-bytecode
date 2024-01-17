use crate::impl_get_pretty;

use super::{const_types::Class, Index};

// #[derive(Debug)]
// pub struct Interfaces {
//     pub interfaces: Vec<Interface>,
// }
pub type Interfaces = Vec<Interface>;

impl_get_pretty! {
#[derive(Debug)]
pub struct Interface {
    pub index: Index<Class>,
}
}

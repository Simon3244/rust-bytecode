use std::convert::TryInto;

use crate::structs::const_pool::TryFromItem;
use crate::structs::{ConstPool, Index};
use crate::Result;

trait GetPretty {
    fn get_pretty(&self, pool: &ConstPool) -> Result<String>;
}

macro_rules! impl_get_pretty_primitive {
    ($($name:ident),*) => {
        $(
            impl GetPretty for $name {
                fn get_pretty(&self, _pool: &ConstPool) -> Result<String> {
                    Ok(self.to_string())
                }
            }
        )*
    };
}

impl_get_pretty_primitive! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool,
    char,
    String
}

macro_rules! impl_get_pretty {}

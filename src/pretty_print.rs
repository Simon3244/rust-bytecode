use crate::structs::ConstPool;
use crate::Result;

pub trait GetPretty {
    fn get_pretty(&self, pool: &ConstPool, tabs: usize) -> Result<String>;
}

macro_rules! impl_get_pretty_primitive {
    ($($name:ident),*) => {
        $(
            impl GetPretty for $name {
                fn get_pretty(&self, _pool: &ConstPool, tabs: usize) -> Result<String> {
                    Ok(format!("{:indent$}{}", "", self, indent=tabs))
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

impl<T: GetPretty> GetPretty for Vec<T> {
    fn get_pretty(&self, pool: &ConstPool, tabs: usize) -> Result<String> {
        let mut result = String::new();
        for item in self {
            result.push_str("\n");
            result.push_str(&item.get_pretty(pool, tabs)?);
        }
        Ok(result)
    }
}

impl<T: GetPretty> GetPretty for Option<T> {
    fn get_pretty(&self, pool: &ConstPool, tabs: usize) -> Result<String> {
        match self {
            Some(x) => x.get_pretty(pool, tabs),
            None => Ok("None".to_string()),
        }
    }
}

#[macro_export]
macro_rules! impl_get_pretty {
    () => {};
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident = $value:expr,)*
        }
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $($variant = $value,)*
        }

        impl crate::pretty_print::GetPretty for $name {
            fn get_pretty(&self, _pool: &crate::structs::ConstPool, tabs: usize) -> crate::Result<String> {
                match self {
                    $($name::$variant => Ok(format!("{:indent$}{}", "", stringify!($variant), indent=tabs)),)*
                }
            }
        }
        impl_get_pretty!($($rest)*);
    };
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident($value:ident),)*
        }
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $($variant($value),)*
        }

        impl crate::pretty_print::GetPretty for $name {
            fn get_pretty(&self, pool: &crate::structs::ConstPool, tabs: usize) -> crate::Result<String> {
                match self {
                    $($name::$variant(x) => x.get_pretty(pool, tabs),)*
                }
            }
        }
        impl_get_pretty!($($rest)*);
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            $($f_vis:vis $field:ident: $type:ty,)*
        }
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis struct $name {
            $($f_vis $field: $type,)*
        }


        impl crate::pretty_print::GetPretty for $name {
            fn get_pretty(&self, pool: &crate::structs::ConstPool, tabs: usize) -> crate::Result<String> {
                let mut result = String::new();
                result.push_str(&format!("{:indent$}{} {{", "", stringify!($name), indent=tabs));
                result.push_str("\n");
                $(
                    result.push_str(&format!(
                        "{:indent$}{}: {}",
                        "",
                        stringify!($field),
                        self.$field.get_pretty(pool, tabs + 4)?.trim_start_matches(' '),
                        indent=tabs + 2
                    ));
                    result.push_str("\n");
                )*
                result.push_str(&format!("{:indent$}}}", "", indent=tabs));
                Ok(result)
            }
        }

        impl_get_pretty!($($rest)*);
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($($type:ty)*);
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis struct $name($($type)*);

        impl crate::pretty_print::GetPretty for $name {
            fn get_pretty(&self, _pool: &crate::structs::ConstPool, tabs: usize) -> crate::Result<String> {
                let mut result = String::new();
                result.push_str(&format!("{:indent$}{} {{", "", stringify!($name), indent=tabs));
                result.push_str("\n");
                $(
                    result.push_str(&format!("{:indent$}{}:", "", stringify!($type), indent=tabs + 2));
                    result.push_str("\n");
                )*
                result.push_str(&format!("{:indent$}}}", "", indent=tabs));
                Ok(result)
            }
        }

        impl_get_pretty!($($rest)*);
    };
    (
        $vis:vis type $name:ident = $type:ty;
        $($rest:tt)*
    ) => {
        $vis type $name = $type;
        impl_get_pretty!($($rest)*);
    }
}

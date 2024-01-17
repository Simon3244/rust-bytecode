use crate::gen_try_from;
use crate::impl_get_pretty;

impl_get_pretty! {
#[derive(Debug, PartialEq)]
pub struct ClassVersion {
    pub major: MajorVersion,
    pub minor: u16,
}
}

// impl_get_pretty! {
gen_try_from! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    #[repr(u16)]
    pub enum MajorVersion {
        JDK_1_1 = 45,
        JDK_1_2 = 46,
        JDK_1_3 = 47,
        JDK_1_4 = 48,
        JDK_5 = 49,
        JDK_6 = 50,
        JDK_7 = 51,
        JDK_8 = 52,
        JDK_9 = 53,
        JDK_10 = 54,
        JDK_11 = 55,
        JDK_12 = 56,
        JDK_13 = 57,
        JDK_14 = 58,
        JDK_15 = 59,
        JDK_16 = 60,
        JDK_17 = 61,
        JDK_18 = 62,
        JDK_19 = 63,
        JDK_20 = 64,
        JDK_21 = 65,
        JDK_22 = 66,
    }
}
// }

// impl_get_pretty! {
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Copy, Clone)]
// #[repr(u16)]
// pub enum MajorVersionn {
//     JDK_1_1 = 45,
//     JDK_1_2 = 46,
//     JDK_1_3 = 47,
//     JDK_1_4 = 48,
//     JDK_5 = 49,
//     JDK_6 = 50,
//     JDK_7 = 51,
//     JDK_8 = 52,
//     JDK_9 = 53,
//     JDK_10 = 54,
//     JDK_11 = 55,
//     JDK_12 = 56,
//     JDK_13 = 57,
//     JDK_14 = 58,
//     JDK_15 = 59,
//     JDK_16 = 60,
//     JDK_17 = 61,
//     JDK_18 = 62,
//     JDK_19 = 63,
//     JDK_20 = 64,
//     JDK_21 = 65,
//     JDK_22 = 66,
// }
// }

impl From<MajorVersion> for u16 {
    fn from(major: MajorVersion) -> u16 {
        major as u16
    }
}

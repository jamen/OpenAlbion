use std::os::raw::{c_char,c_long};

use crate::CBasicString;

#[derive(Debug)]
#[repr(C)]
pub struct CCharString {
    data: *mut CCharStringData,
}

#[derive(Debug)]
#[repr(C)]
pub struct CCharStringData {
    pub data: CBasicString<c_char>,
    pub refs_count: c_long,
}

// impl CCharString {
//     fn as_str(&self) -> &str {
//         let buf = self.data.data.p_data as *mut u8;
//         let len = self.data.data.string_length;
//     }
// }
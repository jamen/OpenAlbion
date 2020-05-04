use std::os::raw::{c_char,c_long};

use crate::CBasicString;

#[repr(C)]
pub struct CCharStringData {
    pub data: CBasicString<c_char>,
    pub no_refs: c_long,
}
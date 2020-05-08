use std::os::raw::c_long;

use winapi::ctypes::wchar_t;

use crate::cxx;

#[derive(Debug)]
#[repr(C)]
pub struct CWideString {
    pub p_string_data: *mut CWideStringData,
}

#[derive(Debug)]
#[repr(C)]
pub struct CWideStringData {
    pub data: cxx::StdBasicString<wchar_t, cxx::StdCharTraits<wchar_t>, cxx::StdAllocator<wchar_t>>,
    pub refs_count: c_long,
}
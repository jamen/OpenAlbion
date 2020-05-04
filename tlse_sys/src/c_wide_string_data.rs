use std::os::raw::c_long;

use winapi::ctypes::wchar_t;

use crate::cxx;

#[repr(C)]
pub struct CWideStringData {
    pub data: cxx::StdBasicString<wchar_t, cxx::StdCharTraits<wchar_t>, cxx::StdAllocator<wchar_t>>,
    pub no_refs: c_long,
}
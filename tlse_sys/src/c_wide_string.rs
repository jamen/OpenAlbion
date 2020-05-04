use crate::CWideStringData;

#[repr(C)]
pub struct CWideString {
    pub p_string_data: *mut CWideStringData,
}
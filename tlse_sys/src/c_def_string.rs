use std::os::raw::c_long;

#[derive(Debug)]
#[repr(C)]
pub struct CDefString {
    pub table_pos: c_long,
}
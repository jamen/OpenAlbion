use std::os::raw::c_long;

#[derive(Debug)]
#[repr(C)]
pub struct CTimer {
    pub timer_index: c_long,
}
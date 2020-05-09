use std::os::raw::c_long;

#[derive(Debug)]
#[repr(C)]
pub struct CCounter {
    pub relevant_world_frame: c_long,
}
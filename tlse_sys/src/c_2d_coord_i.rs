use std::os::raw::c_long;

#[derive(Debug)]
#[repr(C)]
pub struct C2DCoordI {
    pub x: c_long,
    pub y: c_long,
}
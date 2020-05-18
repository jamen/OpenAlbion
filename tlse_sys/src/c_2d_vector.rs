use std::os::raw::c_float;

#[derive(Debug)]
#[repr(C)]
pub struct C2DVector {
    pub x: c_float,
    pub y: c_float,
}
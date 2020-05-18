use std::os::raw::c_float;

#[derive(Debug)]
#[repr(C)]
pub struct C3DVector {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}
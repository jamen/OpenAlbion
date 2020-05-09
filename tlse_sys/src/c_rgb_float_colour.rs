use std::os::raw::c_float;

#[derive(Debug)]
#[repr(C)]
pub struct CRGBFloatColour {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}
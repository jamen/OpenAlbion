use std::os::raw::c_float;

/// Top left and bottom right positions.
#[derive(Debug)]
#[repr(C)]
pub struct C2DBoxF {
    pub tl_x: c_float,
    pub tl_y: c_float,
    pub br_x: c_float,
    pub br_y: c_float,
}
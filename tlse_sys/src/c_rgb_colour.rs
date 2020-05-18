pub use std::os::raw::{c_uchar,c_ulong};

#[derive(Debug)]
#[repr(C)]
pub struct CRGBColour {
    pub b: c_uchar,
    pub g: c_uchar,
    pub r: c_uchar,
    pub a: c_uchar,
    // pub int_value: c_ulong,
}
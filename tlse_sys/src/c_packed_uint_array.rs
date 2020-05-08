use std::os::raw::{c_ulong,c_uchar};

#[derive(Debug)]
#[repr(C)]
pub struct CPackedUIntArray {
    pub packed_ints: *mut c_ulong,
    pub size: c_ulong,
    pub bits: c_uchar,
    pub bias: c_ulong,
}

impl CPackedUIntArray {
}
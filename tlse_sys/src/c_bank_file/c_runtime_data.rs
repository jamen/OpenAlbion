use std::os::raw::{c_ulong,c_uchar};

#[repr(C)]
pub struct CRuntimeData {
    pub data_offset: c_ulong,
    pub data_size: c_ulong,
    pub data_type: c_uchar,
    pub valid: bool,
}

impl CRuntimeData {
}
use std::os::raw::{c_ulong,c_float};

#[repr(C)]
pub struct CRoughFrameCounter {
    pub frame_start: c_ulong,
    pub fps: c_float,
}
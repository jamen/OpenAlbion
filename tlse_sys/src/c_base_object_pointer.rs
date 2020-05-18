use std::os::raw::c_ulong;

use crate::CBaseObject;

#[derive(Debug)]
#[repr(C)]
pub struct CBaseObjectPointer {
    pub object: *mut CBaseObject,
    pub ref_count: c_ulong,
}
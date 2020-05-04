use crate::CCharStringData;

#[repr(C)]
pub struct CCharString {
    data: *mut CCharStringData,
}

impl CCharString {
}
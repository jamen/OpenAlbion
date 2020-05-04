use crate::CCPPointerInfo;

#[repr(C)]
pub struct CCountedPointer<T> {
    pub data: *mut T,
    pub info: *mut CCPPointerInfo,
}

impl<T> CCountedPointer<T> {
}
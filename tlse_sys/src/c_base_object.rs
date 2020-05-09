use crate::{CBaseClass,CBaseObjectPointer};

#[derive(Debug)]
#[repr(C)]
pub struct CBaseObject {
    pub vmt: *mut (),
    pub c_base_class: CBaseClass,
    pub intelligent_pointer: *mut CBaseObjectPointer,
}
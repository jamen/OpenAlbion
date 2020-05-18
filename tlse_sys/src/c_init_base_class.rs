use crate::CBaseClass;

#[derive(Debug)]
#[repr(C)]
pub struct CInitBaseClass {
    pub vmt: *mut (),
    pub c_base_class: CBaseClass,
    // pub valid: bool,
}

impl CInitBaseClass {
}
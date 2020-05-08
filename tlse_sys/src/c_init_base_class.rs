use crate::CBaseClass;

#[derive(Debug)]
#[repr(C)]
pub struct CInitBaseClass {
    pub c_base_class: CBaseClass,
    pub valid: bool,
}

impl CInitBaseClass {
}
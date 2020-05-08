use crate::CInitBaseClass;

/// The methods on this are more interesting. Maybe CI = Class Interface?
#[derive(Debug)]
#[repr(C)]
pub struct CIEngine {
    pub vmt: *mut (),
    pub c_init_base_class: CInitBaseClass,
    pub active: bool,
}

impl CIEngine {
}
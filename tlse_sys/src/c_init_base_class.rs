use crate::CBaseClass;

#[repr(C)]
pub struct CInitBaseClass {
    pub inherited_c_base_class: CBaseClass,
    pub valid: bool,
}

impl CInitBaseClass {
}
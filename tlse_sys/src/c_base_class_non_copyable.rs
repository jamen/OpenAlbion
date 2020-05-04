pub use crate::CBaseClass;

#[repr(C)]
pub struct CBaseClassNonCopyable {
    pub c_base_class: CBaseClass,
}

impl CBaseClassNonCopyable {
}
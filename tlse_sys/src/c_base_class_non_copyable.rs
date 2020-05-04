pub use crate::CBaseClass;

#[repr(C)]
pub struct CBaseClassNonCopyable {
    pub inherited_c_base_class: CBaseClass,
}
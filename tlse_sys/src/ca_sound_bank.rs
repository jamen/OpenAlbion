use crate::{CBaseClassNonCopyable,CCountedPointer,CCRCSymbolMap};

#[repr(C)]
pub struct CASoundBank {
    pub inherited_c_base_class_non_copyable: CBaseClassNonCopyable,
    pub p_symbol_map: CCountedPointer<CCRCSymbolMap>,
}

impl CASoundBank {
}
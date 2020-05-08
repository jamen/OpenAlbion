use crate::{CBaseClassNonCopyable,CCountedPointer,CCRCSymbolMap};

#[derive(Debug)]
#[repr(C)]
pub struct CASoundBank {
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub p_symbol_map: CCountedPointer<CCRCSymbolMap>,
}

impl CASoundBank {
}
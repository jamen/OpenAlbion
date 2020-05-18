use std::os::raw::{c_ulong,c_long};

use crate::{
    CBaseClassNonCopyable,
    CVectorMap,
    CKeyPairCompareLess,
};

#[derive(Debug)]
#[repr(C)]
pub struct CCRCSymbolMap {
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub s_long_map: CVectorMap<c_ulong, c_ulong, CKeyPairCompareLess<c_ulong, c_long>>
}

impl CCRCSymbolMap {
}
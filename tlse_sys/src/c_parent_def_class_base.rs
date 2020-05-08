use std::os::raw::c_ulong;

use crate::{CDefClassBase,CVectorMap,CSubDefInfo,CDefString,CKeyPairCompareLess};

#[derive(Debug)]
#[repr(C)]
pub struct CParentDefClassBase {
    pub vmt: *mut (),
    pub c_def_class_base: CDefClassBase,
    pub instantiation_name: CDefString,
    pub sub_def_info_map: CVectorMap<c_ulong, self::CSubDefInfo, CKeyPairCompareLess<c_ulong, self::CSubDefInfo>>,
}
use std::os::raw::{c_ulong,c_uchar};

use crate::{
    CArray,
    CBaseClassNonCopyable,
    CCharString,
    CBankFileEntryUpdateData,
    CPackedUIntArray,
};

#[derive(Debug)]
#[repr(C)]
pub struct CBankFile {
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub size: c_ulong,
    pub symbols: CArray<CCharString>,
    pub checksums: CArray<c_ulong>,
    pub runtime_data: CArray<self::CRuntimeData>,
    pub update_data: CArray<*mut CBankFileEntryUpdateData>,
    pub packed_data_offset: CPackedUIntArray,
}

impl CBankFile {
}

#[derive(Debug)]
#[repr(C)]
pub struct CRuntimeData {
    pub data_offset: c_ulong,
    pub data_size: c_ulong,
    pub data_type: c_uchar,
    pub valid: bool,
}

impl CRuntimeData {
}
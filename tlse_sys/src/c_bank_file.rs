mod c_runtime_data;

pub use c_runtime_data::*;

use std::os::raw::c_ulong;

use crate::{
    CArray,
    CBaseClassNonCopyable,
    CCharString,
    CBankFileEntryUpdateData,
    CPackedUIntArray,
};

#[repr(C)]
pub struct CBankFile {
    pub inherited_c_base_class_non_copyable: CBaseClassNonCopyable,
    pub size: c_ulong,
    pub symbols: CArray<CCharString>,
    pub checksums: CArray<c_ulong>,
    pub runtime_data: CArray<self::CRuntimeData>,
    pub update_data: CArray<*mut CBankFileEntryUpdateData>,
    pub packed_data_offset: CPackedUIntArray,
}
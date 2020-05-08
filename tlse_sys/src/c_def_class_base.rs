use std::os::raw::{c_ulong,c_long};

use crate::{CDefPointeeBase,CDefinitionManager,UnknownEmptyType};

#[derive(Debug)]
#[repr(C)]
pub struct CDefClassBase {
    pub vmt: *mut (),
    pub c_def_pointee_base: CDefPointeeBase,
    pub p_def_manager: CDefinitionManager,
    pub global_index: c_ulong,
    pub setup: UnknownEmptyType,
    pub template: UnknownEmptyType,
    pub default_vals_applied: UnknownEmptyType,
}

#[derive(Debug)]
#[repr(C)]
pub struct CSubDefInfo {
    pub def_index: c_long,
    pub original_parent_def_index: c_long,
}
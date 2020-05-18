use std::os::raw::c_long;

use crate::{CParentDefClassBase,CRGBColour};

#[derive(Debug)]
#[repr(C)]
pub struct CPlayerDef {
    pub vmt: *mut (),
    pub c_parent_def_class_base: CParentDefClassBase,
    pub character_def: c_long,
    pub colour: CRGBColour,
}
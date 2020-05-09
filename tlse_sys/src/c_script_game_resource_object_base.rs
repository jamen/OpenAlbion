use crate::CBaseObject;

#[derive(Debug)]
#[repr(C)]
pub struct CScriptGameResourceObjectBase {
    pub vmt: *mut (),
    pub c_object_base: CBaseObject,
}
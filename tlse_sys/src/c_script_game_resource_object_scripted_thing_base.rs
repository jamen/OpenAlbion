use crate::{CCountedPointer,CScriptGameResourceObjectBase};

/// Whew lad what a class name
#[derive(Debug)]
#[repr(C)]
pub struct CScriptGameResourceObjectScriptedThingBase {
    pub vmt: *mut (),
    pub c_scripted_game_resource_object_base: CScriptGameResourceObjectBase,
    pub p_imp: CCountedPointer<CScriptGameResourceObjectScriptedThingBase>,
}
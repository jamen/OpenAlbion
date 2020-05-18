use crate::{CScriptGameResourceObjectBase,CCountedPointer};

#[derive(Debug)]
#[repr(C)]
pub struct CScriptGameResourceObjectMovieBase {
    pub c_script_game_resource_object_base: CScriptGameResourceObjectBase,
    pub p_imp: CCountedPointer<CScriptGameResourceObjectMovieBase>,
}
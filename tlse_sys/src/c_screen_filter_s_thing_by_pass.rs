use crate::CScriptThing;

#[derive(Debug)]
#[repr(C)]
pub struct CScreenFilterSThingByPass {
    pub thing: CScriptThing,
    pub bypass_set: bool,
}
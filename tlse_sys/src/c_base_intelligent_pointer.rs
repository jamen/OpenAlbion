use crate::CBaseObjectPointer;

#[derive(Debug)]
#[repr(C)]
pub struct CBaseIntelligentPointer {
    pub p_data: *mut CBaseObjectPointer
}
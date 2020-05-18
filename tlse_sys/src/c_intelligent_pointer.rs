use std::marker::PhantomData;

use crate::CBaseIntelligentPointer;

#[derive(Debug)]
#[repr(C)]
pub struct CIntelligentPointer<T> {
    pub vmt: *mut (),
    pub c_base_intelligent_pointer: CBaseIntelligentPointer,
    _elem_type: PhantomData<T>,
}
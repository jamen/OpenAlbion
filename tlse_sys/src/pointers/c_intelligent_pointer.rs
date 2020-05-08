use std::marker::PhantomData;
use std::os::raw::c_ulong;

use crate::CBaseClass;

#[derive(Debug)]
#[repr(C)]
pub struct CIntelligentPointer<T> {
    pub vmt: *mut (),
    pub c_base_intelligent_pointer: CBaseIntelligentPointer,
    _elem_type: PhantomData<T>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseIntelligentPointer {
    pub p_data: *mut CBaseObjectPointer
}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseObjectPointer {
    pub object: *mut CBaseObject,
    pub ref_count: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBaseObject {
    pub vmt: *mut (),
    pub c_base_class: CBaseClass,
    pub intelligent_ptr: *mut CBaseObjectPointer,
}
use std::marker::PhantomData;
use std::os::raw::{c_ulong,c_long};

#[derive(Debug)]
#[repr(C)]
pub struct CDefPointer<T> {
    pub object: *mut CDefPointeeBase,
    _elem_type: PhantomData<T>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CDefPointeeBase {
    pub vmt: *mut (),
    pub c_resource: CResource,
}

#[derive(Debug)]
#[repr(C)]
pub struct CResourceList {
    pub vmt: *mut (),
    pub c_failed_allocation_handler: CFailedAllocationHandler,
    pub head: CResource,
    pub resource_count: c_ulong,
    pub allocated_memory: c_long,
    pub maximum_memory: c_long,
    pub current_frame: c_ulong,
    pub debug_stats_frame: c_ulong,
    pub unloaded_delay: c_ulong,
    pub unload_this_frame: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CResource {
    pub vmt: *mut (),
    pub c_iv_counted_pointee_base: CIVCountedPointeeBase,
    pub resource_list: *mut CResourceList,
    pub prev_resource: *mut CResource,
    pub next_resource: *mut CResource,
    pub resource_size: c_ulong,
    pub last_used_frame: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CIVCountedPointeeBase {
    pub iv_ref_count: c_ulong
}

#[derive(Debug)]
#[repr(C)]
pub struct CFailedAllocationHandler {
}
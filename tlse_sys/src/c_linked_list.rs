use std::marker::PhantomData;
use std::os::raw::{c_long,c_void};

use crate::CBaseClass;

#[derive(Debug)]
#[repr(C)]
pub struct CLinkedList<T> {
    pub head: *mut CLinkedListPosition,
    pub tail: *mut CLinkedListPosition,
    pub entries_count: c_long,
    pub first_scan_info: CLinkedListScanInfo,
    _elem_type: PhantomData<T>
}

impl<T> CLinkedList<T> {
}

#[derive(Debug)]
#[repr(C)]
pub struct CLinkedListPosition {
    pub data: *mut CBaseClass,
    pub next: *mut CLinkedListPosition,
    pub prev: *mut CLinkedListPosition,
    pub list: *mut c_void,
    pub in_list: bool,
}

impl CLinkedListPosition {
}

#[derive(Debug)]
#[repr(C)]
pub struct CLinkedListScanInfo {
    pub current: *mut CLinkedListPosition,
    pub next: *mut CLinkedListPosition,
    pub prev: *mut CLinkedListPosition,
    pub started: bool,
    pub forwards: bool,
    pub next_scan: *mut CLinkedListScanInfo,
    pub prev_scan: *mut CLinkedListScanInfo,
    pub list: *mut c_void,
}


impl CLinkedListScanInfo {
}
use std::marker::PhantomData;

use std::os::raw::{c_ulong,c_char};

#[derive(Debug)]
#[repr(C)]
pub struct StdSet<T, C = super::StdLess<T>, A = super::StdAllocator<T>> {
    pub comp: C,
    pub head: *mut StdSetNode,
    pub size: u32,
    pub alloc_node: super::StdAllocator<StdSet<T, C, A>>,
    pub alloc_value: A,
    _elem_type: PhantomData<T>,
}

pub struct StdSetNode {
    pub left: *mut StdSetNode,
    pub parent: *mut StdSetNode,
    pub right: *mut StdSetNode,
    // Should this be a type parameter?
    pub val: c_ulong,
    pub color: c_char,
    pub is_nil: c_char,
}
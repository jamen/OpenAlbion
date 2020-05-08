use super::StdAllocator;

#[derive(Debug)]
#[repr(C)]
pub struct StdList<T, A = super::StdAllocator<T>> {
    pub proxy: *mut (),
    pub head: *mut StdListNode<T>,
    pub size: u32,
    pub _alloc_node: StdAllocator<StdListNode<T>>,
    pub _alloc_value: A,
}

#[derive(Debug)]
#[repr(C)]
pub struct StdListNode<T> {
    pub next: *mut StdListNode<T>,
    pub prev: *mut StdListNode<T>,
    pub value: T,
}
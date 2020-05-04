use crate::cxx::StdAllocator;

#[repr(C)]
pub struct StdVector<T, A = StdAllocator<T>> {
    pub first: *mut T,
    pub last: *mut T,
    pub end: *mut T,
    pub allocator: A,
}
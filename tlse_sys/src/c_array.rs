use crate::cxx::{StdVector,StdAllocator};

#[derive(Debug)]
#[repr(C)]
pub struct CArray<T> {
    pub cxx_std_vector: StdVector<T, StdAllocator<T>>,
}

impl<T> CArray<T> {
}
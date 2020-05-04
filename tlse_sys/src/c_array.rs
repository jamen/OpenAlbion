use crate::cxx::{StdVector,StdAllocator};

#[repr(C)]
pub struct CArray<T> {
    pub inherited_cxx_std_vector: StdVector<T, StdAllocator<T>>,
}

impl<T> CArray<T> {
}
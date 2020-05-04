use std::marker::PhantomData;

#[repr(C)]
pub struct StdAllocator<T> {
    t: PhantomData<T>,
}
use std::marker::PhantomData;

#[derive(Debug)]
#[repr(C)]
pub struct StdAllocator<T> {
    t: PhantomData<T>,
}
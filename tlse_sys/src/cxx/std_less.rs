use std::marker::PhantomData;

#[derive(Debug)]
#[repr(C)]
pub struct StdLess<T> {
    _elem_type: PhantomData<T>
}
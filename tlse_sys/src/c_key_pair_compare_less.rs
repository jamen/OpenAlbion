use std::marker::PhantomData;

#[derive(Debug)]
#[repr(C)]
pub struct CKeyPairCompareLess<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

impl<A, B> CKeyPairCompareLess<A, B> {
}
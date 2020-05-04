use std::marker::PhantomData;

#[repr(C)]
pub struct CKeyPairCompareLess<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

impl<A, B> CKeyPairCompareLess<A, B> {
}
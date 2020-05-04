use std::marker::PhantomData;

/// This has no fields and only exists for static methods.
#[repr(C)]
pub struct StdCharTraits<A> {
    _value_type: PhantomData<A>
}
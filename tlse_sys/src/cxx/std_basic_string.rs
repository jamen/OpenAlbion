use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::cxx;

use winapi::shared::basetsd::UINT32;

/// This only works when the value type is WCHAR because the buffer and alias sizes are 16 / 2.
/// If Rust becomes smarter then `[A; 16 / mem::size_of::<A>()]` could be possible.
#[repr(C)]
pub struct StdBasicString<
    A: Sized,
    B = cxx::StdCharTraits<A>,
    C = cxx::StdAllocator<A>
> {
    pub buf: [A; 8],
    pub ptr: *mut A,
    pub alias: [c_char; 8],
    pub size: UINT32,
    pub res: UINT32,
    pub alloc: C,
    _traits: PhantomData<B>,
}
use std::fmt;

use crate::cxx::StdAllocator;

#[repr(C)]
pub struct StdVector<T, A = StdAllocator<T>> {
    pub first: *mut T,
    pub last: *mut T,
    /// This marks the capacity of the vector. The "last" field marks the last element.
    pub end: *mut T,
    pub allocator: A,
}

impl<T, A> StdVector<T, A> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.first, self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.first, self.len()) }
    }

    pub fn len(&self) -> usize {
        Self::ptr_offset_from(self.first, self.last)
    }

    pub fn capacity(&self) -> usize {
        Self::ptr_offset_from(self.first, self.end)
    }

    // Using this scary method until ptr_offset_from is stabilized.
    // https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from
    fn ptr_offset_from(a: *mut T, b: *mut T) -> usize {
        let size = std::mem::size_of::<T>();
        if size == 0 { return 0 }
        debug_assert!(!a.is_null() && !b.is_null(), "The pointers must be non-null.");
        debug_assert!(a <= b, "The first pointer cannot come after the second pointer.");
        let dist = b as usize - a as usize;
        dist / size
    }
}

impl<T: fmt::Debug, A> fmt::Debug for StdVector<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
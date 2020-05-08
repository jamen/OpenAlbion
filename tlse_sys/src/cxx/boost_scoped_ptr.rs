use std::ops::{Deref,DerefMut};

#[derive(Debug)]
#[repr(C)]
pub struct BoostScopedPtr<T> (pub *mut T);

impl<T> Deref for BoostScopedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for BoostScopedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}
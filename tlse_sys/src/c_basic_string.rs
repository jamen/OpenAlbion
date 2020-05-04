use std::marker::PhantomData;
use std::os::raw::{c_char,c_ulong};

#[repr(C)]
pub struct CBasicString<T> {
    pub p_data: *mut c_char,
    pub string_length: c_ulong,
    /// This type was unnamed but 4 bytes long.
    pub data_length: u32,
    /// This type was unnamed but 4 bytes long.
    pub use_fast_extend: u32,
    internal: PhantomData<T>,
}
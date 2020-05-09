use std::marker::PhantomData;
use std::os::raw::{c_char,c_long};

use crate::CBasicString;

#[derive(Debug)]
#[repr(C)]
pub struct CCharString {
    data: *mut CCharStringData,
}

#[derive(Debug)]
#[repr(C)]
pub struct CCharStringData {
    pub data: CBasicString<c_char>,
    pub refs_count: c_long,
}

impl CCharString {
    pub fn new(mut s: String) -> CCharString {
        CCharString {
            data: &mut CCharStringData {
                data: CBasicString {
                    p_data: s.as_mut_ptr() as *mut c_char,
                    string_length: s.len() as u32,
                    data_length: s.as_bytes().len() as u32,
                    use_fast_extend: 0,
                    elem_type: PhantomData,
                },
                refs_count: 0,
            } as *mut CCharStringData
        }
    }
}

// impl CCharString {
//     fn as_str(&self) -> &str {
//         let buf = self.data.data.p_data as *mut u8;
//         let len = self.data.data.string_length;
//     }
// }
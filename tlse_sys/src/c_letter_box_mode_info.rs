use std::os::raw::c_float;

use crate::CFadeInFadeOutBase;

#[derive(Debug)]
#[repr(C)]
pub struct CLetterBoxModeInfo {
    pub c_fade_in_fade_out_base: CFadeInFadeOutBase,
    pub ratio: c_float,
}
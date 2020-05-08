use std::os::raw::c_float;

use crate::CRGBColour;

#[derive(Debug)]
#[repr(C)]
pub struct CFadeInFadeOutBase {
    pub active: bool,
    pub fade_in_time: c_float,
    pub fade_out_time: c_float,
    pub closing: bool,
    pub opening: bool,
    pub open_timer: c_float,
    pub close_timer: c_float,
    pub to_colour: CRGBColour,
}
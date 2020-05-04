use std::os::raw::{c_long,c_char,c_uchar};

#[repr(C)]
pub struct CGameEvent {
    pub event_type: c_long,
    pub player: c_char,
    pub data: [c_uchar; 32],
    pub end_pos: c_uchar,
    pub valid: bool,
    pub replacement: bool,
}
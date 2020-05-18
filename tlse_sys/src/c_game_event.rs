use std::os::raw::{c_long,c_char,c_uchar};

#[derive(Debug)]
#[repr(C)]
pub struct CGameEvent {
    pub event_type: c_long,
    pub player: c_char,
    pub data: [c_uchar; 32],
    pub end_pos: c_uchar,
    /// Many of the events have data that is invalid, e.g. the event_type and data are seemingly random.
    /// This flag tells the game the event is valid and in use.
    pub valid: bool,
    /// I think this indicates whether the event should be replaced?
    /// Some events that were seemingly valid, but marked invalid, are marked with replacement.
    pub replacement: bool,
}
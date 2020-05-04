use std::os::raw::{c_long,c_ulong};

use crate::CGameEvent;

#[repr(C)]
pub struct CGameEventPackage {
    pub timestamp: c_long,
    pub no_events: c_ulong,
    pub events: [CGameEvent; 1600],
}
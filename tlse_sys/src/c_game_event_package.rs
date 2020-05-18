use std::os::raw::{c_long,c_ulong};
use std::fmt;

use crate::CGameEvent;

#[repr(C)]
pub struct CGameEventPackage {
    pub timestamp: c_long,
    pub events_count: c_ulong,
    pub events: [CGameEvent; 40],
}

impl fmt::Debug for CGameEventPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CGameEventPackage")
            .field("timestamp", &self.timestamp)
            .field("events_count", &self.events_count)
            .field("events", &&self.events[..])
            .finish()
    }
}
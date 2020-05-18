use std::os::raw::c_uchar;
use std::fmt;

use crate::{CInitBaseClass,CGameComponent};

#[repr(C)]
pub struct CGame {
    pub c_init_base_class: CInitBaseClass,
    pub current_game_component: *mut CGameComponent,
    pub parameter_buffer: [c_uchar; 512],
    pub quit: bool,
}

impl CGame {
}

impl fmt::Debug for CGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CGame")
            .field("c_init_base_class", &self.c_init_base_class)
            .field("current_game_component", &self.current_game_component)
            .field("parameter_buffer", &&self.parameter_buffer[..])
            .field("quit", &self.quit)
            .finish()
    }
}
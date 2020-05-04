use std::os::raw::c_uchar;

use crate::{CInitBaseClass,CGameComponent};

#[repr(C)]
pub struct CGame {
    pub inherited_c_init_base_class: CInitBaseClass,
    pub current_game_component: *mut CGameComponent,
    pub parameter_buffer: [c_uchar; 512],
    pub quit: bool,
}

impl CGame {
}
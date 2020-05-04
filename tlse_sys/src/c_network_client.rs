use std::os::raw::{c_uchar,c_long,c_ulong,c_double};

use crate::{
    CInitBaseClass,
    CNetworkServer,
    CGameEventPackage,
    CNetworkPlayer,
    CMainGameComponent,
};

#[repr(C)]
pub struct CNetworkClient {
    pub c_init_base_class: CInitBaseClass,
    pub server: CNetworkServer,
    pub receive_buffer: [c_uchar; 8192],
    pub local_event_package: CGameEventPackage,
    pub last_update_time: c_double,
    pub first_time: bool,
    pub host: bool,
    pub local_game: bool,
    pub local_player: *const CNetworkPlayer,
    pub host_player: *const CNetworkPlayer,
    pub local_frame: c_long,
    pub checksum1: c_ulong,
    pub checksum2: c_ulong,
    pub game_component: *mut CMainGameComponent,
}
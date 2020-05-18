use crate::{CBaseClassNonCopyable,CDeviceResetCallback,CGame};

#[derive(Debug)]
#[repr(C)]
pub struct CGameComponent {
    pub vmt: *mut (),
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub c_device_reset_callback: CDeviceResetCallback,
    pub quit: bool,
    pub running: bool,
    pub game: *mut CGame,
}

impl CGameComponent {
}
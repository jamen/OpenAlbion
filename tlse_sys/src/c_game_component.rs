use crate::{CBaseClassNonCopyable,CDeviceResetCallback,CGame};

#[repr(C)]
pub struct CGameComponent {
    pub inherited_c_base_class_non_copyable: CBaseClassNonCopyable,
    pub inherited_c_device_reset_callback: CDeviceResetCallback,
    pub quit: bool,
    pub running: bool,
    pub game: *mut CGame,
}

impl CGameComponent {
}
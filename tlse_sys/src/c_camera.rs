use std::os::raw::{c_float,c_ulong};

use crate::{C3DVector,CRightHandedSet};

#[derive(Debug)]
#[repr(C)]
pub struct CCamera {
    pub world_position: C3DVector,
    pub rh_set: CRightHandedSet,
    pub height_locked: bool,
    pub view_vec_z_locked: bool,
    pub fov_flags: c_ulong,
    pub horizontal_fov: c_float,
    pub vertical_fov: c_float,
    pub zoom: c_float,
}
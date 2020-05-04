use std::os::raw::c_ulong;

mod c_interpolation_info_set;

pub use c_interpolation_info_set::*;

#[repr(C)]
pub struct CInterpolationInfo {
    pub current: CInterpolationInfoSet,
    pub last: CInterpolationInfoSet,
    pub paused_current: CInterpolationInfoSet,
    pub paused_last: CInterpolationInfoSet,
    pub bullet_time_current: CInterpolationInfoSet,
    pub bullet_time_last: CInterpolationInfoSet,
    pub wf_server_current_time: c_ulong,
}
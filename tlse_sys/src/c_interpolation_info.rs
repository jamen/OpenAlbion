use std::os::raw::{c_ulong,c_float};

#[derive(Debug)]
#[repr(C)]
pub struct CInterpolationInfo {
    pub current: self::CInterpolationInfoSet,
    pub last: self::CInterpolationInfoSet,
    pub paused_current: self::CInterpolationInfoSet,
    pub paused_last: self::CInterpolationInfoSet,
    pub bullet_time_current: self::CInterpolationInfoSet,
    pub bullet_time_last: self::CInterpolationInfoSet,
    pub wf_server_current_time: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInterpolationInfoSet {
    gt_predicted_time_since_last_render_frame: c_float,
    wf_interpolate: c_float,
}
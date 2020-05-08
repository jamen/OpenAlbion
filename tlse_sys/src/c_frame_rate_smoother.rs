use std::os::raw::{c_float,c_long,c_double};

use crate::{CInitBaseClass,CArray};

#[derive(Debug)]
#[repr(C)]
pub struct CFrameRateSmoother {
    pub vmt: *mut (),
    pub c_init_base_class: CInitBaseClass,
    pub times: CArray<c_float>,
    pub max_no_times: c_long,
    pub no_times: c_long,
    pub no_frames_to_change_frame_rate_over: c_long,
    pub first_time: c_long,
    pub last_time: c_long,
    pub smoothed_time: c_double,
}
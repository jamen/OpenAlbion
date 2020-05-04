use std::os::raw::c_float;

#[repr(C)]
pub struct CInterpolationInfoSet {
    gt_predicted_time_since_last_render_frame: c_float,
    wf_interpolate: c_float,
}
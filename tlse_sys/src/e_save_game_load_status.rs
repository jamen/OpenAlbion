#[derive(Debug)]
#[repr(C)]
pub enum ESaveGameLoadStatus {
    SAVE_LOAD_STATUS_NONE = 0,
    SAVE_LOAD_STATUS_FADE_OUT = 1,
    SAVE_LOAD_STATUS_FADING_OUT = 2,
    SAVE_LOAD_STATUS_LOADING = 3,
    SAVE_LOAD_STATUS_FADE_IN = 4,
}
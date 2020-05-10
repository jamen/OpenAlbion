#[derive(Debug)]
#[repr(C)]
pub enum ESaveType {
    SAVE_TYPE_NONE = 0,
    SAVE_TYPE_MANUAL_SAVE = 1,
    SAVE_TYPE_AUTO_SAVE = 2,
    SAVE_TYPE_QUEST_START_SAVE = 3,
}
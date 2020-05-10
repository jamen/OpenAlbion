#[derive(Debug)]
#[repr(C)]
pub enum EProcessedEventType {
    PROCESSED_INPUT_NULL = 0,
    PROCESSED_INPUT_GAME_EVENTS = 1,
    PROCESSED_INPUT_PERFORMED_EVENT = 2,
}
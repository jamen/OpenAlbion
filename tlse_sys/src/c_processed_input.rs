use std::os::raw::{c_long,c_char};

use crate::CGameEvent;

#[derive(Debug)]
#[repr(C)]
pub struct CProcessedInput {
    pub player: c_long,
    pub event_type: EProcessedEventType,
    pub game_events: [CGameEvent; 4],
    pub game_events_count: c_char,
    pub priority: EGameEventPriority,
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum EProcessedEventType {
    PROCESSED_INPUT_NULL = 0,
    PROCESSED_INPUT_GAME_EVENTS = 1,
    PROCESSED_INPUT_PERFORMED_EVENT = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum EGameEventPriority {
    GAME_EVENT_PRIORITY_NULL = 0,
    GAME_EVENT_PRIORITY_MIN = 1,
    GAME_EVENT_PRIORITY_LOW = 2,
    GAME_EVENT_PRIORITY_MEDIUM = 3,
    GAME_EVENT_PRIORITY_HIGH = 4,
    GAME_EVENT_PRIORITY_MAX = 5,
}
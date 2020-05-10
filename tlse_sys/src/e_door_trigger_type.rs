#[derive(Debug)]
#[repr(C)]
pub enum EDoorTriggerType {
	DOOR_TRIGGER_ON_PERSON = 0,
	DOOR_TRIGGER_MANUAL = 1,
}
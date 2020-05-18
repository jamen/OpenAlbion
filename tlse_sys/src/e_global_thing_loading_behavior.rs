#[derive(Debug)]
#[repr(C)]
pub enum EGlobalThingLoadingBehaviour {
    LOAD_ON_STARTUP = 1,
    LOAD_PER_LEVEL = 2,
}
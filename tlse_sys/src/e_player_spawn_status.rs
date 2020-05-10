#[derive(Debug)]
#[repr(C)]
pub enum EPlayerSpawnStatus {
    PLAYER_SPAWN_STATUS_NULL = 0,
    PLAYER_SPAWN_STATUS_START = 1,
    PLAYER_SPAWN_STATUS_END = 2,
}
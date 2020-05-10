use std::os::raw::{c_float,c_long};

use crate::cxx;
use crate::{
    C3DVector,
    CDefPointer,
    CHeroCombatDef,
    CPlayerMovementDef,
    CThingCreatureBase,
    EControlledMovementType,
    EGameAction,
    EMovementBandType,
    UnknownEmptyType,
};

#[derive(Debug)]
#[repr(C)]
pub struct CThingPlayerCreature {
    pub vmt: *mut (),
    pub c_thing_creature_base: CThingCreatureBase,
    pub control_pos: C3DVector,
    pub control_move_by_vector: C3DVector,
    pub moved_by_player: UnknownEmptyType,
    pub jumping: UnknownEmptyType,
    /// An unknown type with 4 bytes.
    pub pad1: u32,
    pub controlled_movement_type: EControlledMovementType,
    pub movement_acceleration: C3DVector,
    pub max_slow_walking_speed: c_long,
    pub max_walking_speed: c_float,
    pub max_jogging_speed: c_float,
    pub max_running_speed: c_float,
    pub max_springing_speed: c_float,
    pub max_flying_speed: c_float,
    pub max_strafing_speed: c_float,
    pub relative_movement_acceleration_components: cxx::StdMap<EGameAction, cxx::StdPair<C3DVector, c_long>, cxx::StdLess<EGameAction>>,
    pub relative_movement_acceleration: C3DVector,
    pub last_relative_movement_acceleration: C3DVector,
    pub collided_with_thing: bool,
    pub last_thing_collision_normal: C3DVector,
    pub last_thing_collision_position: C3DVector,
    pub p_hero_combat_def: CDefPointer<CHeroCombatDef>,
    pub p_player_movement_def: CDefPointer<CPlayerMovementDef>,
    pub impulse_velocity: C3DVector,
    pub lean_angle: c_float,
    pub movement_band_type: EMovementBandType,
}
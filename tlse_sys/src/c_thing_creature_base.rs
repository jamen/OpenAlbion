// use std::os::raw::{c_ulong,c_long,c_float,c_char};

// use crate::cxx;
// use crate::{
//     C3DVector,
//     CCountedPointer,
//     CCreatureActionBase,
//     CCreatureInteraction,
//     CDefPointer,
//     CEnginePrimitiveHandle,
//     CIntelligentPointer,
//     CTCCreatureModeManager,
//     CThing,
//     CThingBodyReorienter,
//     CThingCreatureDef,
//     UnknownEmptyType,
// };

#[derive(Debug)]
#[repr(C)]
pub struct CThingCreatureBase {
    // pub c_thing_game_object: CThingGameObject,
    // pub vmt: *mut (),
    // pub last_message_event_i_created_id: c_ulong,
    // pub combat_collision_debug_graphics: cxx::StdVector<CEnginePrimitiveHandle>,
    // pub p_def: CDefPointer<CThingCreatureDef>,
    // pub shot_accuracy_percentage: c_long,
    // pub initial_pos: C3DVector,
    // pub p_last_attacked_by_creature: CIntelligentPointer<CThingCreatureBase>,
    // pub wf_last_attacked_by_Creature: c_ulong,
    // pub p_current_action: CCountedPointer<CCreatureActionBase>,
    // pub p_queued_actions: cxx::StdList<CCountedPointer<CCreatureActionBase>>,
    // pub movement_vector: C3DVector,
    // pub head_pos_offset: C3DVector,
    // pub idle_counter: c_long,
    // pub turn_speed: c_float,
    // pub p_creature_interaction: CCountedPointer<CCreatureInteraction>,
    // pub p_tc_mode_manager: CTCCreatureModeManager,
    // pub previous_action_handedness: ECombatAnimationHandedness,
    // pub previous_action_handedness_wd: c_long,
    // pub body_reorienter: cxx::BoostScopedPtr<CThingBodyReorienter>,
    // pub combat_debug_graphics: cxx::StdVector<CEnginePrimitiveHandle>,
    // pub p_item_to_unseathe_after_cutscene: CIntelligentPointer<CThing>,
    // /// c_rchar
    // pub debug_text: c_char,
    // pub currently_frame_updating: UnknownEmptyType,
    // pub blinking: UnknownEmptyType,
    // pub melee_attacker: UnknownEmptyType,
    // pub melee_attacked: UnknownEmptyType,
    // pub leave_dead_body_override: UnknownEmptyType,
    // pub leave_experience_orbs_override: UnknownEmptyType,
    // pub follow_hero_through_teleporters: UnknownEmptyType,
    // // Unknown one byte type
    // pub head_pos_cached: u8,
    // pub use_movement_in_actions: UnknownEmptyType,
    // pub update_movement_vector_this_frame: UnknownEmptyType,
    // pub aborting_current_action: UnknownEmptyType,
    // pub demon_door: UnknownEmptyType,
    // pub oracle: UnknownEmptyType,
    // pub had_facing_angle_y_z_set_by_body_orientation: UnknownEmptyType,
    // pub under_influence_of_epic_spell: UnknownEmptyType,
    // // Unknown one byte type
    // pub hidden_on_mini_map: u8,
    // pub is_showing_debug_info: UnknownEmptyType,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum ECombatAnimationHandedness {
    HANDED_RIGHT = 0,
    HANDED_LEFT = 1,
    HANDED_NONE = 2,
}

pub struct CThingGameObject {
    // pub c_thing_physical: CThingPhysical,
    // pub vmt: *mut (),
    // pub p_thing_standing_on: CIntelligentPointer<CThing>,
    // pub add_to_combo_multiplier_on_hit: UnknownEmptyType,
    // pub add_to_combo_multiplier_on_hit_override_set: UnknownEmptyType,
    // pub give_hero_stat_changes_on_being_hit: UnknownEmptyType,
    // pub give_hero_stat_changes_on_being_hit_override_set: UnknownEmptyType,
}

pub struct CThingPhysical {
    // pub c_thing: CThing,

}
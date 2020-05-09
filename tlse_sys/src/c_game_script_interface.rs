#![allow(non_camel_case_types)]

use std::os::raw::{c_long,c_void,c_ulong,c_float,c_char};

use crate::cxx;
use crate::{
    C2DCoordI,
    C2DVector,
    C3DVector,
    CArray,
    CCharString,
    CCounter,
    CDisplayEngine,
    CGameDefinitionManager,
    CGameScriptInterfaceBase,
    CMainGameComponent,
    CPlayerManager,
    CRGBColour,
    CRGBFloatColour,
    CRightHandedSet,
    CScreenFilterSThingByPass,
    CScriptGameResourceObjectMovieBase,
    CScriptGameResourceObjectScriptedThingBase,
    CScriptThing,
    CThing,
    CThingSearchTools,
    CTimer,
    CWideString,
    CWorld,
    NEntityEvents,
};

#[derive(Debug)]
#[repr(C)]
pub struct CGameScriptInterface {
    pub vmt: *mut CGameScriptInterfaceVmt,
    pub c_game_script_interface_base: CGameScriptInterfaceBase,
    pub world: *mut CWorld,
    pub component: *mut CMainGameComponent,
    pub display_engine: *mut CDisplayEngine,
    pub definition_manager: *const CGameDefinitionManager,
    pub player_manager: *mut CPlayerManager,
    pub thing_search_tools: *const CThingSearchTools,
    pub current_player: c_long,
    pub current_level_id: c_long,
    pub current_script_level_id: c_long,
    pub current_script_id: c_long,
    pub in_movie_sequence: bool,
    pub alow_screen_fading_if_already_faded: bool,
    pub hero_script_thing: CScriptThing,
    pub script_timers: cxx::StdMap<c_long, c_long>,
    pub camera_rset_to_view_behind_hero_count: c_long,
    pub create_creature_delay_frames: c_long,
}

impl CGameScriptInterface {
    pub fn end_letter_box(&mut self) {
        unsafe { ((*self.vmt).end_letter_box)(self as *mut CGameScriptInterface) }
    }
    pub fn error(&mut self, a: *const CCharString, b: *const CCharString, c: c_ulong) {
        unsafe { ((*self.vmt).error)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn trace_message(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).trace_message)(self as *mut CGameScriptInterface, a) }
    }
    pub fn validate(&mut self) {
        unsafe { ((*self.vmt).validate)(self as *mut CGameScriptInterface) }
    }
    pub fn set_debug_camera_type(&mut self, a: c_long) {
        unsafe { ((*self.vmt).set_debug_camera_type)(self as *mut CGameScriptInterface, a) }
    }
    pub fn deactivate_boast_ui(&mut self) {
        unsafe { ((*self.vmt).deactivate_boast_ui)(self as *mut CGameScriptInterface) }
    }
    pub fn is_xbox(&mut self) -> bool {
        unsafe { ((*self.vmt).is_xbox)(self as *mut CGameScriptInterface) }
    }
    pub fn new_script_frame(&mut self) {
        unsafe { ((*self.vmt).new_script_frame)(self as *mut CGameScriptInterface) }
    }
    pub fn start_scripting_entity(&mut self, a: *const CScriptThing, b: *mut CScriptGameResourceObjectScriptedThingBase, c: EScriptAIPriority) -> bool {
        unsafe { ((*self.vmt).start_scripting_entity)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn is_entity_under_scripted_control(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_under_scripted_control)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_active_thread_terminating(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).is_active_thread_terminating)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_level_loaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_level_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_region_loaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_region_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_region_loaded_and_preloaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_region_loaded_and_preloaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_region_def_loaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_region_def_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_region_name(&mut self) -> *const CCharString {
        unsafe { ((*self.vmt).get_region_name)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_is_level_loaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_is_level_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_is_level_unloaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_is_level_unloaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_level_loaded(&mut self, a: *mut cxx::StdList<CCharString>) -> bool {
        unsafe { ((*self.vmt).msg_on_level_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_level_unloaded(&mut self, a: *mut cxx::StdList<CCharString>) -> bool {
        unsafe { ((*self.vmt).msg_on_level_unloaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_is_region_loaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_is_region_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_is_region_unloaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_is_region_unloaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_region_loaded(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_region_loaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_region_unloaded(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_region_unloaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_region_preunloaded(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_region_preunloaded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_quest_completed(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_quest_completed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_any_quest_completed(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_any_quest_completed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_quest_failed(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_quest_failed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_quest_completed_before_screen_shown(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_quest_completed_before_screen_shown)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_quest_failed_before_screen_shown(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_quest_failed_before_screen_shown)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_quest_accepted(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_quest_accepted)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_feat_accepted(&mut self, a: *mut c_long) -> bool {
        unsafe { ((*self.vmt).msg_on_feat_accepted)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_is_boast_made(&mut self, a: c_long) -> bool {
        unsafe { ((*self.vmt).msg_is_boast_made)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_boast_made(&mut self, a: *mut c_long, b: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_boast_made)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn msg_on_boasts_made(&mut self, a: *mut cxx::StdVector<CCharString>) -> bool {
        unsafe { ((*self.vmt).msg_on_boasts_made)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_boast_message(&mut self) {
        unsafe { ((*self.vmt).remove_boast_message)(self as *mut CGameScriptInterface) }
    }
    pub fn is_quest_start_screen_active(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).is_quest_start_screen_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_leaving_quest_start_screen(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_leaving_quest_start_screen)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_leaving_experience_spending_screen(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_on_leaving_experience_spending_screen)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_is_answered_yes_or_no(&mut self) -> c_long {
        unsafe { ((*self.vmt).msg_is_answered_yes_or_no)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_is_game_info_clicked_past(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_is_game_info_clicked_past)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_is_tutorial_click_past(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_is_tutorial_click_past)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_is_action_mode_button_pressed(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_is_action_mode_button_pressed)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_on_expression_performed(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_expression_performed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_is_cut_scene_skipped(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_is_cut_scene_skipped)(self as *mut CGameScriptInterface) }
    }
    pub fn remove_all_cut_scene_skipped_messages(&mut self, a: *mut CCharString) {
        unsafe { ((*self.vmt).remove_all_cut_scene_skipped_messages)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_hero_hair_type_changed(&mut self, a: EClothingCoversArea, b: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_hair_type_changed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn msg_on_hero_used_teleporter(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_used_teleporter)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_hero_used_guild_seal(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_used_guild_seal)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_on_game_saved_manually(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_on_game_saved_manually)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_on_hero_slept(&mut self, a: *mut bool) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_slept)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_hero_fired_ranged_weapon(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_fired_ranged_weapon)(self as *mut CGameScriptInterface) }
    }
    pub fn msg_on_hero_cast_spell(&mut self, a: *mut EHeroAbility) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_cast_spell)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_hero_picked_pocket(&mut self, a: *mut CScriptThing) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_picked_pocket)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_hero_picked_lock(&mut self, a: *mut CScriptThing) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_picked_lock)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_fishing_game_finished(&mut self, a: *mut CScriptThing) -> bool {
        unsafe { ((*self.vmt).msg_on_fishing_game_finished)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_tavern_game_finished(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).msg_on_tavern_game_finished)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_hero_rewarded_with_items_from(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).msg_on_hero_rewarded_with_items_from)(self as *mut CGameScriptInterface, a) }
    }
    pub fn msg_on_chest_opening_cancelled(&mut self) -> bool {
        unsafe { ((*self.vmt).msg_on_chest_opening_cancelled)(self as *mut CGameScriptInterface) }
    }
    pub fn dont_populate_next_loaded_region(&mut self) {
        unsafe { ((*self.vmt).dont_populate_next_loaded_region)(self as *mut CGameScriptInterface) }
    }
    pub fn get_wandering_population_script_def_name_in_current_region(&mut self, a: *mut CCharString) {
        unsafe { ((*self.vmt).get_wandering_population_script_def_name_in_current_region)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_wandering_population_script_def_name_in_region(&mut self, a: *const CCharString, b: *mut CCharString) {
        unsafe { ((*self.vmt).get_wandering_population_script_def_name_in_region)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_hero_allowed_henchmen_in_current_region(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_allowed_henchmen_in_current_region)(self as *mut CGameScriptInterface) }
    }
    pub fn is_hero_allowed_henchmen_in_region(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_hero_allowed_henchmen_in_region)(self as *mut CGameScriptInterface, a) }
    }
    pub fn post_add_scripted_entities(&mut self) {
        unsafe { ((*self.vmt).post_add_scripted_entities)(self as *mut CGameScriptInterface) }
    }
    pub fn is_player_holding_unsheathe_ranged_weapon_button(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_holding_unsheathe_ranged_weapon_button)(self as *mut CGameScriptInterface) }
    }
    pub fn is_player_holding_lock_target_button(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_holding_lock_target_button)(self as *mut CGameScriptInterface) }
    }
    pub fn is_player_holding_fire_ranged_weapon_button(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_holding_fire_ranged_weapon_button)(self as *mut CGameScriptInterface) }
    }
    pub fn is_player_holding_first_person_targeting_button(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_holding_first_person_targeting_button)(self as *mut CGameScriptInterface) }
    }
    pub fn is_hero_in_projectile_weapon_mode(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_in_projectile_weapon_mode)(self as *mut CGameScriptInterface) }
    }
    pub fn register_timer(&mut self) -> c_long {
        unsafe { ((*self.vmt).register_timer)(self as *mut CGameScriptInterface) }
    }
    pub fn deregister_timer(&mut self, a: c_long) {
        unsafe { ((*self.vmt).deregister_timer)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_timer(&mut self, a: c_long, b: c_long) {
        unsafe { ((*self.vmt).set_timer)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_timer(&mut self, a: c_long) -> c_long {
        unsafe { ((*self.vmt).get_timer)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero(&mut self) -> *mut CScriptThing {
        unsafe { ((*self.vmt).get_hero)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_targeted_thing(&mut self) -> CScriptThing {
        unsafe { ((*self.vmt).get_hero_targeted_thing)(self as *mut CGameScriptInterface) }
    }
    pub fn get_thing_with_script_name(&mut self, a: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_thing_with_script_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_thing_with_script_name_2(&mut self, a: *const CCharString, b: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_thing_with_script_name_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_random_thing_with_script_name(&mut self, a: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_random_thing_with_script_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_all_things_with_script_name(&mut self, a: *const CCharString, b: *mut cxx::StdVector<CScriptThing>) -> c_long {
        unsafe { ((*self.vmt).get_all_things_with_script_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_all_creatures_in_area_with_script_name(&mut self, a: *const CCharString, b: *const C3DVector, c: c_float, d: *const cxx::StdVector<CScriptThing>) -> c_long {
        unsafe { ((*self.vmt).get_all_creatures_in_area_with_script_name)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn get_nearest_with_script_name(&mut self, a: *const CScriptThing, b: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_nearest_with_script_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_furthest_with_script_name(&mut self, a: *const CScriptThing, b: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_furthest_with_script_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_all_things_with_def_name(&mut self, a: *const CCharString, b: *mut cxx::StdVector<CScriptThing>) -> c_long {
        unsafe { ((*self.vmt).get_all_things_with_def_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_all_things_with_def_name_by_distance_from(&mut self, a: *const CScriptThing, b: *const CCharString, c: *mut cxx::StdVector<CScriptThing>) -> c_long {
        unsafe { ((*self.vmt).get_all_things_with_def_name_by_distance_from)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn get_nearest_with_def_name(&mut self, a: *const CScriptThing, b: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_nearest_with_def_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_furthest_with_def_name(&mut self, a: *const CScriptThing, b: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).get_furthest_with_def_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_thing_with_uid(&mut self, a: c_ulong) -> CScriptThing {
        unsafe { ((*self.vmt).get_thing_with_uid)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_all_creatures_excluding_hero(&mut self, a: *mut cxx::StdVector<CScriptThing>) -> c_long {
        unsafe { ((*self.vmt).get_all_creatures_excluding_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_all_things_in_level(&mut self, a: *const CCharString, b: *mut cxx::StdVector<CScriptThing>) -> c_long {
        unsafe { ((*self.vmt).get_all_things_in_level)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_thing_with_this_uid_alive(&mut self, a: c_ulong) -> bool {
        unsafe { ((*self.vmt).is_thing_with_this_uid_alive)(self as *mut CGameScriptInterface, a) }
    }
    pub fn create_creature(&mut self, a: *const CCharString, b: *const C3DVector, c: *const CCharString, d: bool) -> CScriptThing {
        unsafe { ((*self.vmt).create_creature)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn create_creature_nearby(&mut self, a: *const CCharString, b: *const C3DVector, c: c_float, d: *const CCharString, e: bool) -> CScriptThing {
        unsafe { ((*self.vmt).create_creature_nearby)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn create_creature_on_entity(&mut self, a: *const CCharString, b: *const CScriptThing, c: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_creature_on_entity)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn turn_creature_into(&mut self, a: *const CScriptThing, b: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).turn_creature_into)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_creation_delay_frames(&mut self, a: c_long) {
        unsafe { ((*self.vmt).set_creature_creation_delay_frames)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_creature_creation_delay_frames(&mut self) {
        unsafe { ((*self.vmt).reset_creature_creation_delay_frames)(self as *mut CGameScriptInterface) }
    }
    pub fn create_object(&mut self, a: *const CCharString, b: *const C3DVector, c: c_float, d: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_object)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn create_object_2(&mut self, a: *const CCharString, b: *const C3DVector, c: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_object_2)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn create_object_on_entity(&mut self, a: *const CCharString, b: *const CScriptThing, c: *const CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_object_on_entity)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn create_effect(&mut self, a: *const CCharString, b: *const C3DVector, c: *const CCharString, d: c_float, e: bool, f: bool) -> CScriptThing {
        unsafe { ((*self.vmt).create_effect)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
    }
    pub fn create_effect_2(&mut self, a: *const CCharString, b: *const CScriptThing, c: *const CCharString, d: *const CCharString, e: bool, f: bool) -> CScriptThing {
        unsafe { ((*self.vmt).create_effect_2)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
    }
    pub fn create_light(&mut self, a: *const C3DVector, b: *const CRGBColour, c: *const CCharString, d: c_float, e: c_float, f: bool) -> CScriptThing {
        unsafe { ((*self.vmt).create_light)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
    }
    pub fn create_experience_orb(&mut self, a: *const C3DVector, b: c_long) -> CScriptThing {
        unsafe { ((*self.vmt).create_experience_orb)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn create_explosion(&mut self, a: *const CCharString, b: *const C3DVector, c: *const C3DVector, d: CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_explosion)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn create_physical_barrier(&mut self, a: c_float, b: *const C3DVector, c: *const C3DVector, d: CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_physical_barrier)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn create_rumble(&mut self, a: *const C3DVector, b: c_float, c: c_float, d: CCharString) -> CScriptThing {
        unsafe { ((*self.vmt).create_rumble)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn clear_all_rumbles(&mut self) {
        unsafe { ((*self.vmt).clear_all_rumbles)(self as *mut CGameScriptInterface) }
    }
    pub fn remove_thing(&mut self, a: *const CScriptThing, b: bool, c: bool) {
        unsafe { ((*self.vmt).remove_thing)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn show_on_screen_message(&mut self, a: *const C2DVector, b: *const CCharString, c: *const CRGBColour, d: *const CCharString) {
        unsafe { ((*self.vmt).show_on_screen_message)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn show_on_screen_message_2(&mut self, a: *const CCharString, b: *const CCharString, c: *const CCharString, d: c_float) {
        unsafe { ((*self.vmt).show_on_screen_message_2)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn show_on_screen_message_3(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).show_on_screen_message_3)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_screen_message(&mut self, a: *const CCharString, b: ETextGroupSelectionMethod) {
        unsafe { ((*self.vmt).add_screen_message)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_screen_title_message(&mut self, a: *const CCharString, b: c_float, c: bool) {
        unsafe { ((*self.vmt).add_screen_title_message)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn give_hero_yes_no_question(&mut self, a: *const CCharString, b: *const CCharString, c: *const CCharString, d: *const CCharString, e: bool) {
        unsafe { ((*self.vmt).give_hero_yes_no_question)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn display_game_info(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).display_game_info)(self as *mut CGameScriptInterface, a) }
    }
    pub fn display_game_info_text(&mut self, a: *const CWideString) {
        unsafe { ((*self.vmt).display_game_info_text)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_safe_to_display_game_info(&mut self) -> bool {
        unsafe { ((*self.vmt).is_safe_to_display_game_info)(self as *mut CGameScriptInterface) }
    }
    pub fn display_tutorial(&mut self, a: ETutorialCategory) -> bool {
        unsafe { ((*self.vmt).display_tutorial)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_tutorial_system_enabled(&mut self) -> bool {
        unsafe { ((*self.vmt).is_tutorial_system_enabled)(self as *mut CGameScriptInterface) }
    }
    pub fn give_hero_weapon(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).give_hero_weapon)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn give_hero_object(&mut self, a: *const CCharString, b: c_long, c: bool) {
        unsafe { ((*self.vmt).give_hero_object)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn set_weapon_as_heros_active_weapon(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).set_weapon_as_heros_active_weapon)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_hero_item(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).give_hero_item)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_hero_items_from_container(&mut self, a: *const CScriptThing, b: bool) -> bool {
        unsafe { ((*self.vmt).give_hero_items_from_container)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn take_object_from_hero(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).take_object_from_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_hero_gold(&mut self, a: c_long) {
        unsafe { ((*self.vmt).give_hero_gold)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_gold(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_gold)(self as *mut CGameScriptInterface) }
    }
    pub fn give_hero_experience(&mut self, a: c_long) {
        unsafe { ((*self.vmt).give_hero_experience)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_able_to_gain_experience(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_able_to_gain_experience)(self as *mut CGameScriptInterface, a) }
    }
    pub fn sheathe_hero_weapons(&mut self) {
        unsafe { ((*self.vmt).sheathe_hero_weapons)(self as *mut CGameScriptInterface) }
    }
    pub fn set_hero_will_as_usable(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_will_as_usable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_weapons_as_usable(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_weapons_as_usable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_weapon_out_crime_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_weapon_out_crime_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_guards_ignore_crimes(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_guards_ignore_crimes)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_all_hero_weapons(&mut self) {
        unsafe { ((*self.vmt).remove_all_hero_weapons)(self as *mut CGameScriptInterface) }
    }
    pub fn is_reported_or_unreported_crime_known(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_reported_or_unreported_crime_known)(self as *mut CGameScriptInterface, a) }
    }
    pub fn confiscate_all_hero_items(&mut self) {
        unsafe { ((*self.vmt).confiscate_all_hero_items)(self as *mut CGameScriptInterface) }
    }
    pub fn confiscate_all_hero_weapons(&mut self) {
        unsafe { ((*self.vmt).confiscate_all_hero_weapons)(self as *mut CGameScriptInterface) }
    }
    pub fn confiscate_items_of_type_from_hero(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).confiscate_items_of_type_from_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn return_all_confiscated_items_to_hero(&mut self) {
        unsafe { ((*self.vmt).return_all_confiscated_items_to_hero)(self as *mut CGameScriptInterface) }
    }
    pub fn give_hero_tutorial(&mut self, a: ETutorialCategory) -> bool {
        unsafe { ((*self.vmt).give_hero_tutorial)(self as *mut CGameScriptInterface, a) }
    }
    pub fn make_hero_carry_item_in_hand(&mut self, a: *const CScriptThing, b: bool, c: bool) {
        unsafe { ((*self.vmt).make_hero_carry_item_in_hand)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn make_hero_carry_item_in_hand_2(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).make_hero_carry_item_in_hand_2)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_tattoo_to_hero(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).add_tattoo_to_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_hero_ability(&mut self, a: EHeroAbility, b: bool) {
        unsafe { ((*self.vmt).give_hero_ability)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_player_z_targeting_thing(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_player_z_targeting_thing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_player_creature_blocking(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_creature_blocking)(self as *mut CGameScriptInterface) }
    }
    pub fn is_player_creature_ready_to_fire_projectile_weapon(&mut self, a: *mut c_float) -> bool {
        unsafe { ((*self.vmt).is_player_creature_ready_to_fire_projectile_weapon)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_player_creature_combat_multiplier(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_player_creature_combat_multiplier)(self as *mut CGameScriptInterface) }
    }
    pub fn get_player_creature_combat_multiplier_running_num_hits(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_player_creature_combat_multiplier_running_num_hits)(self as *mut CGameScriptInterface) }
    }
    pub fn reset_player_creature_combat_multiplier(&mut self) {
        unsafe { ((*self.vmt).reset_player_creature_combat_multiplier)(self as *mut CGameScriptInterface) }
    }
    pub fn is_player_creature_flourish_enabled(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_creature_flourish_enabled)(self as *mut CGameScriptInterface) }
    }
    pub fn set_player_creature_only_target(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).set_player_creature_only_target)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_player_creature_only_target(&mut self) {
        unsafe { ((*self.vmt).reset_player_creature_only_target)(self as *mut CGameScriptInterface) }
    }
    pub fn respawn_hero(&mut self, a: bool) {
        unsafe { ((*self.vmt).respawn_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_hero_morality(&mut self, a: c_float) {
        unsafe { ((*self.vmt).give_hero_morality)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_morality(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_morality)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_morality_category(&mut self) -> EMorality {
        unsafe { ((*self.vmt).get_hero_morality_category)(self as *mut CGameScriptInterface) }
    }
    pub fn change_hero_morality_due_to_theft(&mut self) {
        unsafe { ((*self.vmt).change_hero_morality_due_to_theft)(self as *mut CGameScriptInterface) }
    }
    pub fn change_hero_morality_due_to_picklock(&mut self) {
        unsafe { ((*self.vmt).change_hero_morality_due_to_picklock)(self as *mut CGameScriptInterface) }
    }
    pub fn give_hero_renown_points(&mut self, a: c_long) {
        unsafe { ((*self.vmt).give_hero_renown_points)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_renown_level(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_renown_level)(self as *mut CGameScriptInterface) }
    }
    pub fn is_hero_renown_level_full(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_renown_level_full)(self as *mut CGameScriptInterface) }
    }
    pub fn increase_hero_renown_level(&mut self) {
        unsafe { ((*self.vmt).increase_hero_renown_level)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_strength_level(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_strength_level)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_skill_level(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_skill_level)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_will_level(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_will_level)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_stat_level(&mut self, a: EHeroTrainableStatType) -> c_long {
        unsafe { ((*self.vmt).get_hero_stat_level)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_stat_max(&mut self, a: EHeroTrainableStatType) -> c_long {
        unsafe { ((*self.vmt).get_hero_stat_max)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_age(&mut self, a: c_float) {
        unsafe { ((*self.vmt).set_hero_age)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_age(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_age)(self as *mut CGameScriptInterface) }
    }
    pub fn set_hero_as_teenager(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_as_teenager)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_as_apprentice(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_as_apprentice)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_distance_hero_can_be_heard_from(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_distance_hero_can_be_heard_from)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_rough_experience_level(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_rough_experience_level)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_experience_available_to_spend(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_experience_available_to_spend)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_fatness(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_fatness)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_scariness(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_scariness)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_attractiveness(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_attractiveness)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_will_energy_level(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_will_energy_level)(self as *mut CGameScriptInterface) }
    }
    pub fn set_hero_will_energy_level(&mut self, a: c_float) {
        unsafe { ((*self.vmt).set_hero_will_energy_level)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_will_energy_as_able_to_refill(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_will_energy_as_able_to_refill)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_number_of_items_of_type_in_inventory(&mut self, a: *const CCharString) -> c_long {
        unsafe { ((*self.vmt).get_number_of_items_of_type_in_inventory)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_object_in_things_possession(&mut self, a: *const CCharString, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_object_in_things_possession)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_hero_hand_lamp_lit(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_hand_lamp_lit)(self as *mut CGameScriptInterface) }
    }
    pub fn set_hero_hand_lamp_as_lit(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_hand_lamp_as_lit)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_wearing_clothing_item(&mut self, a: *const CScriptThing, b: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_wearing_clothing_item)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_hero_naked(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_naked)(self as *mut CGameScriptInterface) }
    }
    pub fn remove_hero_clothing(&mut self) {
        unsafe { ((*self.vmt).remove_hero_clothing)(self as *mut CGameScriptInterface) }
    }
    pub fn set_hero_as_wearing(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).set_hero_as_wearing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn change_hero_hairstyle(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).change_hero_hairstyle)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_hero_hairstyle(&mut self) {
        unsafe { ((*self.vmt).remove_hero_hairstyle)(self as *mut CGameScriptInterface) }
    }
    pub fn is_wearing_hairstyle(&mut self, a: *mut CScriptThing, b: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_wearing_hairstyle)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_player_carrying_item_of_type(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_player_carrying_item_of_type)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_player_wielding_weapon(&mut self) -> bool {
        unsafe { ((*self.vmt).is_player_wielding_weapon)(self as *mut CGameScriptInterface) }
    }
    pub fn is_entity_wielding_weapon(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_wielding_weapon)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_wielding_melee_weapon(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_wielding_melee_weapon)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_wielding_ranged_weapon(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_wielding_ranged_weapon)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_previously_wielded_melee_weapon_name(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).get_previously_wielded_melee_weapon_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_previously_wielded_ranged_weapon_name(&mut self, a: *mut CCharString) -> bool {
        unsafe { ((*self.vmt).get_previously_wielded_ranged_weapon_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_able_to_attack(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_able_to_attack)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_get_thing_in_primary_slot(&mut self, a: *const CScriptThing) -> CScriptThing {
        unsafe { ((*self.vmt).entity_get_thing_in_primary_slot)(self as *mut CGameScriptInterface, a) }
    }
    pub fn apply_hero_penalty_for_death(&mut self) {
        unsafe { ((*self.vmt).apply_hero_penalty_for_death)(self as *mut CGameScriptInterface) }
    }
    pub fn give_hero_title(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).give_hero_title)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_title(&mut self) -> EHeroTitle {
        unsafe { ((*self.vmt).get_hero_title)(self as *mut CGameScriptInterface) }
    }
    pub fn entity_set_as_marryable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_marryable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_able_to_region_follow_when_married(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_able_to_region_follow_when_married)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_force_marriage_to_hero(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_force_marriage_to_hero)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_entity_married_to_hero(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_married_to_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_marriable(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_marriable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_has_married(&mut self) -> bool {
        unsafe { ((*self.vmt).get_hero_has_married)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_has_current_marriage(&mut self) -> bool {
        unsafe { ((*self.vmt).get_hero_has_current_marriage)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_has_divorced_marriage(&mut self) -> bool {
        unsafe { ((*self.vmt).get_hero_has_divorced_marriage)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_has_children(&mut self) -> bool {
        unsafe { ((*self.vmt).get_hero_has_children)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_has_murdered_wife(&mut self) -> bool {
        unsafe { ((*self.vmt).get_hero_has_murdered_wife)(self as *mut CGameScriptInterface) }
    }
    pub fn is_hero_child(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_child)(self as *mut CGameScriptInterface) }
    }
    pub fn cancel_hero_teleport_effects(&mut self) {
        unsafe { ((*self.vmt).cancel_hero_teleport_effects)(self as *mut CGameScriptInterface) }
    }
    pub fn get_number_of_times_hero_has_had_sex(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_number_of_times_hero_has_had_sex)(self as *mut CGameScriptInterface) }
    }
    pub fn set_number_of_times_hero_has_had_sex(&mut self, a: c_long) {
        unsafe { ((*self.vmt).set_number_of_times_hero_has_had_sex)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_as_having_had_sex(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_as_having_had_sex)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_as_having_had_gay_sex(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_as_having_had_gay_sex)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_thing_hero_reward_item(&mut self, a: *const CScriptThing, b: *const CCharString, c: *const CCharString) {
        unsafe { ((*self.vmt).give_thing_hero_reward_item)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn give_thing_item_in_hand(&mut self, a: *const CScriptThing, b: *const CCharString, c: bool) {
        unsafe { ((*self.vmt).give_thing_item_in_hand)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn give_thing_item_in_slot(&mut self, a: *const CScriptThing, b: *const CCharString, c: *const CCharString) {
        unsafe { ((*self.vmt).give_thing_item_in_slot)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn give_hero_expression(&mut self, a: *const CCharString, b: c_long, c: bool) {
        unsafe { ((*self.vmt).give_hero_expression)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn hero_has_expression(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).hero_has_expression)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_hero_performing_expression(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_hero_performing_expression)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_d_pad_button_held_for_expression(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_d_pad_button_held_for_expression)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_follow_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: c_float, d: bool) {
        unsafe { ((*self.vmt).entity_follow_thing)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn entity_stop_following(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_stop_following)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_following_entity_list(&mut self, a: *const CScriptThing, b: *mut cxx::StdVector<CScriptThing>) -> bool {
        unsafe { ((*self.vmt).get_following_entity_list)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_perceiving_hero_entity_list(&mut self, a: *mut cxx::StdVector<CScriptThing>) -> bool {
        unsafe { ((*self.vmt).get_perceiving_hero_entity_list)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_summoned_creatures_list(&mut self, a: *mut cxx::StdVector<CScriptThing>) -> bool {
        unsafe { ((*self.vmt).get_hero_summoned_creatures_list)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_following_hero(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_following_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_allowed_to_follow_hero(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_allowed_to_follow_hero)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_allowed_to_change_region_following_state(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_allowed_to_change_region_following_state)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_responding_to_follow_and_wait_expressions(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_responding_to_follow_and_wait_expressions)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_mirroring_hero_enemy_relations_while_following(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_mirroring_hero_enemy_relations_while_following)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn teleport_all_followers_to_hero_position(&mut self) {
        unsafe { ((*self.vmt).teleport_all_followers_to_hero_position)(self as *mut CGameScriptInterface) }
    }
    pub fn entity_teleport_to_hero_position(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_teleport_to_hero_position)(self as *mut CGameScriptInterface, a) }
    }
    pub fn send_entity_event(&mut self, a: NEntityEvents::EEventType, b: *const CScriptThing, c: *const CScriptThing, d: *mut CThing) {
        unsafe { ((*self.vmt).send_entity_event)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn get_water_height_at_position(&mut self, a: *const C3DVector) -> c_float {
        unsafe { ((*self.vmt).get_water_height_at_position)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_fishing_spot_enabled(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_fishing_spot_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn disable_fishing_spot(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).disable_fishing_spot)(self as *mut CGameScriptInterface, a) }
    }
    pub fn update_fish_weight(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).update_fish_weight)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_best_fish_weight(&mut self, a: *const CCharString) -> c_float {
        unsafe { ((*self.vmt).get_best_fish_weight)(self as *mut CGameScriptInterface, a) }
    }
    pub fn hero_go_fishing(&mut self, a: bool) {
        unsafe { ((*self.vmt).hero_go_fishing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_hero_fishing_level(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_fishing_level)(self as *mut CGameScriptInterface) }
    }
    pub fn hero_go_digging(&mut self) {
        unsafe { ((*self.vmt).hero_go_digging)(self as *mut CGameScriptInterface) }
    }
    pub fn hero_stop_digging(&mut self) {
        unsafe { ((*self.vmt).hero_stop_digging)(self as *mut CGameScriptInterface) }
    }
    pub fn hero_play_oracle_minigame(&mut self) {
        unsafe { ((*self.vmt).hero_play_oracle_minigame)(self as *mut CGameScriptInterface) }
    }
    pub fn is_hero_playing_oracle_minigame(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_playing_oracle_minigame)(self as *mut CGameScriptInterface) }
    }
    pub fn has_hero_won_oracle_minigame(&mut self) -> bool {
        unsafe { ((*self.vmt).has_hero_won_oracle_minigame)(self as *mut CGameScriptInterface) }
    }
    pub fn hero_play_fireheart_minigame(&mut self) {
        unsafe { ((*self.vmt).hero_play_fireheart_minigame)(self as *mut CGameScriptInterface) }
    }
    pub fn hero_quit_fireheart_minigame(&mut self) {
        unsafe { ((*self.vmt).hero_quit_fireheart_minigame)(self as *mut CGameScriptInterface) }
    }
    pub fn has_hero_force_quit_fireheart_minigame(&mut self) -> bool {
        unsafe { ((*self.vmt).has_hero_force_quit_fireheart_minigame)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_health(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_health)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_health_max(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_health_max)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_health_percentage(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_hero_health_percentage)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_will_energy(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_will_energy)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_will_energy_max(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_will_energy_max)(self as *mut CGameScriptInterface) }
    }
    pub fn get_hero_will_energy_percentage(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_hero_will_energy_percentage)(self as *mut CGameScriptInterface) }
    }
    pub fn change_hero_health_by(&mut self, a: c_float, b: bool, c: bool) {
        unsafe { ((*self.vmt).change_hero_health_by)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn set_thing_as_killed(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).set_thing_as_killed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_health(&mut self, a: *const CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_health)(self as *mut CGameScriptInterface, a) }
    }
    pub fn modify_thing_health(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
        unsafe { ((*self.vmt).modify_thing_health)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_max_health(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
        unsafe { ((*self.vmt).entity_set_max_health)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn give_hero_new_quest_objective(&mut self, a: *const CCharString, b: c_ulong) {
        unsafe { ((*self.vmt).give_hero_new_quest_objective)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn tell_hero_quest_objective_completed(&mut self, a: c_ulong) {
        unsafe { ((*self.vmt).tell_hero_quest_objective_completed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn tell_hero_quest_objective_failed(&mut self, a: c_ulong) {
        unsafe { ((*self.vmt).tell_hero_quest_objective_failed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_quest_region(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).add_quest_region)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_quest_world_map_offset(&mut self, a: *const CCharString, b: *const C2DCoordI) {
        unsafe { ((*self.vmt).set_quest_world_map_offset)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_hero_on_quest(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_on_quest)(self as *mut CGameScriptInterface) }
    }
    pub fn hero_receive_message_from_guild_master(&mut self, a: *const CCharString, b: *const CCharString, c: bool, d: bool) {
        unsafe { ((*self.vmt).hero_receive_message_from_guild_master)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn set_guild_master_messages(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_guild_master_messages)(self as *mut CGameScriptInterface, a) }
    }
    pub fn activate_quest(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).activate_quest)(self as *mut CGameScriptInterface, a) }
    }
    pub fn activate_multiple_quests(&mut self, a: *const CArray<CCharString>) {
        unsafe { ((*self.vmt).activate_multiple_quests)(self as *mut CGameScriptInterface, a) }
    }
    pub fn activate_quest_without_loading_resources(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).activate_quest_without_loading_resources)(self as *mut CGameScriptInterface, a) }
    }
    pub fn activate_multiple_quests_without_loading_resources(&mut self, a: *const CArray<CCharString>) {
        unsafe { ((*self.vmt).activate_multiple_quests_without_loading_resources)(self as *mut CGameScriptInterface, a) }
    }
    pub fn deactivate_quest(&mut self, a: *const CCharString, b: c_ulong) {
        unsafe { ((*self.vmt).deactivate_quest)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn deactivate_quest_later(&mut self, a: *const CCharString, b: c_ulong) {
        unsafe { ((*self.vmt).deactivate_quest_later)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn prepare_quests_when_final_quest_is_activated(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).prepare_quests_when_final_quest_is_activated)(self as *mut CGameScriptInterface, a) }
    }
    pub fn prepare_quests_when_final_quest_is_completed(&mut self) {
        unsafe { ((*self.vmt).prepare_quests_when_final_quest_is_completed)(self as *mut CGameScriptInterface) }
    }
    pub fn is_quest_active(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_quest_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_quest_registered(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_quest_registered)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_quest_completed(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_quest_completed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_quest_failed(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_quest_failed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_quest_as_completed(&mut self, a: *const CCharString, b: bool, c: bool, d: bool) {
        unsafe { ((*self.vmt).set_quest_as_completed)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn set_quest_as_failed(&mut self, a: *const CCharString, b: bool, c: *const CWideString, d: bool) {
        unsafe { ((*self.vmt).set_quest_as_failed)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn set_quest_as_persistent(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).set_quest_as_persistent)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_exclusive_quest_script_name(&mut self) -> *const CCharString {
        unsafe { ((*self.vmt).get_exclusive_quest_script_name)(self as *mut CGameScriptInterface) }
    }
    pub fn add_quest_card(&mut self, a: *const CCharString, b: *const CCharString, c: bool, d: bool) {
        unsafe { ((*self.vmt).add_quest_card)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn remove_quest_card_from_guild(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).remove_quest_card_from_guild)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_quest_card_from_hero(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).remove_quest_card_from_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn give_hero_quest_card_directly(&mut self, a: *const CCharString, b: *const CCharString, c: bool) {
        unsafe { ((*self.vmt).give_hero_quest_card_directly)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn set_quest_card_objective(&mut self, a: *const CCharString, b: *const CCharString, c: *const CCharString, d: *const CCharString) {
        unsafe { ((*self.vmt).set_quest_card_objective)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn set_quest_card_gold_reward(&mut self, a: *const CCharString, b: c_long) {
        unsafe { ((*self.vmt).set_quest_card_gold_reward)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_quest_card_renown_reward(&mut self, a: *const CCharString, b: c_long) {
        unsafe { ((*self.vmt).set_quest_card_renown_reward)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn remove_all_available_quest_cards_from_guild(&mut self) {
        unsafe { ((*self.vmt).remove_all_available_quest_cards_from_guild)(self as *mut CGameScriptInterface) }
    }
    pub fn fail_all_active_quests(&mut self) {
        unsafe { ((*self.vmt).fail_all_active_quests)(self as *mut CGameScriptInterface) }
    }
    pub fn get_all_active_quest_info(&mut self, a: *mut cxx::StdVector<CCharString>, b: *mut cxx::StdVector<CCharString>) {
        unsafe { ((*self.vmt).get_all_active_quest_info)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_feat_card(&mut self, a: c_long, b: *const CCharString, c: *const CCharString) {
        unsafe { ((*self.vmt).add_feat_card)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_boast(&mut self, a: *const CCharString, b: c_long, c: c_long, d: c_long, e: bool, f: *const CCharString, g: c_long) {
        unsafe { ((*self.vmt).add_boast)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g) }
    }
    pub fn remove_boast(&mut self, a: c_long, b: *const CCharString) {
        unsafe { ((*self.vmt).remove_boast)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_boast_as_failed(&mut self, a: c_long, b: *const CCharString) {
        unsafe { ((*self.vmt).set_boast_as_failed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_boast_as_completed(&mut self, a: c_long, b: *const CCharString) {
        unsafe { ((*self.vmt).set_boast_as_completed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_boast_taken(&mut self, a: c_long, b: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_boast_taken)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_log_book_entry(&mut self, a: *const CWideString, b: *const CWideString, c: *const CWideString, d: ECategory) {
        unsafe { ((*self.vmt).add_log_book_entry)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn kick_off_quest_start_screen(&mut self, a: *const CCharString, b: bool, c: bool) {
        unsafe { ((*self.vmt).kick_off_quest_start_screen)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn kick_off_death_screen(&mut self) {
        unsafe { ((*self.vmt).kick_off_death_screen)(self as *mut CGameScriptInterface) }
    }
    pub fn kick_off_credits_screen(&mut self, a: *mut CCharString) {
        unsafe { ((*self.vmt).kick_off_credits_screen)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_preferred_quick_access_item(&mut self, a: *const CCharString, b: c_long, c: c_long) {
        unsafe { ((*self.vmt).set_preferred_quick_access_item)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn get_death_recovery_marker_name(&mut self) -> CCharString {
        unsafe { ((*self.vmt).get_death_recovery_marker_name)(self as *mut CGameScriptInterface) }
    }
    pub fn set_death_recovery_marker_name(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).set_death_recovery_marker_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_death_recovery_marker_name_to_default(&mut self) {
        unsafe { ((*self.vmt).reset_death_recovery_marker_name_to_default)(self as *mut CGameScriptInterface) }
    }
    pub fn is_to_fail_quest_on_death(&mut self) -> bool {
        unsafe { ((*self.vmt).is_to_fail_quest_on_death)(self as *mut CGameScriptInterface) }
    }
    pub fn set_whether_to_fail_quest_on_death(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_whether_to_fail_quest_on_death)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_whether_to_fail_quest_on_death_to_default(&mut self) {
        unsafe { ((*self.vmt).reset_whether_to_fail_quest_on_death_to_default)(self as *mut CGameScriptInterface) }
    }
    pub fn get_most_recent_valid_used_target(&mut self) -> CScriptThing {
        unsafe { ((*self.vmt).get_most_recent_valid_used_target)(self as *mut CGameScriptInterface) }
    }
    pub fn get_most_recent_valid_used_target_name(&mut self) -> CCharString {
        unsafe { ((*self.vmt).get_most_recent_valid_used_target_name)(self as *mut CGameScriptInterface) }
    }
    pub fn display_quest_info(&mut self, a: bool) {
        unsafe { ((*self.vmt).display_quest_info)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_quest_info_name(&mut self, a: *const c_char) {
        unsafe { ((*self.vmt).set_quest_info_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_quest_info_text(&mut self, a: *const c_char) {
        unsafe { ((*self.vmt).set_quest_info_text)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_quest_info_bar(&mut self, a: c_float, b: c_float, c: *const CRGBColour, d: *const CRGBColour, e: *const CCharString, f: *const CCharString, g: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_bar)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g) }
    }
    pub fn add_quest_info_bar_health(&mut self, a: *const CScriptThing, b: *const CRGBColour, c: *const CCharString, d: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_bar_health)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn add_quest_info_timer(&mut self, a: *const CTimer, b: *const CCharString, c: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_timer)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_quest_info_counter(&mut self, a: *const CCharString, b: c_long, c: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_counter)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_quest_info_counter_2(&mut self, a: *const CCounter, b: *const CCharString, c: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_counter_2)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_quest_info_counter_list(&mut self, a: *const CCharString, b: c_long, c: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_counter_list)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_quest_info_tick(&mut self, a: EGameAction, b: bool, c: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_tick)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_quest_info_tick_2(&mut self, a: *const CCharString, b: bool, c: c_float) -> c_long {
        unsafe { ((*self.vmt).add_quest_info_tick_2)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn update_quest_info_bar(&mut self, a: c_long, b: c_float, c: c_float, d: c_float) {
        unsafe { ((*self.vmt).update_quest_info_bar)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn change_quest_info_bar_colour(&mut self, a: c_long, b: *const CRGBColour, c: *const CRGBColour) {
        unsafe { ((*self.vmt).change_quest_info_bar_colour)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn update_quest_info_timer(&mut self, a: c_long, b: c_float) {
        unsafe { ((*self.vmt).update_quest_info_timer)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn update_quest_info_counter(&mut self, a: c_long, b: c_long, c: c_long) {
        unsafe { ((*self.vmt).update_quest_info_counter)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn update_quest_info_counter_list(&mut self, a: c_long, b: c_long, c: c_long) {
        unsafe { ((*self.vmt).update_quest_info_counter_list)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn update_quest_info_tick(&mut self, a: c_long, b: bool) {
        unsafe { ((*self.vmt).update_quest_info_tick)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn remove_quest_info_element(&mut self, a: c_long) {
        unsafe { ((*self.vmt).remove_quest_info_element)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_all_quest_info_elements(&mut self) {
        unsafe { ((*self.vmt).remove_all_quest_info_elements)(self as *mut CGameScriptInterface) }
    }
    pub fn display_time(&mut self, a: bool) {
        unsafe { ((*self.vmt).display_time)(self as *mut CGameScriptInterface, a) }
    }
    pub fn display_money_bag(&mut self, a: bool) {
        unsafe { ((*self.vmt).display_money_bag)(self as *mut CGameScriptInterface, a) }
    }
    pub fn display_mini_game_info(&mut self, a: bool, b: EMiniGameType) {
        unsafe { ((*self.vmt).display_mini_game_info)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn update_mini_game_info_bar(&mut self, a: c_float) {
        unsafe { ((*self.vmt).update_mini_game_info_bar)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_pick_pocketable(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_pick_pocketable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_pick_lockable(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_pick_lockable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_entity_stealable(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_stealable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_pick_pocketed(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_as_pick_pocketed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_pick_locked(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_as_pick_locked)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_stolen(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_as_stolen)(self as *mut CGameScriptInterface, a) }
    }
    pub fn mini_map_add_marker(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).mini_map_add_marker)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn mini_map_set_marker_graphic(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).mini_map_set_marker_graphic)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn mini_map_remove_marker(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).mini_map_remove_marker)(self as *mut CGameScriptInterface, a) }
    }
    pub fn mini_map_remove_all_markers(&mut self) {
        unsafe { ((*self.vmt).mini_map_remove_all_markers)(self as *mut CGameScriptInterface) }
    }
    pub fn mini_map_allow_route_between_regions(&mut self, a: *const CCharString, b: *const CCharString, c: bool) {
        unsafe { ((*self.vmt).mini_map_allow_route_between_regions)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn mini_map_set_as_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).mini_map_set_as_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_hidden_on_mini_map(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_hidden_on_mini_map)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_hud_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hud_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn text_entry_exists(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).text_entry_exists)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_valid_text_entry_name_with_attitude(&mut self, a: *const CScriptThing, b: *const CCharString) -> CCharString {
        unsafe { ((*self.vmt).get_valid_text_entry_name_with_attitude)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_thing_has_information(&mut self, a: *const CScriptThing, b: bool, c: bool, d: bool) {
        unsafe { ((*self.vmt).set_thing_has_information)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn clear_thing_has_information(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).clear_thing_has_information)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_will_be_using_narrator(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_will_be_using_narrator)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_reset_as_pure_ai_narrator(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_reset_as_pure_ai_narrator)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_new_conversation(&mut self, a: *const CScriptThing, b: bool, c: bool) -> c_long {
        unsafe { ((*self.vmt).add_new_conversation)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn add_person_to_conversation(&mut self, a: c_long, b: *const CScriptThing) {
        unsafe { ((*self.vmt).add_person_to_conversation)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_line_to_conversation(&mut self, a: c_long, b: *const CCharString, c: bool, d: *const CScriptThing, e: *const CScriptThing) {
        unsafe { ((*self.vmt).add_line_to_conversation)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn remove_conversation(&mut self, a: c_long, b: bool) {
        unsafe { ((*self.vmt).remove_conversation)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_conversation_active(&mut self, a: c_long) -> bool {
        unsafe { ((*self.vmt).is_conversation_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn play_avi_movie(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).play_avi_movie)(self as *mut CGameScriptInterface, a) }
    }
    pub fn start_movie_sequence(&mut self, a: *const CCharString, b: *mut CScriptGameResourceObjectMovieBase) {
        unsafe { ((*self.vmt).start_movie_sequence)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn fix_movie_sequence_camera(&mut self, a: bool) {
        unsafe { ((*self.vmt).fix_movie_sequence_camera)(self as *mut CGameScriptInterface, a) }
    }
    pub fn fade_screen_out_until_next_call_to_fade_screen_in(&mut self, a: c_float, b: c_float) {
        unsafe { ((*self.vmt).fade_screen_out_until_next_call_to_fade_screen_in)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn fade_screen_out(&mut self, a: c_float, b: c_float, c: CRGBColour) -> bool {
        unsafe { ((*self.vmt).fade_screen_out)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn fade_screen_in(&mut self) {
        unsafe { ((*self.vmt).fade_screen_in)(self as *mut CGameScriptInterface) }
    }
    pub fn is_screen_fading_out(&mut self) -> bool {
        unsafe { ((*self.vmt).is_screen_fading_out)(self as *mut CGameScriptInterface) }
    }
    pub fn pause(&mut self, a: c_float) {
        unsafe { ((*self.vmt).pause)(self as *mut CGameScriptInterface, a) }
    }
    pub fn end_cut_fade(&mut self) {
        unsafe { ((*self.vmt).end_cut_fade)(self as *mut CGameScriptInterface) }
    }
    pub fn pause_all_non_scripted_entities(&mut self, a: bool) {
        unsafe { ((*self.vmt).pause_all_non_scripted_entities)(self as *mut CGameScriptInterface, a) }
    }
    pub fn pause_all_entities(&mut self, a: bool) {
        unsafe { ((*self.vmt).pause_all_entities)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_allow_screen_fading_on_next_region_change(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_allow_screen_fading_on_next_region_change)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_allow_screen_fading_if_already_faded(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_allow_screen_fading_if_already_faded)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_to_keep_hero_abilities_during_cutscenes(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_to_keep_hero_abilities_during_cutscenes)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_to_display_tutorials_during_cutscenes(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_to_display_tutorials_during_cutscenes)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_cutscene_mode(&mut self, a: bool, b: bool) {
        unsafe { ((*self.vmt).set_cutscene_mode)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_in_cutscene(&mut self) -> bool {
        unsafe { ((*self.vmt).is_in_cutscene)(self as *mut CGameScriptInterface) }
    }
    pub fn set_cutscene_skippable(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_cutscene_skippable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_cutscene_skippable_while_paused(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_cutscene_skippable_while_paused)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_cutscene_action_mode(&mut self, a: bool, b: *const CCharString) {
        unsafe { ((*self.vmt).set_cutscene_action_mode)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn preload_new_scene(&mut self, a: c_float) {
        unsafe { ((*self.vmt).preload_new_scene)(self as *mut CGameScriptInterface, a) }
    }
    pub fn start_progress_display(&mut self) {
        unsafe { ((*self.vmt).start_progress_display)(self as *mut CGameScriptInterface) }
    }
    pub fn stop_progress_display(&mut self) {
        unsafe { ((*self.vmt).stop_progress_display)(self as *mut CGameScriptInterface) }
    }
    pub fn set_screen_messages_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_screen_messages_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_hero_controlled_by_player(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_controlled_by_player)(self as *mut CGameScriptInterface) }
    }
    pub fn is_in_movie_sequence(&mut self) -> bool {
        unsafe { ((*self.vmt).is_in_movie_sequence)(self as *mut CGameScriptInterface) }
    }
    pub fn cancel_abilities_for_cutscene(&mut self) {
        unsafe { ((*self.vmt).cancel_abilities_for_cutscene)(self as *mut CGameScriptInterface) }
    }
    pub fn resume_abilities_for_cutscene(&mut self) {
        unsafe { ((*self.vmt).resume_abilities_for_cutscene)(self as *mut CGameScriptInterface) }
    }
    pub fn cancel_using_ability(&mut self, a: EHeroAbility) {
        unsafe { ((*self.vmt).cancel_using_ability)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_ability_availability(&mut self, a: EHeroAbility, b: bool) {
        unsafe { ((*self.vmt).set_ability_availability)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_environmental_effects_always_update(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_environmental_effects_always_update)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_dead_creature(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).remove_dead_creature)(self as *mut CGameScriptInterface, a) }
    }
    pub fn camera_set_camera_preload_flag(&mut self, a: bool) {
        unsafe { ((*self.vmt).camera_set_camera_preload_flag)(self as *mut CGameScriptInterface, a) }
    }
    pub fn camera_circle_around_thing(&mut self, a: *const CScriptThing, b: *const C3DVector, c: c_float) {
        unsafe { ((*self.vmt).camera_circle_around_thing)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn camera_circle_around_pos(&mut self, a: *const C3DVector, b: *const C3DVector, c: c_float) {
        unsafe { ((*self.vmt).camera_circle_around_pos)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn camera_move_to_pos_and_look_at_pos(&mut self, a: *const C3DVector, b: *const C3DVector, c: c_float) {
        unsafe { ((*self.vmt).camera_move_to_pos_and_look_at_pos)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn camera_move_to_pos_and_look_at_thing(&mut self, a: *const C3DVector, b: *const CScriptThing, c: c_float) {
        unsafe { ((*self.vmt).camera_move_to_pos_and_look_at_thing)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn camera_move_between_looking_at(&mut self, a: *const C3DVector, b: *const C3DVector, c: *const CScriptThing, d: c_float, e: c_float) {
        unsafe { ((*self.vmt).camera_move_between_looking_at)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn camera_move_between_looking_at_2(&mut self, a: *const C3DVector, b: *const C3DVector, c: *const C3DVector, d: c_float, e: c_float) {
        unsafe { ((*self.vmt).camera_move_between_looking_at_2)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn camera_move_between_look_from_and_look_to(&mut self, a: *const C3DVector, b: *const C3DVector, c: *const C3DVector, d: *const C3DVector, e: c_float) {
        unsafe { ((*self.vmt).camera_move_between_look_from_and_look_to)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn camera_use_camera_point(&mut self, a: *const CScriptThing, b: *const C3DVector, c: *const CRightHandedSet, d: c_float, e: c_long, f: c_long) {
        unsafe { ((*self.vmt).camera_use_camera_point)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
    }
    pub fn camera_use_camera_point_2(&mut self, a: *const CCharString, b: *const C3DVector, c: *const CRightHandedSet, d: c_float, e: c_long, f: c_long) {
        unsafe { ((*self.vmt).camera_use_camera_point_2)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
    }
    pub fn camera_use_camera_point_3(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: c_float, d: c_long, e: c_long) {
        unsafe { ((*self.vmt).camera_use_camera_point_3)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn camera_use_camera_point_4(&mut self, a: *const CCharString, b: *const CScriptThing, c: c_float, d: c_long, e: c_long) {
        unsafe { ((*self.vmt).camera_use_camera_point_4)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn camera_do_conversation(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: ECameraOp, d: bool) {
        unsafe { ((*self.vmt).camera_do_conversation)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn camera_default(&mut self) {
        unsafe { ((*self.vmt).camera_default)(self as *mut CGameScriptInterface) }
    }
    pub fn camera_reset_to_view_behind_hero(&mut self, a: c_float) {
        unsafe { ((*self.vmt).camera_reset_to_view_behind_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_camera_in_scripted_mode(&mut self) -> bool {
        unsafe { ((*self.vmt).is_camera_in_scripted_mode)(self as *mut CGameScriptInterface) }
    }
    pub fn camera_use_screen_effect(&mut self, a: c_float, b: c_float, c: c_float) {
        unsafe { ((*self.vmt).camera_use_screen_effect)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn camera_cancel_screen_effect(&mut self) {
        unsafe { ((*self.vmt).camera_cancel_screen_effect)(self as *mut CGameScriptInterface) }
    }
    pub fn is_camera_pos_on_screen(&mut self, a: *const C3DVector) -> bool {
        unsafe { ((*self.vmt).is_camera_pos_on_screen)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_game_angle_xy(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_game_angle_xy)(self as *mut CGameScriptInterface) }
    }
    pub fn camera_earthquake_intensity_at_pos(&mut self, a: *const C3DVector, b: c_float, c: c_float) {
        unsafe { ((*self.vmt).camera_earthquake_intensity_at_pos)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn camera_shake(&mut self, a: c_float, b: c_float) {
        unsafe { ((*self.vmt).camera_shake)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn open_door(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).open_door)(self as *mut CGameScriptInterface, a) }
    }
    pub fn close_door(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).close_door)(self as *mut CGameScriptInterface, a) }
    }
    pub fn open_house_doors(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).open_house_doors)(self as *mut CGameScriptInterface, a) }
    }
    pub fn close_house_doors(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).close_house_doors)(self as *mut CGameScriptInterface, a) }
    }
    pub fn jam_door(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).jam_door)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_door_trigger_type(&mut self, a: *const CScriptThing, b: EDoorTriggerType) {
        unsafe { ((*self.vmt).set_door_trigger_type)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn override_automatic_house_locking(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).override_automatic_house_locking)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_house_owned_by_player(&mut self, a: *const CScriptThing, b: bool, c: bool) {
        unsafe { ((*self.vmt).set_house_owned_by_player)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn set_buyable_house_as_scripted(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_buyable_house_as_scripted)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_chest_open(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_chest_open)(self as *mut CGameScriptInterface, a) }
    }
    pub fn open_chest(&mut self, a: *const CScriptThing, b: bool) -> bool {
        unsafe { ((*self.vmt).open_chest)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn close_chest(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).close_chest)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_number_of_keys_needed_to_unlock_chest(&mut self, a: *const CScriptThing, b: *mut CCharString) -> c_long {
        unsafe { ((*self.vmt).get_number_of_keys_needed_to_unlock_chest)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn display_locked_chest_message(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).display_locked_chest_message)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_trophy_as_mountable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_trophy_as_mountable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_village_limbo(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_village_limbo)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_not_reload(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).set_creature_not_reload)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_sleeping_time(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_sleeping_time)(self as *mut CGameScriptInterface, a) }
    }
    pub fn enable_guards(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).enable_guards)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn enable_villager_def_types(&mut self, a: *const CScriptThing, b: bool, c: *const CCharString) {
        unsafe { ((*self.vmt).enable_villager_def_types)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn try_to_respawn_def_named(&mut self, a: *const CScriptThing, b: *const CCharString, c: *const C3DVector) -> CScriptThing {
        unsafe { ((*self.vmt).try_to_respawn_def_named)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn clear_hero_enemy_of_guards(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).clear_hero_enemy_of_guards)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_thing_as_usable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_thing_as_usable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_thing_home_building(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).set_thing_home_building)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_village_attitude(&mut self, a: *const CScriptThing, b: EScriptVillageAttitude) {
        unsafe { ((*self.vmt).set_village_attitude)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_crime_committed(&mut self, a: *const CScriptThing, b: ECrime, c: bool, d: *const CScriptThing, e: *const CScriptThing, f: EOpinionPostDeedType) {
        unsafe { ((*self.vmt).add_crime_committed)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
    }
    pub fn give_thing_best_enemy_target(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).give_thing_best_enemy_target)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn clear_thing_best_enemy_target(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).clear_thing_best_enemy_target)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_in_limbo(&mut self, a: *const CScriptThing, b: bool, c: bool) {
        unsafe { ((*self.vmt).entity_set_in_limbo)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn is_entity_in_limbo(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_entity_in_limbo)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_get_shot_strike_pos(&mut self, a: *const CScriptThing, b: *mut C3DVector) -> bool {
        unsafe { ((*self.vmt).entity_get_shot_strike_pos)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_negate_all_hits(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_negate_all_hits)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_evade_all_hits(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_evade_all_hits)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_able_to_be_engaged_in_combat(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_able_to_be_engaged_in_combat)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_always_block_attacks_from_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: bool) {
        unsafe { ((*self.vmt).entity_set_always_block_attacks_from_thing)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_attack_thing_immediately(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: bool, d: bool) {
        unsafe { ((*self.vmt).entity_set_attack_thing_immediately)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn entity_set_combat_type(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_combat_type)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_reset_combat_type_to_default(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_reset_combat_type_to_default)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_max_number_of_attackers(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_max_number_of_attackers)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_clear_max_number_of_attackers(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_clear_max_number_of_attackers)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_attach_to_script(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_attach_to_script)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_combat_ability(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_combat_ability)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_ranged_target(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_ranged_target)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_clear_ranged_target(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_clear_ranged_target)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_targetable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_targetable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_targeting_type(&mut self, a: *const CScriptThing, b: ETargetingType) {
        unsafe { ((*self.vmt).entity_set_targeting_type)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_targeting_valid_target_without_los(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_targeting_valid_target_without_los)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_teleport_to_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: bool) {
        unsafe { ((*self.vmt).entity_teleport_to_thing)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_teleport_to_position(&mut self, a: *const CScriptThing, b: *const C3DVector, c: c_float, d: bool, e: bool) {
        unsafe { ((*self.vmt).entity_teleport_to_position)(self as *mut CGameScriptInterface, a, b, c, d, e) }
    }
    pub fn entity_set_facing_angle(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
        unsafe { ((*self.vmt).entity_set_facing_angle)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_facing_angle_towards_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: bool) {
        unsafe { ((*self.vmt).entity_set_facing_angle_towards_thing)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_perception_variables(&mut self, a: *const CScriptThing, b: c_float, c: c_float, d: c_float) {
        unsafe { ((*self.vmt).entity_set_perception_variables)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn set_thing_persistent(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_thing_persistent)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_thing_as_wanting_money(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_thing_as_wanting_money)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_appearance_morph_seed(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_appearance_morph_seed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_entity_as_region_following(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: bool) {
        unsafe { ((*self.vmt).set_entity_as_region_following)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn set_entity_as_following_hero_through_teleporters(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_entity_as_following_hero_through_teleporters)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_appearance_seed(&mut self, a: *const CScriptThing, b: c_ulong) {
        unsafe { ((*self.vmt).entity_set_appearance_seed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_appearance_seed(&mut self, a: *const CScriptThing, b: *mut c_ulong) {
        unsafe { ((*self.vmt).entity_get_appearance_seed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_for_sale(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_for_sale)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_stock_item_price(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_stock_item_price)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_stock_item_price(&mut self, a: *const CScriptThing) -> c_long {
        unsafe { ((*self.vmt).entity_get_stock_item_price)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_play_object_animation(&mut self, a: *const CScriptThing, b: *const CCharString, c: bool) {
        unsafe { ((*self.vmt).entity_play_object_animation)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_max_running_speed(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_max_running_speed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_max_walking_speed(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_max_walking_speed)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_reset_max_running_speed(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_reset_max_running_speed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_reset_max_walking_speed(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_reset_max_walking_speed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_attach_to_village(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_attach_to_village)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_sitting_on_floor(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_sitting_on_floor)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_scared(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_scared)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_drunk(&mut self, a: *mut CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_drunk)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_having_bound_hands(&mut self, a: *mut CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_having_bound_hands)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_remove_all_movement_blocking_modes(&mut self, a: *mut CScriptThing) {
        unsafe { ((*self.vmt).entity_set_as_remove_all_movement_blocking_modes)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_force_to_look_at_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_force_to_look_at_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_force_to_look_at_camera(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_force_to_look_at_camera)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_force_to_look_at_nothing(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_force_to_look_at_nothing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_reset_force_to_look_at(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_reset_force_to_look_at)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_shot_accuracy_percentage(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_shot_accuracy_percentage)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_standing_on_thing(&mut self, a: *const CScriptThing) -> CScriptThing {
        unsafe { ((*self.vmt).entity_get_standing_on_thing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_get_standing_inside_building(&mut self, a: *const CScriptThing) -> CScriptThing {
        unsafe { ((*self.vmt).entity_get_standing_inside_building)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_drop_generic_box(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_drop_generic_box)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_sheathe_weapons(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_sheathe_weapons)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_unsheathe_weapons(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_unsheathe_weapons)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_unsheathe_melee_weapon(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_unsheathe_melee_weapon)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_unsheathe_ranged_weapon(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_unsheathe_ranged_weapon)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_alpha(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
        unsafe { ((*self.vmt).entity_set_alpha)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_as_drawable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_drawable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_cutscene_behaviour(&mut self, a: *const CScriptThing, b: ECutsceneBehaviour) {
        unsafe { ((*self.vmt).entity_set_cutscene_behaviour)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_sex(&mut self, a: *const CScriptThing) -> ESex {
        unsafe { ((*self.vmt).entity_get_sex)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_able_to_walk_through_solid_objects(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_able_to_walk_through_solid_objects)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_respond_to_hit(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_respond_to_hit)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_damageable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_damageable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_killable(&mut self, a: *const CScriptThing, b: bool, c: bool) {
        unsafe { ((*self.vmt).entity_set_as_killable)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_as_locked(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_locked)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_decapitate(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_decapitate)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_give_gold(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_give_gold)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_allow_boss_phase_changes(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_allow_boss_phase_changes)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_boss_phase(&mut self, a: *const CScriptThing) -> c_long {
        unsafe { ((*self.vmt).entity_get_boss_phase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_boss_phase(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_boss_phase)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_reset_creature_mode(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_reset_creature_mode)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_receiving_events(&mut self, a: *const CScriptThing, b: bool) -> bool {
        unsafe { ((*self.vmt).entity_set_as_receiving_events)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_to_add_to_combo_multiplier_when_hit(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_to_add_to_combo_multiplier_when_hit)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_to_add_to_stat_changes_when_hit(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_to_add_to_stat_changes_when_hit)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_leave_combat_stance(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_leave_combat_stance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_use_movement_in_actions(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_use_movement_in_actions)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_displaying_emote_icon(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_displaying_emote_icon)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_collidable_to_things(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_as_collidable_to_things)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_enable_gravity(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_enable_gravity)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_light_as_on(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_light_as_on)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_fade_out(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_fade_out)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_fade_in(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_fade_in)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_begin_loading_animation(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_begin_loading_animation)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_begin_loading_basic_animations(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_begin_loading_basic_animations)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_cast_force_push(&mut self, a: *const CScriptThing, b: bool) -> bool {
        unsafe { ((*self.vmt).entity_cast_force_push)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_cast_lightning_at_target(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_cast_lightning_at_target)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn begin_loading_mesh(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).begin_loading_mesh)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_will_teleport_to_area(&mut self, a: *const CScriptThing, b: C3DVector, c: c_float, d: c_float) -> bool {
        unsafe { ((*self.vmt).entity_will_teleport_to_area)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn entity_start_screamer_super_attack_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_start_screamer_super_attack_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_end_screamer_super_attack_thing(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_end_screamer_super_attack_thing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_guide_to_show_quest_cards_when_spoken_to(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_guide_to_show_quest_cards_when_spoken_to)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_light_colour(&mut self, a: *const CScriptThing, b: *const CRGBColour) {
        unsafe { ((*self.vmt).set_light_colour)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn creature_generator_set_family(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).creature_generator_set_family)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn creature_generator_trigger(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).creature_generator_trigger)(self as *mut CGameScriptInterface, a) }
    }
    pub fn creature_generator_set_always_create_creatures_on_trigger(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).creature_generator_set_always_create_creatures_on_trigger)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn creature_generator_is_depleted(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).creature_generator_is_depleted)(self as *mut CGameScriptInterface, a) }
    }
    pub fn creature_generator_is_destroyed(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).creature_generator_is_destroyed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn creature_generator_set_generated_creature_script_name(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).creature_generator_set_generated_creature_script_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn creature_generator_set_num_triggers(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).creature_generator_set_num_triggers)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn creature_generator_get_num_generated_creatures(&mut self, a: *const CScriptThing) -> c_long {
        unsafe { ((*self.vmt).creature_generator_get_num_generated_creatures)(self as *mut CGameScriptInterface, a) }
    }
    pub fn creature_generator_are_all_creatures_alive(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).creature_generator_are_all_creatures_alive)(self as *mut CGameScriptInterface, a) }
    }
    pub fn creature_generator_add_triggerer(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).creature_generator_add_triggerer)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn creature_generator_remove_triggerer(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).creature_generator_remove_triggerer)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_generator_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_creature_generator_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_generators_enabled(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).set_creature_generators_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_generators_enabled_during_script(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).set_creature_generators_enabled_during_script)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_generators_creature_group_as_enabled(&mut self, a: ECreatureGroup, b: bool) {
        unsafe { ((*self.vmt).set_creature_generators_creature_group_as_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_creature_generation_enabled_for_region(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_creature_generation_enabled_for_region)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_creature_flying(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_creature_flying)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_teleporter_as_active(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_teleporter_as_active)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_teleporter_active(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_teleporter_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_teleporting_as_active(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_teleporting_as_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_teleporting_active(&mut self) -> bool {
        unsafe { ((*self.vmt).is_teleporting_active)(self as *mut CGameScriptInterface) }
    }
    pub fn set_region_exit_as_active(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_region_exit_as_active)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_region_entrance_as_active(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_region_entrance_as_active)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_region_text_display_as_active(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_region_text_display_as_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_hero_sleeping_as_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_hero_sleeping_as_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_hero_sleeping_enabled(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_sleeping_enabled)(self as *mut CGameScriptInterface) }
    }
    pub fn set_experience_spending_as_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_experience_spending_as_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_morality_changing_as_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_morality_changing_as_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_summoner_death_explosion_affects_hero(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_summoner_death_explosion_affects_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_nearest_enabled_digging_spot(&mut self, a: *const CScriptThing) -> CScriptThing {
        unsafe { ((*self.vmt).get_nearest_enabled_digging_spot)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_digging_spot_enabled(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_digging_spot_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_digging_spot_hidden(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_digging_spot_hidden)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_digging_spot_as_hidden(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_digging_spot_as_hidden)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn check_for_camera_message(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).check_for_camera_message)(self as *mut CGameScriptInterface, a) }
    }
    pub fn wait_for_camera_message(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).wait_for_camera_message)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_thing_as_conscious(&mut self, a: *const CScriptThing, b: bool, c: *const CCharString) {
        unsafe { ((*self.vmt).set_thing_as_conscious)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn set_fire_to_thing(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).set_fire_to_thing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn extinguish_fires_on_thing(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).extinguish_fires_on_thing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_thing_on_fire(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_on_fire)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_item_to_container(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).add_item_to_container)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn remove_item_from_container(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).remove_item_from_container)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_death_container_as_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_death_container_as_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_item_def_names_from_container(&mut self, a: *const CScriptThing, b: *mut cxx::StdVector<CCharString>) {
        unsafe { ((*self.vmt).get_item_def_names_from_container)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_creature_brain(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).set_creature_brain)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_stategroup_enabled(&mut self, a: *const CScriptThing, b: *const CCharString, c: bool) {
        unsafe { ((*self.vmt).entity_set_stategroup_enabled)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_all_stategroups_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_all_stategroups_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_combat_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_combat_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_sleep_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_sleep_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_opinion_reactions_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_opinion_reactions_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_deed_reactions_enabled(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_set_deed_reactions_enabled)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn debug_get_all_text_entries_for_targeted_thing(&mut self, a: *mut cxx::StdSet<c_ulong>) {
        unsafe { ((*self.vmt).debug_get_all_text_entries_for_targeted_thing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_thing_as_enemy_of_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_thing_as_enemy_of_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_unset_thing_as_enemy_of_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_unset_thing_as_enemy_of_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_thing_as_ally_of_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_thing_as_ally_of_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_unset_thing_as_ally_of_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_unset_thing_as_ally_of_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_in_faction(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_in_faction)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_faction_as_allied_to_faction(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).set_faction_as_allied_to_faction)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_faction_as_neutral_to_faction(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).set_faction_as_neutral_to_faction)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_faction_as_enemy_to_faction(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).set_faction_as_enemy_to_faction)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn are_entities_enemies(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).are_entities_enemies)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_next_in_opinion_attitude_graph(&mut self, a: EOpinionAttitudeType) -> EOpinionAttitudeType {
        unsafe { ((*self.vmt).get_next_in_opinion_attitude_graph)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_opinion_attitude_as_string(&mut self, a: EOpinionAttitudeType, b: *mut CCharString) {
        unsafe { ((*self.vmt).get_opinion_attitude_as_string)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_opinion_attitude_to_player(&mut self, a: *const CScriptThing) -> EOpinionAttitudeType {
        unsafe { ((*self.vmt).entity_get_opinion_attitude_to_player)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_get_opinion_attitude_to_player_as_string(&mut self, a: *const CScriptThing, b: *mut CCharString) {
        unsafe { ((*self.vmt).entity_get_opinion_attitude_to_player_as_string)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_opinion_of_player(&mut self, a: *const CScriptThing, b: EOpinion) -> c_float {
        unsafe { ((*self.vmt).entity_get_opinion_of_player)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_opinion_reaction_mask(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_opinion_reaction_mask)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_opinion_reaction_mask_2(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_opinion_reaction_mask_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_opinion_deed_mask(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_opinion_deed_mask)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_opinion_deed_mask_2(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_opinion_deed_mask_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_opinion_deed_type_enabled(&mut self, a: *const CScriptThing, b: EOpinionDeedType, c: bool) {
        unsafe { ((*self.vmt).entity_set_opinion_deed_type_enabled)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_opinion_attitude_enabled(&mut self, a: *const CScriptThing, b: EOpinionAttitudeType, c: bool) {
        unsafe { ((*self.vmt).entity_set_opinion_attitude_enabled)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_opinion_reaction_enabled(&mut self, a: *const CScriptThing, b: EOpinionReactionType, c: bool) {
        unsafe { ((*self.vmt).entity_set_opinion_reaction_enabled)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_set_personality_override(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_personality_override)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_personality_override_2(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_personality_override_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_clear_personality_override(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_clear_personality_override)(self as *mut CGameScriptInterface, a) }
    }
    pub fn entity_set_as_opinion_source(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).entity_set_as_opinion_source)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_opinion_source_2(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).entity_set_as_opinion_source_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_unset_as_opinion_source(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).entity_unset_as_opinion_source)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn opinion_source_set_as_exclusive(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).opinion_source_set_as_exclusive)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn opinion_source_set_as_attention_grabbing(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).opinion_source_set_as_attention_grabbing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_post_opinion_deed_to_all(&mut self, a: *const CScriptThing, b: EOpinionDeedType) {
        unsafe { ((*self.vmt).entity_post_opinion_deed_to_all)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_post_opinion_deed_to_recipient(&mut self, a: *const CScriptThing, b: EOpinionDeedType, c: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_post_opinion_deed_to_recipient)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_post_opinion_deed_to_recipient_village(&mut self, a: *const CScriptThing, b: EOpinionDeedType, c: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_post_opinion_deed_to_recipient_village)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn entity_post_opinion_deed_keep_searching_for_witnesses(&mut self, a: *const CScriptThing, b: EOpinionDeedType, c: *const CScriptThing) -> c_long {
        unsafe { ((*self.vmt).entity_post_opinion_deed_keep_searching_for_witnesses)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn remove_opinion_deed_still_searching_for_witnesses(&mut self, a: *const CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).remove_opinion_deed_still_searching_for_witnesses)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_deed_witnessed(&mut self, a: c_long) -> bool {
        unsafe { ((*self.vmt).is_deed_witnessed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn can_thing_be_seen_by_other_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).can_thing_be_seen_by_other_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn can_thing_be_nearly_seen_by_other_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).can_thing_be_nearly_seen_by_other_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn can_thing_be_smelled_by_other_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).can_thing_be_smelled_by_other_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn can_thing_be_heard_by_other_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).can_thing_be_heard_by_other_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_thing_aware_of_other_thing_in_any_way(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_aware_of_other_thing_in_any_way)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_as_aware_of_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
        unsafe { ((*self.vmt).entity_set_as_aware_of_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_sound_radius(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_sound_radius)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_smell_radius(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_smell_radius)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_sight_radius(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_sight_radius)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_extended_sight_radius(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_extended_sight_radius)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_set_give_up_chase_radius(&mut self, a: *const CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).entity_set_give_up_chase_radius)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn entity_get_hearing_radius(&mut self, a: *const CScriptThing) -> c_float {
        unsafe { ((*self.vmt).entity_get_hearing_radius)(self as *mut CGameScriptInterface, a) }
    }
    pub fn manually_trigger_trap(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).manually_trigger_trap)(self as *mut CGameScriptInterface, a) }
    }
    pub fn manually_reset_trap(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).manually_reset_trap)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_time_of_day(&mut self, a: c_float) {
        unsafe { ((*self.vmt).set_time_of_day)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_time_of_day(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_time_of_day)(self as *mut CGameScriptInterface) }
    }
    pub fn set_time_as_stopped(&mut self, a: bool, b: *mut c_long) {
        unsafe { ((*self.vmt).set_time_as_stopped)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn fast_forward_time_to(&mut self, a: c_float, b: c_float) {
        unsafe { ((*self.vmt).fast_forward_time_to)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_time_of_day_between(&mut self, a: c_long, b: c_long) -> bool {
        unsafe { ((*self.vmt).is_time_of_day_between)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_day_of_week(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_day_of_week)(self as *mut CGameScriptInterface) }
    }
    pub fn get_day_count(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_day_count)(self as *mut CGameScriptInterface) }
    }
    pub fn get_world_frame(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_world_frame)(self as *mut CGameScriptInterface) }
    }
    pub fn get_constant_fps(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_constant_fps)(self as *mut CGameScriptInterface) }
    }
    pub fn get_active_quest_name(&mut self) -> CCharString {
        unsafe { ((*self.vmt).get_active_quest_name)(self as *mut CGameScriptInterface) }
    }
    pub fn transition_to_theme(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).transition_to_theme)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn reset_to_default_theme(&mut self, a: c_float) {
        unsafe { ((*self.vmt).reset_to_default_theme)(self as *mut CGameScriptInterface, a) }
    }
    pub fn transition_to_theme_all_internals(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).transition_to_theme_all_internals)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn reset_to_default_theme_all_internals(&mut self, a: c_float) {
        unsafe { ((*self.vmt).reset_to_default_theme_all_internals)(self as *mut CGameScriptInterface, a) }
    }
    pub fn transition_to_theme_externals(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).transition_to_theme_externals)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn reset_to_default_theme_externals(&mut self, a: c_float) {
        unsafe { ((*self.vmt).reset_to_default_theme_externals)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_environment_theme_weight_all_channels(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).set_environment_theme_weight_all_channels)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_environment_theme_weight_all_internals(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).set_environment_theme_weight_all_internals)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_environment_theme_weight_externals(&mut self, a: *const CCharString, b: c_float) {
        unsafe { ((*self.vmt).set_environment_theme_weight_externals)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_sound_themes_as_enabled_for_region(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).set_sound_themes_as_enabled_for_region)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_all_sounds_as_muted(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_all_sounds_as_muted)(self as *mut CGameScriptInterface, a) }
    }
    pub fn radial_blur_fade_to(&mut self, a: c_float, b: c_float, c: c_float, d: c_float, e: c_float, f: c_float, g: c_float) -> *mut c_void {
        unsafe { ((*self.vmt).radial_blur_fade_to)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g) }
    }
    pub fn radial_blur_fade_to_2(&mut self, a: c_float, b: C3DVector, c: c_float, d: C3DVector, e: c_float, f: c_float, g: c_float) -> *mut c_void {
        unsafe { ((*self.vmt).radial_blur_fade_to_2)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g) }
    }
    pub fn radial_blur_fade_out(&mut self, a: c_float, b: *mut c_void) {
        unsafe { ((*self.vmt).radial_blur_fade_out)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_radial_blur_fade_active(&mut self) -> bool {
        unsafe { ((*self.vmt).is_radial_blur_fade_active)(self as *mut CGameScriptInterface) }
    }
    pub fn cancel_radial_blur_fade(&mut self) {
        unsafe { ((*self.vmt).cancel_radial_blur_fade)(self as *mut CGameScriptInterface) }
    }
    pub fn radial_blur_set_center_world_pos(&mut self, a: *mut c_void, b: *const C3DVector) {
        unsafe { ((*self.vmt).radial_blur_set_center_world_pos)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn displacement_monochrome_effect_colour_fade_to(&mut self, a: c_float, b: *const CRGBFloatColour) -> c_void {
        unsafe { ((*self.vmt).displacement_monochrome_effect_colour_fade_to)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn displacement_monochrome_effect_colour_fade_out(&mut self, a: c_float, b: *mut c_void) {
        unsafe { ((*self.vmt).displacement_monochrome_effect_colour_fade_out)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn screen_filter_fade_to(&mut self, a: c_float, b: c_float, c: c_float, d: c_float, e: c_float, f: *const CRGBFloatColour, g: *const cxx::StdVector<CScreenFilterSThingByPass>) -> c_void {
        unsafe { ((*self.vmt).screen_filter_fade_to)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g) }
    }
    pub fn screen_filter_fade_out(&mut self, a: c_float, b: *mut c_void) {
        unsafe { ((*self.vmt).screen_filter_fade_out)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_thing_and_carried_items_not_affected_by_screen_filter(&mut self, a: *mut CScriptThing, b: *mut c_void) {
        unsafe { ((*self.vmt).set_thing_and_carried_items_not_affected_by_screen_filter)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn un_set_thing_and_carried_items_not_affected_by_screen_filter(&mut self, a: *mut CScriptThing) {
        unsafe { ((*self.vmt).un_set_thing_and_carried_items_not_affected_by_screen_filter)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_gift_romantic(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_gift_romantic)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_gift_friendly(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_gift_friendly)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_gift_offensive(&mut self, a: *const CCharString) -> bool {
        unsafe { ((*self.vmt).is_gift_offensive)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_thing_a_bed(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_a_bed)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_thing_a_chest(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_a_chest)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_thing_a_door(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_a_door)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_thing_smashable(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_smashable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_thing_searchable(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_searchable)(self as *mut CGameScriptInterface, a) }
    }
    pub fn apply_script_brush(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).apply_script_brush)(self as *mut CGameScriptInterface, a) }
    }
    pub fn enable_decals(&mut self, a: bool) {
        unsafe { ((*self.vmt).enable_decals)(self as *mut CGameScriptInterface, a) }
    }
    pub fn play_criteria_sound_on_thing(&mut self, a: *const CScriptThing, b: *const CCharString) -> c_ulong {
        unsafe { ((*self.vmt).play_criteria_sound_on_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn play_sound_on_thing(&mut self, a: *const CScriptThing, b: *const CCharString) -> c_ulong {
        unsafe { ((*self.vmt).play_sound_on_thing)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_sound_playing(&mut self, a: c_ulong) -> bool {
        unsafe { ((*self.vmt).is_sound_playing)(self as *mut CGameScriptInterface, a) }
    }
    pub fn stop_sound(&mut self, a: c_ulong) {
        unsafe { ((*self.vmt).stop_sound)(self as *mut CGameScriptInterface, a) }
    }
    pub fn play_sound_at_pos(&mut self, a: *const C3DVector, b: *const CCharString) -> c_ulong {
        unsafe { ((*self.vmt).play_sound_at_pos)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn play_2d_sound(&mut self, a: *const CCharString) -> c_ulong {
        unsafe { ((*self.vmt).play_2d_sound)(self as *mut CGameScriptInterface, a) }
    }
    pub fn enable_sounds(&mut self, a: bool) {
        unsafe { ((*self.vmt).enable_sounds)(self as *mut CGameScriptInterface, a) }
    }
    pub fn override_music(&mut self, a: EMusicSetType, b: bool, c: bool) {
        unsafe { ((*self.vmt).override_music)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn stop_override_music(&mut self, a: bool) {
        unsafe { ((*self.vmt).stop_override_music)(self as *mut CGameScriptInterface, a) }
    }
    pub fn cache_music_set(&mut self, a: EMusicSetType) {
        unsafe { ((*self.vmt).cache_music_set)(self as *mut CGameScriptInterface, a) }
    }
    pub fn enable_danger_music(&mut self, a: bool) {
        unsafe { ((*self.vmt).enable_danger_music)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_danger_music_enabled(&mut self) -> bool {
        unsafe { ((*self.vmt).is_danger_music_enabled)(self as *mut CGameScriptInterface) }
    }
    pub fn start_countdown_timer(&mut self, a: c_float) {
        unsafe { ((*self.vmt).start_countdown_timer)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_countdown_timer(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_countdown_timer)(self as *mut CGameScriptInterface) }
    }
    pub fn auto_save_check_point(&mut self) {
        unsafe { ((*self.vmt).auto_save_check_point)(self as *mut CGameScriptInterface) }
    }
    pub fn auto_save_quest_start(&mut self) {
        unsafe { ((*self.vmt).auto_save_quest_start)(self as *mut CGameScriptInterface) }
    }
    pub fn auto_save(&mut self) {
        unsafe { ((*self.vmt).auto_save)(self as *mut CGameScriptInterface) }
    }
    pub fn set_saving_as_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_saving_as_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn is_saving_enabled(&mut self) -> bool {
        unsafe { ((*self.vmt).is_saving_enabled)(self as *mut CGameScriptInterface) }
    }
    pub fn set_save_game_marker_pos(&mut self, a: *const C3DVector) {
        unsafe { ((*self.vmt).set_save_game_marker_pos)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_to_front_end(&mut self) {
        unsafe { ((*self.vmt).reset_to_front_end)(self as *mut CGameScriptInterface) }
    }
    pub fn set_guild_seal_recall_location(&mut self, a: *const C3DVector, b: c_float) {
        unsafe { ((*self.vmt).set_guild_seal_recall_location)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_guild_seal_recall_pos(&mut self) -> C3DVector {
        unsafe { ((*self.vmt).get_guild_seal_recall_pos)(self as *mut CGameScriptInterface) }
    }
    pub fn get_guild_seal_recall_angle_xy(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_guild_seal_recall_angle_xy)(self as *mut CGameScriptInterface) }
    }
    pub fn set_readable_object_text(&mut self, a: *const CScriptThing, b: *const CWideString) {
        unsafe { ((*self.vmt).set_readable_object_text)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_readable_object_text_tag(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).set_readable_object_text_tag)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_formatted_string(&mut self, a: *const CCharString, b: *const cxx::StdVector<CWideString>) -> CWideString {
        unsafe { ((*self.vmt).get_formatted_string)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_text_string(&mut self, a: *const CCharString) -> CWideString {
        unsafe { ((*self.vmt).get_text_string)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_rumour_category(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).add_rumour_category)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_new_rumour_to_category(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).add_new_rumour_to_category)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn remove_rumour_category(&mut self, a: *const CCharString) {
        unsafe { ((*self.vmt).remove_rumour_category)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_category_activity(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).set_category_activity)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_gossip_village(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).add_gossip_village)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn add_gossip_faction_to_category(&mut self, a: *const CCharString, b: *const CCharString) {
        unsafe { ((*self.vmt).add_gossip_faction_to_category)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_is_gossip_for_player(&mut self, a: CCharString, b: bool) {
        unsafe { ((*self.vmt).set_is_gossip_for_player)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_is_gossip_for_player_2(&mut self, a: *const CCharString, b: bool) {
        unsafe { ((*self.vmt).set_is_gossip_for_player_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn update_online_score_archery(&mut self, a: c_float) {
        unsafe { ((*self.vmt).update_online_score_archery)(self as *mut CGameScriptInterface, a) }
    }
    pub fn update_online_score_chicken_kick(&mut self, a: c_float) {
        unsafe { ((*self.vmt).update_online_score_chicken_kick)(self as *mut CGameScriptInterface, a) }
    }
    pub fn update_online_score_chapel_or_temple(&mut self, a: c_float) {
        unsafe { ((*self.vmt).update_online_score_chapel_or_temple)(self as *mut CGameScriptInterface, a) }
    }
    pub fn update_online_score_fishing_compo(&mut self, a: c_float) {
        unsafe { ((*self.vmt).update_online_score_fishing_compo)(self as *mut CGameScriptInterface, a) }
    }
    pub fn update_score_fishing_competition(&mut self, a: c_float) {
        unsafe { ((*self.vmt).update_score_fishing_competition)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_best_time_pairs(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_best_time_pairs)(self as *mut CGameScriptInterface) }
    }
    pub fn get_best_time_sorting(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_best_time_sorting)(self as *mut CGameScriptInterface) }
    }
    pub fn get_best_score_blackjack(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_best_score_blackjack)(self as *mut CGameScriptInterface) }
    }
    pub fn get_best_score_coin_golf_oak_vale(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_best_score_coin_golf_oak_vale)(self as *mut CGameScriptInterface) }
    }
    pub fn get_best_score_coin_golf_snow_spire(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_best_score_coin_golf_snow_spire)(self as *mut CGameScriptInterface) }
    }
    pub fn get_best_score_shove_ha_penny(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_best_score_shove_ha_penny)(self as *mut CGameScriptInterface) }
    }
    pub fn get_best_time_guess_the_addition(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_best_time_guess_the_addition)(self as *mut CGameScriptInterface) }
    }
    pub fn is_hero_in_tavern_game(&mut self) -> bool {
        unsafe { ((*self.vmt).is_hero_in_tavern_game)(self as *mut CGameScriptInterface) }
    }
    pub fn get_num_houses_owned(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_num_houses_owned)(self as *mut CGameScriptInterface) }
    }
    pub fn start_sneaking(&mut self) {
        unsafe { ((*self.vmt).start_sneaking)(self as *mut CGameScriptInterface) }
    }
    pub fn get_steal_duration(&mut self, a: *const CScriptThing) -> c_long {
        unsafe { ((*self.vmt).get_steal_duration)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_useable_by_hero(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_useable_by_hero)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_owned_by_hero(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_owned_by_hero)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_tavern_table_available_for_use(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_tavern_table_available_for_use)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_is_thing_turncoatable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_is_thing_turncoatable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_is_thing_force_pushable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_is_thing_force_pushable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_is_thing_lightningable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_is_thing_lightningable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_is_thing_epic_spellable(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_is_thing_epic_spellable)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn is_thing_turncoated(&mut self, a: *const CScriptThing) -> bool {
        unsafe { ((*self.vmt).is_thing_turncoated)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_creature_scripted_mode(&mut self, a: *const CScriptThing, b: *const CCharString) {
        unsafe { ((*self.vmt).add_creature_scripted_mode)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn remove_creature_scripted_mode(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).remove_creature_scripted_mode)(self as *mut CGameScriptInterface, a) }
    }
    pub fn force_ships_visible(&mut self) {
        unsafe { ((*self.vmt).force_ships_visible)(self as *mut CGameScriptInterface) }
    }
    pub fn get_sleeping_position_and_orientation_from_bed(&mut self, a: *const CScriptThing, b: *const CScriptThing, c: *mut C3DVector, d: *mut C3DVector) -> bool {
        unsafe { ((*self.vmt).get_sleeping_position_and_orientation_from_bed)(self as *mut CGameScriptInterface, a, b, c, d) }
    }
    pub fn set_bed_availability(&mut self, a: *const CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_bed_availability)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn repopulate_village(&mut self, a: *const CScriptThing) {
        unsafe { ((*self.vmt).repopulate_village)(self as *mut CGameScriptInterface, a) }
    }
    pub fn smash_all_windows_within_radius_of_point(&mut self, a: *const C3DVector, b: c_float) {
        unsafe { ((*self.vmt).smash_all_windows_within_radius_of_point)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_residency(&mut self, a: *const CScriptThing, b: bool) -> CScriptThing {
        unsafe { ((*self.vmt).set_residency)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_thanking_phrase(&mut self, a: CScriptThing, b: c_ulong) {
        unsafe { ((*self.vmt).set_thanking_phrase)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_thanking_phrase(&mut self, a: CScriptThing) -> c_ulong {
        unsafe { ((*self.vmt).get_thanking_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_thanking_phrase(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_thanking_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_ignoring_phrase(&mut self, a: CScriptThing, b: c_ulong) {
        unsafe { ((*self.vmt).set_ignoring_phrase)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_ignoring_phrase(&mut self, a: CScriptThing) -> c_ulong {
        unsafe { ((*self.vmt).get_ignoring_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_ignoring_phrase(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_ignoring_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_wander_centre_point(&mut self, a: CScriptThing, b: C3DVector) {
        unsafe { ((*self.vmt).set_wander_centre_point)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_wander_centre_point(&mut self, a: CScriptThing) -> C3DVector {
        unsafe { ((*self.vmt).get_wander_centre_point)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_wander_centre_point(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_wander_centre_point)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_wander_min_distance(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_wander_min_distance)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_wander_min_distance(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_wander_min_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_wander_min_distance(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_wander_min_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_wander_max_distance(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_wander_max_distance)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_wander_max_distance(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_wander_max_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_wander_max_distance(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_wander_max_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_gossip_counter(&mut self, a: CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).set_gossip_counter)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_gossip_counter(&mut self, a: CScriptThing) -> c_long {
        unsafe { ((*self.vmt).get_gossip_counter)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_gossip_counter(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_gossip_counter)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_max_gossip_phrase(&mut self, a: CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).set_max_gossip_phrase)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_max_gossip_phrase(&mut self, a: CScriptThing) -> c_long {
        unsafe { ((*self.vmt).get_max_gossip_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_max_gossip_phrase(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_max_gossip_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_warning_phrase(&mut self, a: CScriptThing, b: c_ulong) {
        unsafe { ((*self.vmt).set_warning_phrase)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_warning_phrase(&mut self, a: CScriptThing) -> c_ulong {
        unsafe { ((*self.vmt).get_warning_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_warning_phrase(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_warning_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_beer_request_phrase(&mut self, a: CScriptThing, b: c_ulong) {
        unsafe { ((*self.vmt).set_beer_request_phrase)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_beer_request_phrase(&mut self, a: CScriptThing) -> c_ulong {
        unsafe { ((*self.vmt).get_beer_request_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_beer_request_phrase(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_beer_request_phrase)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_scripting_state_group(&mut self, a: CScriptThing, b: EScriptingStateGroups) {
        unsafe { ((*self.vmt).set_scripting_state_group)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_scripting_state_group(&mut self, a: CScriptThing) -> EScriptingStateGroups {
        unsafe { ((*self.vmt).get_scripting_state_group)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_scripting_state_group(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_scripting_state_group)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_max_hero_reaction_distance(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_max_hero_reaction_distance)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_max_hero_reaction_distance(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_max_hero_reaction_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_max_hero_reaction_distance(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_max_hero_reaction_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_action_frequency(&mut self, a: CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).set_action_frequency)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_action_frequency(&mut self, a: CScriptThing) -> c_long {
        unsafe { ((*self.vmt).get_action_frequency)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_action_frequency(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_action_frequency)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_action_frequency_variation(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_action_frequency_variation)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_action_frequency_variation(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_action_frequency_variation)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_action_frequency_variation(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_action_frequency_variation)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_action(&mut self, a: CScriptThing, b: CCharString) {
        unsafe { ((*self.vmt).set_action)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_action(&mut self, a: CScriptThing) -> CCharString {
        unsafe { ((*self.vmt).get_action)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_action(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_action)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_face_hero_for_action(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_face_hero_for_action)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_face_hero_for_action(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_face_hero_for_action)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_face_hero_for_action(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_face_hero_for_action)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_target_name(&mut self, a: CScriptThing, b: CCharString) {
        unsafe { ((*self.vmt).set_target_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_target_name(&mut self, a: CScriptThing) -> CCharString {
        unsafe { ((*self.vmt).get_target_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_target_name(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_target_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_follow_distance(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_follow_distance)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_follow_distance(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_follow_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_follow_distance(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_follow_distance)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_attack_hero_on_sight(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_attack_hero_on_sight)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_attack_hero_on_sight(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_attack_hero_on_sight)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_attack_hero_on_sight(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_attack_hero_on_sight)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_time_to_spend_harassing_hero(&mut self, a: CScriptThing, b: c_long) {
        unsafe { ((*self.vmt).set_time_to_spend_harassing_hero)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_time_to_spend_harassing_hero(&mut self, a: CScriptThing) -> c_long {
        unsafe { ((*self.vmt).get_time_to_spend_harassing_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_time_to_spend_harassing_hero(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_time_to_spend_harassing_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_combat_nearby_enemy_fleeing_break_off_range(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_combat_nearby_enemy_fleeing_break_off_range)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_combat_nearby_enemy_fleeing_break_off_range(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_combat_nearby_enemy_fleeing_break_off_range)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_combat_nearby_enemy_fleeing_break_off_range(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_combat_nearby_enemy_fleeing_break_off_range)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_combat_nearby_break_off_range(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_combat_nearby_break_off_range)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_combat_nearby_break_off_range(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_combat_nearby_break_off_range)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_combat_nearby_break_off_range(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_combat_nearby_break_off_range)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_steal_stealable_items(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_steal_stealable_items)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_steal_stealable_items(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_steal_stealable_items)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_steal_stealable_items(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_steal_stealable_items)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_recover_stealable_items(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_recover_stealable_items)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_recover_stealable_items(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_recover_stealable_items)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_recover_stealable_items(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_recover_stealable_items)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_take_stealable_item_to_random_destination(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_take_stealable_item_to_random_destination)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_take_stealable_item_to_random_destination(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_take_stealable_item_to_random_destination)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_take_stealable_item_to_random_destination(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_take_stealable_item_to_random_destination)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_kill_self_and_stealable_item_after_reaching_destination(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_kill_self_and_stealable_item_after_reaching_destination)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_kill_self_and_stealable_item_after_reaching_destination(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_kill_self_and_stealable_item_after_reaching_destination)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_kill_self_and_stealable_item_after_reaching_destination(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_kill_self_and_stealable_item_after_reaching_destination)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_allowed_to_follow(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_allowed_to_follow)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_allowed_to_follow(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_allowed_to_follow)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_allowed_to_follow(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_allowed_to_follow)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_table_name(&mut self, a: CScriptThing, b: CCharString) {
        unsafe { ((*self.vmt).set_table_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_table_name(&mut self, a: CScriptThing) -> CCharString {
        unsafe { ((*self.vmt).get_table_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_table_name(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_table_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_seat_name(&mut self, a: CScriptThing, b: CCharString) {
        unsafe { ((*self.vmt).set_seat_name)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_seat_name(&mut self, a: CScriptThing) -> CCharString {
        unsafe { ((*self.vmt).get_seat_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_seat_name(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_seat_name)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_disable_head_looking(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_disable_head_looking)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_disable_head_looking(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_disable_head_looking)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_disable_head_looking(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_disable_head_looking)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_is_pushable_by_hero(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_is_pushable_by_hero)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_is_pushable_by_hero(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_is_pushable_by_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_is_pushable_by_hero(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_is_pushable_by_hero)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_look_for_finite_time(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_look_for_finite_time)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_look_for_finite_time(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_look_for_finite_time)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_look_for_finite_time(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_look_for_finite_time)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_avoid_region_exits(&mut self, a: CScriptThing, b: bool) {
        unsafe { ((*self.vmt).set_avoid_region_exits)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_avoid_region_exits(&mut self, a: CScriptThing) -> bool {
        unsafe { ((*self.vmt).get_avoid_region_exits)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_avoid_region_exits(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_avoid_region_exits)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_targeting_distance_offset(&mut self, a: CScriptThing, b: c_float) {
        unsafe { ((*self.vmt).set_targeting_distance_offset)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_targeting_distance_offset(&mut self, a: CScriptThing) -> c_float {
        unsafe { ((*self.vmt).get_targeting_distance_offset)(self as *mut CGameScriptInterface, a) }
    }
    pub fn reset_targeting_distance_offset(&mut self, a: CScriptThing) {
        unsafe { ((*self.vmt).reset_targeting_distance_offset)(self as *mut CGameScriptInterface, a) }
    }
    pub fn set_player_using_melee_dummies(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_player_using_melee_dummies)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_player_using_melee_dummies(&mut self) -> bool {
        unsafe { ((*self.vmt).get_player_using_melee_dummies)(self as *mut CGameScriptInterface) }
    }
    pub fn set_player_using_ranged_dummies(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_player_using_ranged_dummies)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_player_using_ranged_dummies(&mut self) -> bool {
        unsafe { ((*self.vmt).get_player_using_ranged_dummies)(self as *mut CGameScriptInterface) }
    }
    pub fn set_player_using_will_dummies(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_player_using_will_dummies)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_player_using_will_dummies(&mut self) -> bool {
        unsafe { ((*self.vmt).get_player_using_will_dummies)(self as *mut CGameScriptInterface) }
    }
    pub fn set_cheap_head_looking(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_cheap_head_looking)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_cheap_head_looking(&mut self) -> bool {
        unsafe { ((*self.vmt).get_cheap_head_looking)(self as *mut CGameScriptInterface) }
    }
    pub fn set_quit_tavern_game(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_quit_tavern_game)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_quit_tavern_game(&mut self) -> bool {
        unsafe { ((*self.vmt).get_quit_tavern_game)(self as *mut CGameScriptInterface) }
    }
    pub fn set_prize_tavern_table(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_prize_tavern_table)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_prize_tavern_table(&mut self) -> bool {
        unsafe { ((*self.vmt).get_prize_tavern_table)(self as *mut CGameScriptInterface) }
    }
    pub fn set_betting_active(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_betting_active)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_betting_active(&mut self) -> bool {
        unsafe { ((*self.vmt).get_betting_active)(self as *mut CGameScriptInterface) }
    }
    pub fn set_betting_accept(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_betting_accept)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_betting_accept(&mut self) -> bool {
        unsafe { ((*self.vmt).get_betting_accept)(self as *mut CGameScriptInterface) }
    }
    pub fn set_betting_amount(&mut self, a: c_long) {
        unsafe { ((*self.vmt).set_betting_amount)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_betting_amount(&mut self) -> c_long {
        unsafe { ((*self.vmt).get_betting_amount)(self as *mut CGameScriptInterface) }
    }
    pub fn set_count_bet_money_down(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_count_bet_money_down)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_count_bet_money_down(&mut self) -> bool {
        unsafe { ((*self.vmt).get_count_bet_money_down)(self as *mut CGameScriptInterface) }
    }
    pub fn set_spot_the_addition_beaten(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_spot_the_addition_beaten)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_spot_the_addition_beaten(&mut self) -> bool {
        unsafe { ((*self.vmt).get_spot_the_addition_beaten)(self as *mut CGameScriptInterface) }
    }
    pub fn set_global_targeting_distance_offset(&mut self, a: c_float) {
        unsafe { ((*self.vmt).set_global_targeting_distance_offset)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_global_targeting_distance_offset(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_global_targeting_distance_offset)(self as *mut CGameScriptInterface) }
    }
    pub fn set_trading_price_mult(&mut self, a: c_float) {
        unsafe { ((*self.vmt).set_trading_price_mult)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_trading_price_mult(&mut self) -> c_float {
        unsafe { ((*self.vmt).get_trading_price_mult)(self as *mut CGameScriptInterface) }
    }
    pub fn set_boasting_enabled(&mut self, a: bool) {
        unsafe { ((*self.vmt).set_boasting_enabled)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_boasting_enabled(&mut self) -> bool {
        unsafe { ((*self.vmt).get_boasting_enabled)(self as *mut CGameScriptInterface) }
    }
    pub fn set_active_gossip_categories(&mut self, a: CCharString, b: bool) {
        unsafe { ((*self.vmt).set_active_gossip_categories)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_active_gossip_categories(&mut self) -> *const cxx::StdMap<CCharString, bool> {
        unsafe { ((*self.vmt).get_active_gossip_categories)(self as *mut CGameScriptInterface) }
    }
    pub fn get_active_gossip_categories_2(&mut self, a: CCharString) -> *mut bool {
        unsafe { ((*self.vmt).get_active_gossip_categories_2)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_active_gossip_categories_size(&mut self) -> i32 {
        unsafe { ((*self.vmt).get_active_gossip_categories_size)(self as *mut CGameScriptInterface) }
    }
    pub fn clear_active_gossip_categories(&mut self) {
        unsafe { ((*self.vmt).clear_active_gossip_categories)(self as *mut CGameScriptInterface) }
    }
    pub fn get_is_gossip_for_player(&mut self) -> *const cxx::StdMap<CCharString, bool> {
        unsafe { ((*self.vmt).get_is_gossip_for_player)(self as *mut CGameScriptInterface) }
    }
    pub fn get_is_gossip_for_player_2(&mut self, a: CCharString) -> *mut bool {
        unsafe { ((*self.vmt).get_is_gossip_for_player_2)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_is_gossip_for_player_size(&mut self) -> i32 {
        unsafe { ((*self.vmt).get_is_gossip_for_player_size)(self as *mut CGameScriptInterface) }
    }
    pub fn clear_is_gossip_for_player(&mut self) {
        unsafe { ((*self.vmt).clear_is_gossip_for_player)(self as *mut CGameScriptInterface) }
    }
    pub fn set_gossip(&mut self, a: CCharString, b: CCharString, c: c_long) {
        unsafe { ((*self.vmt).set_gossip)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn get_gossip(&mut self, a: CCharString) -> *const cxx::StdVector<CCharString> {
        unsafe { ((*self.vmt).get_gossip)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_gossip_2(&mut self, a: CCharString, b: c_long) -> CCharString {
        unsafe { ((*self.vmt).get_gossip_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_gossip_size(&mut self, a: CCharString) -> i32 {
        unsafe { ((*self.vmt).get_gossip_size)(self as *mut CGameScriptInterface, a) }
    }
    pub fn clear_gossip(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).clear_gossip)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_gossip(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).remove_gossip)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_gossip(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).add_gossip)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_gossip_2(&mut self, a: CCharString, b: CCharString) {
        unsafe { ((*self.vmt).add_gossip_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_gossip_villages(&mut self, a: CCharString, b: CCharString, c: c_long) {
        unsafe { ((*self.vmt).set_gossip_villages)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn get_gossip_villages(&mut self, a: CCharString) -> *const cxx::StdVector<CCharString> {
        unsafe { ((*self.vmt).get_gossip_villages)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_gossip_villages_2(&mut self, a: CCharString, b: c_long) -> CCharString {
        unsafe { ((*self.vmt).get_gossip_villages_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_gossip_villages_size(&mut self, a: CCharString) -> i32 {
        unsafe { ((*self.vmt).get_gossip_villages_size)(self as *mut CGameScriptInterface, a) }
    }
    pub fn clear_gossip_villages(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).clear_gossip_villages)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_gossip_villages(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).remove_gossip_villages)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_gossip_villages(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).add_gossip_villages)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_gossip_villages_2(&mut self, a: CCharString, b: CCharString) {
        unsafe { ((*self.vmt).add_gossip_villages_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn set_gossip_factions(&mut self, a: CCharString, b: CCharString, c: c_long) {
        unsafe { ((*self.vmt).set_gossip_factions)(self as *mut CGameScriptInterface, a, b, c) }
    }
    pub fn get_gossip_factions(&mut self, a: CCharString) -> *const cxx::StdVector<CCharString> {
        unsafe { ((*self.vmt).get_gossip_factions)(self as *mut CGameScriptInterface, a) }
    }
    pub fn get_gossip_factions_2(&mut self, a: CCharString, b: c_long) -> CCharString {
        unsafe { ((*self.vmt).get_gossip_factions_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn get_gossip_factions_size(&mut self, a: CCharString) -> i32 {
        unsafe { ((*self.vmt).get_gossip_factions_size)(self as *mut CGameScriptInterface, a) }
    }
    pub fn clear_gossip_factions(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).clear_gossip_factions)(self as *mut CGameScriptInterface, a) }
    }
    pub fn remove_gossip_factions(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).remove_gossip_factions)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_gossip_factions(&mut self, a: CCharString) {
        unsafe { ((*self.vmt).add_gossip_factions)(self as *mut CGameScriptInterface, a) }
    }
    pub fn add_gossip_factions_2(&mut self, a: CCharString, b: CCharString) {
        unsafe { ((*self.vmt).add_gossip_factions_2)(self as *mut CGameScriptInterface, a, b) }
    }
    pub fn c_game_script_interface_destructor(&mut self) {
        unsafe { ((*self.vmt).c_game_script_interface_destructor)(self as *mut CGameScriptInterface) }
    }
}

// #[derive(Debug)]
#[repr(C)]
pub struct CGameScriptInterfaceVmt {
    pub end_letter_box: extern "thiscall" fn(*mut CGameScriptInterface),
    pub error: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, c_ulong),
    pub trace_message: extern "thiscall"  fn(*mut CGameScriptInterface, *const CCharString),
    pub validate: extern "thiscall"  fn(*mut CGameScriptInterface),
    pub set_debug_camera_type: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub deactivate_boast_ui: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_xbox: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub new_script_frame: extern "thiscall" fn(*mut CGameScriptInterface),
    pub start_scripting_entity: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut CScriptGameResourceObjectScriptedThingBase, EScriptAIPriority) -> bool,
    pub is_entity_under_scripted_control: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_active_thread_terminating: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub is_level_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_region_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_region_loaded_and_preloaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_region_def_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub get_region_name: extern "thiscall" fn(*mut CGameScriptInterface) -> *const CCharString,
    pub msg_is_level_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_is_level_unloaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_on_level_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdList<CCharString>) -> bool,
    pub msg_on_level_unloaded: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdList<CCharString>) -> bool,
    pub msg_is_region_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_is_region_unloaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_on_region_loaded: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_region_unloaded: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_on_region_preunloaded: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_quest_completed: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_any_quest_completed: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_quest_failed: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_on_quest_completed_before_screen_shown: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_on_quest_failed_before_screen_shown: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub msg_on_quest_accepted: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_feat_accepted: extern "thiscall" fn(*mut CGameScriptInterface, *mut c_long) -> bool,
    pub msg_is_boast_made: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> bool,
    pub msg_on_boast_made: extern "thiscall" fn(*mut CGameScriptInterface, *mut c_long, *mut CCharString) -> bool,
    pub msg_on_boasts_made: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdVector<CCharString>) -> bool,
    pub remove_boast_message: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_quest_start_screen_active: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_leaving_quest_start_screen: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_leaving_experience_spending_screen: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_is_answered_yes_or_no: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub msg_is_game_info_clicked_past: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_is_tutorial_click_past: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_is_action_mode_button_pressed: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_on_expression_performed: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_is_cut_scene_skipped: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub remove_all_cut_scene_skipped_messages: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString),
    pub msg_on_hero_hair_type_changed: extern "thiscall" fn(*mut CGameScriptInterface, EClothingCoversArea, *mut CCharString) -> bool,
    pub msg_on_hero_used_teleporter: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_hero_used_guild_seal: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_on_game_saved_manually: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_on_hero_slept: extern "thiscall" fn(*mut CGameScriptInterface, *mut bool) -> bool,
    pub msg_on_hero_fired_ranged_weapon: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub msg_on_hero_cast_spell: extern "thiscall" fn(*mut CGameScriptInterface, *mut EHeroAbility) -> bool,
    pub msg_on_hero_picked_pocket: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing) -> bool,
    pub msg_on_hero_picked_lock: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing) -> bool,
    pub msg_on_fishing_game_finished: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing) -> bool,
    pub msg_on_tavern_game_finished: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub msg_on_hero_rewarded_with_items_from: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub msg_on_chest_opening_cancelled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub dont_populate_next_loaded_region: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_wandering_population_script_def_name_in_current_region: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString),
    pub get_wandering_population_script_def_name_in_region: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *mut CCharString),
    pub is_hero_allowed_henchmen_in_current_region: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_hero_allowed_henchmen_in_region: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub post_add_scripted_entities: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_player_holding_unsheathe_ranged_weapon_button: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_player_holding_lock_target_button: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_player_holding_fire_ranged_weapon_button: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_player_holding_first_person_targeting_button: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_hero_in_projectile_weapon_mode: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub register_timer: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub deregister_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub set_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long),
    pub get_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> c_long,
    pub get_hero: extern "thiscall" fn(*mut CGameScriptInterface) -> *mut CScriptThing,
    pub get_hero_targeted_thing: extern "thiscall" fn(*mut CGameScriptInterface) -> CScriptThing,
    pub get_thing_with_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> CScriptThing,
    pub get_thing_with_script_name_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString) -> CScriptThing,
    pub get_random_thing_with_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> CScriptThing,
    pub get_all_things_with_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *mut cxx::StdVector<CScriptThing>) -> c_long,
    pub get_all_creatures_in_area_with_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, c_float, *const cxx::StdVector<CScriptThing>) -> c_long,
    pub get_nearest_with_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub get_furthest_with_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub get_all_things_with_def_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *mut cxx::StdVector<CScriptThing>) -> c_long,
    pub get_all_things_with_def_name_by_distance_from: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, *mut cxx::StdVector<CScriptThing>) -> c_long,
    pub get_nearest_with_def_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub get_furthest_with_def_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub get_thing_with_uid: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong) -> CScriptThing,
    pub get_all_creatures_excluding_hero: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdVector<CScriptThing>) -> c_long,
    pub get_all_things_in_level: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *mut cxx::StdVector<CScriptThing>) -> c_long,
    pub is_thing_with_this_uid_alive: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong) -> bool,
    pub create_creature: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, *const CCharString, bool) -> CScriptThing,
    pub create_creature_nearby: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, c_float, *const CCharString, bool) -> CScriptThing,
    pub create_creature_on_entity: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub turn_creature_into: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub set_creature_creation_delay_frames: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub reset_creature_creation_delay_frames: extern "thiscall" fn(*mut CGameScriptInterface),
    pub create_object: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, c_float, *const CCharString) -> CScriptThing,
    pub create_object_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, *const CCharString) -> CScriptThing,
    pub create_object_on_entity: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CScriptThing, *const CCharString) -> CScriptThing,
    pub create_effect: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, *const CCharString, c_float, bool, bool) -> CScriptThing,
    pub create_effect_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CScriptThing, *const CCharString, *const CCharString, bool, bool) -> CScriptThing,
    pub create_light: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const CRGBColour, *const CCharString, c_float, c_float, bool) -> CScriptThing,
    pub create_experience_orb: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_long) -> CScriptThing,
    pub create_explosion: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, *const C3DVector, CCharString) -> CScriptThing,
    pub create_physical_barrier: extern "thiscall" fn(*mut CGameScriptInterface, c_float, *const C3DVector, *const C3DVector, CCharString) -> CScriptThing,
    pub create_rumble: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float, c_float, CCharString) -> CScriptThing,
    pub clear_all_rumbles: extern "thiscall" fn(*mut CGameScriptInterface),
    pub remove_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
    pub show_on_screen_message: extern "thiscall" fn(*mut CGameScriptInterface, *const C2DVector, *const CCharString, *const CRGBColour, *const CCharString),
    pub show_on_screen_message_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, *const CCharString, c_float),
    pub show_on_screen_message_3: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub add_screen_message: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, ETextGroupSelectionMethod),
    pub add_screen_title_message: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float, bool),
    pub give_hero_yes_no_question: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, *const CCharString, *const CCharString, bool),
    pub display_game_info: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub display_game_info_text: extern "thiscall" fn(*mut CGameScriptInterface, *const CWideString),
    pub is_safe_to_display_game_info: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub display_tutorial: extern "thiscall" fn(*mut CGameScriptInterface, ETutorialCategory) -> bool,
    pub is_tutorial_system_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub give_hero_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub give_hero_object: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, bool),
    pub set_weapon_as_heros_active_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub give_hero_item: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub give_hero_items_from_container: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
    pub take_object_from_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub give_hero_gold: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub get_hero_gold: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub give_hero_experience: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub set_hero_able_to_gain_experience: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub sheathe_hero_weapons: extern "thiscall" fn(*mut CGameScriptInterface),
    pub set_hero_will_as_usable: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_hero_weapons_as_usable: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_weapon_out_crime_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_guards_ignore_crimes: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub remove_all_hero_weapons: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_reported_or_unreported_crime_known: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub confiscate_all_hero_items: extern "thiscall" fn(*mut CGameScriptInterface),
    pub confiscate_all_hero_weapons: extern "thiscall" fn(*mut CGameScriptInterface),
    pub confiscate_items_of_type_from_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub return_all_confiscated_items_to_hero: extern "thiscall" fn(*mut CGameScriptInterface),
    pub give_hero_tutorial: extern "thiscall" fn(*mut CGameScriptInterface, ETutorialCategory) -> bool,
    pub make_hero_carry_item_in_hand: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
    pub make_hero_carry_item_in_hand_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub add_tattoo_to_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub give_hero_ability: extern "thiscall" fn(*mut CGameScriptInterface, EHeroAbility, bool),
    pub is_player_z_targeting_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_player_creature_blocking: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_player_creature_ready_to_fire_projectile_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *mut c_float) -> bool,
    pub get_player_creature_combat_multiplier: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_player_creature_combat_multiplier_running_num_hits: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub reset_player_creature_combat_multiplier: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_player_creature_flourish_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_player_creature_only_target: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub reset_player_creature_only_target: extern "thiscall" fn(*mut CGameScriptInterface),
    pub respawn_hero: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub give_hero_morality: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_hero_morality: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_morality_category: extern "thiscall" fn(*mut CGameScriptInterface) -> EMorality,
    pub change_hero_morality_due_to_theft: extern "thiscall" fn(*mut CGameScriptInterface),
    pub change_hero_morality_due_to_picklock: extern "thiscall" fn(*mut CGameScriptInterface),
    pub give_hero_renown_points: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub get_hero_renown_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub is_hero_renown_level_full: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub increase_hero_renown_level: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_hero_strength_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_skill_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_will_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_stat_level: extern "thiscall" fn(*mut CGameScriptInterface, EHeroTrainableStatType) -> c_long,
    pub get_hero_stat_max: extern "thiscall" fn(*mut CGameScriptInterface, EHeroTrainableStatType) -> c_long,
    pub set_hero_age: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_hero_age: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub set_hero_as_teenager: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_hero_as_apprentice: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_distance_hero_can_be_heard_from: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_rough_experience_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_experience_available_to_spend: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_fatness: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_scariness: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_attractiveness: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_will_energy_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub set_hero_will_energy_level: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub set_hero_will_energy_as_able_to_refill: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_number_of_items_of_type_in_inventory: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> c_long,
    pub is_object_in_things_possession: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CScriptThing) -> bool,
    pub is_hero_hand_lamp_lit: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_hero_hand_lamp_as_lit: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub is_wearing_clothing_item: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> bool,
    pub is_hero_naked: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub remove_hero_clothing: extern "thiscall" fn(*mut CGameScriptInterface),
    pub set_hero_as_wearing: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub change_hero_hairstyle: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub remove_hero_hairstyle: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_wearing_hairstyle: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, *const CCharString) -> bool,
    pub is_player_carrying_item_of_type: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_player_wielding_weapon: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_entity_wielding_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_entity_wielding_melee_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_entity_wielding_ranged_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub get_previously_wielded_melee_weapon_name: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub get_previously_wielded_ranged_weapon_name: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
    pub is_entity_able_to_attack: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub entity_get_thing_in_primary_slot: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
    pub apply_hero_penalty_for_death: extern "thiscall" fn(*mut CGameScriptInterface),
    pub give_hero_title: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub get_hero_title: extern "thiscall" fn(*mut CGameScriptInterface) -> EHeroTitle,
    pub entity_set_as_marryable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_able_to_region_follow_when_married: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_force_marriage_to_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub is_entity_married_to_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_entity_marriable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub get_hero_has_married: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub get_hero_has_current_marriage: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub get_hero_has_divorced_marriage: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub get_hero_has_children: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub get_hero_has_murdered_wife: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_hero_child: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub cancel_hero_teleport_effects: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_number_of_times_hero_has_had_sex: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub set_number_of_times_hero_has_had_sex: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub set_hero_as_having_had_sex: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_hero_as_having_had_gay_sex: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub give_thing_hero_reward_item: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, *const CCharString),
    pub give_thing_item_in_hand: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, bool),
    pub give_thing_item_in_slot: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, *const CCharString),
    pub give_hero_expression: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, bool),
    pub hero_has_expression: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_hero_performing_expression: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_d_pad_button_held_for_expression: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub entity_follow_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, c_float, bool),
    pub entity_stop_following: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub get_following_entity_list: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut cxx::StdVector<CScriptThing>) -> bool,
    pub get_perceiving_hero_entity_list: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdVector<CScriptThing>) -> bool,
    pub get_hero_summoned_creatures_list: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdVector<CScriptThing>) -> bool,
    pub is_entity_following_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub entity_set_as_allowed_to_follow_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_allowed_to_change_region_following_state: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_responding_to_follow_and_wait_expressions: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_mirroring_hero_enemy_relations_while_following: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub teleport_all_followers_to_hero_position: extern "thiscall" fn(*mut CGameScriptInterface),
    pub entity_teleport_to_hero_position: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub send_entity_event: extern "thiscall" fn(*mut CGameScriptInterface, NEntityEvents::EEventType, *const CScriptThing, *const CScriptThing, *mut CThing),
    pub get_water_height_at_position: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector) -> c_float,
    pub is_fishing_spot_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub disable_fishing_spot: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub update_fish_weight: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub get_best_fish_weight: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> c_float,
    pub hero_go_fishing: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_hero_fishing_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub hero_go_digging: extern "thiscall" fn(*mut CGameScriptInterface),
    pub hero_stop_digging: extern "thiscall" fn(*mut CGameScriptInterface),
    pub hero_play_oracle_minigame: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_hero_playing_oracle_minigame: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub has_hero_won_oracle_minigame: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub hero_play_fireheart_minigame: extern "thiscall" fn(*mut CGameScriptInterface),
    pub hero_quit_fireheart_minigame: extern "thiscall" fn(*mut CGameScriptInterface),
    pub has_hero_force_quit_fireheart_minigame: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub get_hero_health: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_health_max: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_health_percentage: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_hero_will_energy: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_will_energy_max: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_hero_will_energy_percentage: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub change_hero_health_by: extern "thiscall" fn(*mut CGameScriptInterface, c_float, bool, bool),
    pub set_thing_as_killed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub get_health: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_float,
    pub modify_thing_health: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
    pub entity_set_max_health: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
    pub give_hero_new_quest_objective: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_ulong),
    pub tell_hero_quest_objective_completed: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong),
    pub tell_hero_quest_objective_failed: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong),
    pub add_quest_region: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub set_quest_world_map_offset: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C2DCoordI),
    pub is_hero_on_quest: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub hero_receive_message_from_guild_master: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, bool, bool),
    pub set_guild_master_messages: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub activate_quest: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub activate_multiple_quests: extern "thiscall" fn(*mut CGameScriptInterface, *const CArray<CCharString>),
    pub activate_quest_without_loading_resources: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub activate_multiple_quests_without_loading_resources: extern "thiscall" fn(*mut CGameScriptInterface, *const CArray<CCharString>),
    pub deactivate_quest: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_ulong),
    pub deactivate_quest_later: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_ulong),
    pub prepare_quests_when_final_quest_is_activated: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub prepare_quests_when_final_quest_is_completed: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_quest_active: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_quest_registered: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_quest_completed: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_quest_failed: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub set_quest_as_completed: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool, bool, bool),
    pub set_quest_as_failed: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool, *const CWideString, bool),
    pub set_quest_as_persistent: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub get_exclusive_quest_script_name: extern "thiscall" fn(*mut CGameScriptInterface) -> *const CCharString,
    pub add_quest_card: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, bool, bool),
    pub remove_quest_card_from_guild: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub remove_quest_card_from_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub give_hero_quest_card_directly: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, bool),
    pub set_quest_card_objective: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, *const CCharString, *const CCharString),
    pub set_quest_card_gold_reward: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long),
    pub set_quest_card_renown_reward: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long),
    pub remove_all_available_quest_cards_from_guild: extern "thiscall" fn(*mut CGameScriptInterface),
    pub fail_all_active_quests: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_all_active_quest_info: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdVector<CCharString>, *mut cxx::StdVector<CCharString>),
    pub add_feat_card: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString, *const CCharString),
    pub add_boast: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, c_long, c_long, bool, *const CCharString, c_long),
    pub remove_boast: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString),
    pub set_boast_as_failed: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString),
    pub set_boast_as_completed: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString),
    pub is_boast_taken: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString) -> bool,
    pub add_log_book_entry: extern "thiscall" fn(*mut CGameScriptInterface, *const CWideString, *const CWideString, *const CWideString, ECategory),
    pub kick_off_quest_start_screen: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool, bool),
    pub kick_off_death_screen: extern "thiscall" fn(*mut CGameScriptInterface),
    pub kick_off_credits_screen: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString),
    pub set_preferred_quick_access_item: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, c_long),
    pub get_death_recovery_marker_name: extern "thiscall" fn(*mut CGameScriptInterface) -> CCharString,
    pub set_death_recovery_marker_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub reset_death_recovery_marker_name_to_default: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_to_fail_quest_on_death: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_whether_to_fail_quest_on_death: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub reset_whether_to_fail_quest_on_death_to_default: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_most_recent_valid_used_target: extern "thiscall" fn(*mut CGameScriptInterface) -> CScriptThing,
    pub get_most_recent_valid_used_target_name: extern "thiscall" fn(*mut CGameScriptInterface) -> CCharString,
    pub display_quest_info: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_quest_info_name: extern "thiscall" fn(*mut CGameScriptInterface, *const c_char),
    pub set_quest_info_text: extern "thiscall" fn(*mut CGameScriptInterface, *const c_char),
    pub add_quest_info_bar: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, *const CRGBColour, *const CRGBColour, *const CCharString, *const CCharString, c_float) -> c_long,
    pub add_quest_info_bar_health: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CRGBColour, *const CCharString, c_float) -> c_long,
    pub add_quest_info_timer: extern "thiscall" fn(*mut CGameScriptInterface, *const CTimer, *const CCharString, c_float) -> c_long,
    pub add_quest_info_counter: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, c_float) -> c_long,
    pub add_quest_info_counter_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCounter, *const CCharString, c_float) -> c_long,
    pub add_quest_info_counter_list: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, c_float) -> c_long,
    pub add_quest_info_tick: extern "thiscall" fn(*mut CGameScriptInterface, EGameAction, bool, c_float) -> c_long,
    pub add_quest_info_tick_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool, c_float) -> c_long,
    pub update_quest_info_bar: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_float, c_float, c_float),
    pub change_quest_info_bar_colour: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CRGBColour, *const CRGBColour),
    pub update_quest_info_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_float),
    pub update_quest_info_counter: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long, c_long),
    pub update_quest_info_counter_list: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long, c_long),
    pub update_quest_info_tick: extern "thiscall" fn(*mut CGameScriptInterface, c_long, bool),
    pub remove_quest_info_element: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub remove_all_quest_info_elements: extern "thiscall" fn(*mut CGameScriptInterface),
    pub display_time: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub display_money_bag: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub display_mini_game_info: extern "thiscall" fn(*mut CGameScriptInterface, bool, EMiniGameType),
    pub update_mini_game_info_bar: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub is_entity_pick_pocketable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_entity_pick_lockable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_entity_stealable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub entity_set_as_pick_pocketed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_as_pick_locked: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_as_stolen: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub mini_map_add_marker: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub mini_map_set_marker_graphic: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub mini_map_remove_marker: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub mini_map_remove_all_markers: extern "thiscall" fn(*mut CGameScriptInterface),
    pub mini_map_allow_route_between_regions: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString, bool),
    pub mini_map_set_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub entity_set_as_hidden_on_mini_map: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_hud_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub text_entry_exists: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub get_valid_text_entry_name_with_attitude: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> CCharString,
    pub set_thing_has_information: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool, bool),
    pub clear_thing_has_information: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_will_be_using_narrator: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_reset_as_pure_ai_narrator: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub add_new_conversation: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool) -> c_long,
    pub add_person_to_conversation: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CScriptThing),
    pub add_line_to_conversation: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString, bool, *const CScriptThing, *const CScriptThing),
    pub remove_conversation: extern "thiscall" fn(*mut CGameScriptInterface, c_long, bool),
    pub is_conversation_active: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> bool,
    pub play_avi_movie: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub start_movie_sequence: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *mut CScriptGameResourceObjectMovieBase),
    pub fix_movie_sequence_camera: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub fade_screen_out_until_next_call_to_fade_screen_in: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float),
    pub fade_screen_out: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, CRGBColour) -> bool,
    pub fade_screen_in: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_screen_fading_out: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub pause: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub end_cut_fade: extern "thiscall" fn(*mut CGameScriptInterface),
    pub pause_all_non_scripted_entities: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub pause_all_entities: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_allow_screen_fading_on_next_region_change: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_allow_screen_fading_if_already_faded: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_to_keep_hero_abilities_during_cutscenes: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_to_display_tutorials_during_cutscenes: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_cutscene_mode: extern "thiscall" fn(*mut CGameScriptInterface, bool, bool),
    pub is_in_cutscene: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_cutscene_skippable: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_cutscene_skippable_while_paused: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_cutscene_action_mode: extern "thiscall" fn(*mut CGameScriptInterface, bool, *const CCharString),
    pub preload_new_scene: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub start_progress_display: extern "thiscall" fn(*mut CGameScriptInterface),
    pub stop_progress_display: extern "thiscall" fn(*mut CGameScriptInterface),
    pub set_screen_messages_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub is_hero_controlled_by_player: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub is_in_movie_sequence: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub cancel_abilities_for_cutscene: extern "thiscall" fn(*mut CGameScriptInterface),
    pub resume_abilities_for_cutscene: extern "thiscall" fn(*mut CGameScriptInterface),
    pub cancel_using_ability: extern "thiscall" fn(*mut CGameScriptInterface, EHeroAbility),
    pub set_ability_availability: extern "thiscall" fn(*mut CGameScriptInterface, EHeroAbility, bool),
    pub set_environmental_effects_always_update: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub remove_dead_creature: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub camera_set_camera_preload_flag: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub camera_circle_around_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const C3DVector, c_float),
    pub camera_circle_around_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const C3DVector, c_float),
    pub camera_move_to_pos_and_look_at_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const C3DVector, c_float),
    pub camera_move_to_pos_and_look_at_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const CScriptThing, c_float),
    pub camera_move_between_looking_at: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const C3DVector, *const CScriptThing, c_float, c_float),
    pub camera_move_between_looking_at_2: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const C3DVector, *const C3DVector, c_float, c_float),
    pub camera_move_between_look_from_and_look_to: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const C3DVector, *const C3DVector, *const C3DVector, c_float),
    pub camera_use_camera_point: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const C3DVector, *const CRightHandedSet, c_float, c_long, c_long),
    pub camera_use_camera_point_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C3DVector, *const CRightHandedSet, c_float, c_long, c_long),
    pub camera_use_camera_point_3: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, c_float, c_long, c_long),
    pub camera_use_camera_point_4: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CScriptThing, c_float, c_long, c_long),
    pub camera_do_conversation: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, ECameraOp, bool),
    pub camera_default: extern "thiscall" fn(*mut CGameScriptInterface),
    pub camera_reset_to_view_behind_hero: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub is_camera_in_scripted_mode: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub camera_use_screen_effect: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, c_float),
    pub camera_cancel_screen_effect: extern "thiscall" fn(*mut CGameScriptInterface),
    pub is_camera_pos_on_screen: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector) -> bool,
    pub get_game_angle_xy: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub camera_earthquake_intensity_at_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float, c_float),
    pub camera_shake: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float),
    pub open_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub close_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub open_house_doors: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub close_house_doors: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub jam_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub set_door_trigger_type: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EDoorTriggerType),
    pub override_automatic_house_locking: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_house_owned_by_player: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
    pub set_buyable_house_as_scripted: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub is_chest_open: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub open_chest: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
    pub close_chest: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub get_number_of_keys_needed_to_unlock_chest: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut CCharString) -> c_long,
    pub display_locked_chest_message: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub set_trophy_as_mountable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_village_limbo: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_creature_not_reload: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub is_sleeping_time: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub enable_guards: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub enable_villager_def_types: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, *const CCharString),
    pub try_to_respawn_def_named: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, *const C3DVector) -> CScriptThing,
    pub clear_hero_enemy_of_guards: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub set_thing_as_usable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_thing_home_building: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub set_village_attitude: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EScriptVillageAttitude),
    pub add_crime_committed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, ECrime, bool, *const CScriptThing, *const CScriptThing, EOpinionPostDeedType),
    pub give_thing_best_enemy_target: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub clear_thing_best_enemy_target: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_in_limbo: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
    pub is_entity_in_limbo: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub entity_get_shot_strike_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut C3DVector) -> bool,
    pub entity_set_negate_all_hits: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_evade_all_hits: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_able_to_be_engaged_in_combat: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_always_block_attacks_from_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, bool),
    pub entity_set_attack_thing_immediately: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, bool, bool),
    pub entity_set_combat_type: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_reset_combat_type_to_default: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_max_number_of_attackers: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_clear_max_number_of_attackers: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_attach_to_script: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_set_combat_ability: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_set_ranged_target: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_clear_ranged_target: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_targetable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_targeting_type: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, ETargetingType),
    pub entity_set_targeting_valid_target_without_los: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_teleport_to_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, bool),
    pub entity_teleport_to_position: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const C3DVector, c_float, bool, bool),
    pub entity_set_facing_angle: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
    pub entity_set_facing_angle_towards_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, bool),
    pub entity_set_perception_variables: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, c_float, c_float),
    pub set_thing_persistent: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_thing_as_wanting_money: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_set_appearance_morph_seed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub set_entity_as_region_following: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, bool),
    pub set_entity_as_following_hero_through_teleporters: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_appearance_seed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_ulong),
    pub entity_get_appearance_seed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut c_ulong),
    pub entity_set_as_for_sale: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_stock_item_price: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_get_stock_item_price: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
    pub entity_play_object_animation: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, bool),
    pub entity_set_max_running_speed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_set_max_walking_speed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_reset_max_running_speed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_reset_max_walking_speed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_attach_to_village: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_set_as_sitting_on_floor: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_scared: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_drunk: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, bool),
    pub entity_set_as_having_bound_hands: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, bool),
    pub entity_set_as_remove_all_movement_blocking_modes: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing),
    pub entity_force_to_look_at_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_force_to_look_at_camera: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_force_to_look_at_nothing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_reset_force_to_look_at: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_shot_accuracy_percentage: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_get_standing_on_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
    pub entity_get_standing_inside_building: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
    pub entity_drop_generic_box: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_sheathe_weapons: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_unsheathe_weapons: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_unsheathe_melee_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_unsheathe_ranged_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_alpha: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
    pub entity_set_as_drawable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_cutscene_behaviour: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, ECutsceneBehaviour),
    pub entity_get_sex: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> ESex,
    pub entity_set_as_able_to_walk_through_solid_objects: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_respond_to_hit: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_damageable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_killable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
    pub entity_set_as_locked: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_decapitate: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_give_gold: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_set_allow_boss_phase_changes: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_get_boss_phase: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
    pub entity_set_boss_phase: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_reset_creature_mode: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_as_receiving_events: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
    pub entity_set_as_to_add_to_combo_multiplier_when_hit: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_to_add_to_stat_changes_when_hit: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_leave_combat_stance: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_as_use_movement_in_actions: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_displaying_emote_icon: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_as_collidable_to_things: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_enable_gravity: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_light_as_on: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_fade_out: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_fade_in: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_begin_loading_animation: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_begin_loading_basic_animations: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_cast_force_push: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
    pub entity_cast_lightning_at_target: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub begin_loading_mesh: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub entity_will_teleport_to_area: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, C3DVector, c_float, c_float) -> bool,
    pub entity_start_screamer_super_attack_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_end_screamer_super_attack_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub set_hero_guide_to_show_quest_cards_when_spoken_to: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_light_colour: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CRGBColour),
    pub creature_generator_set_family: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub creature_generator_trigger: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub creature_generator_set_always_create_creatures_on_trigger: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub creature_generator_is_depleted: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub creature_generator_is_destroyed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub creature_generator_set_generated_creature_script_name: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub creature_generator_set_num_triggers: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub creature_generator_get_num_generated_creatures: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
    pub creature_generator_are_all_creatures_alive: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub creature_generator_add_triggerer: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub creature_generator_remove_triggerer: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub set_creature_generator_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_creature_generators_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub set_creature_generators_enabled_during_script: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub set_creature_generators_creature_group_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, ECreatureGroup, bool),
    pub is_creature_generation_enabled_for_region: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_creature_flying: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub set_teleporter_as_active: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub is_teleporter_active: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub set_teleporting_as_active: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub is_teleporting_active: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_region_exit_as_active: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_region_entrance_as_active: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_region_text_display_as_active: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_hero_sleeping_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub is_hero_sleeping_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_experience_spending_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_morality_changing_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub set_summoner_death_explosion_affects_hero: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_nearest_enabled_digging_spot: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
    pub is_digging_spot_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_digging_spot_hidden: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub set_digging_spot_as_hidden: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub check_for_camera_message: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub wait_for_camera_message: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub set_thing_as_conscious: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, *const CCharString),
    pub set_fire_to_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub extinguish_fires_on_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub is_thing_on_fire: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub add_item_to_container: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub remove_item_from_container: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_set_death_container_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub get_item_def_names_from_container: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut cxx::StdVector<CCharString>),
    pub set_creature_brain: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_set_stategroup_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString, bool),
    pub entity_set_all_stategroups_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_combat_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_sleep_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_opinion_reactions_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_set_deed_reactions_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub debug_get_all_text_entries_for_targeted_thing: extern "thiscall" fn(*mut CGameScriptInterface, *mut cxx::StdSet<c_ulong>),
    pub entity_set_thing_as_enemy_of_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_unset_thing_as_enemy_of_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_set_thing_as_ally_of_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_unset_thing_as_ally_of_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_set_in_faction: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub set_faction_as_allied_to_faction: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub set_faction_as_neutral_to_faction: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub set_faction_as_enemy_to_faction: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub are_entities_enemies: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing) -> bool,
    pub get_next_in_opinion_attitude_graph: extern "thiscall" fn(*mut CGameScriptInterface, EOpinionAttitudeType) -> EOpinionAttitudeType,
    pub get_opinion_attitude_as_string: extern "thiscall" fn(*mut CGameScriptInterface, EOpinionAttitudeType, *mut CCharString),
    pub entity_get_opinion_attitude_to_player: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> EOpinionAttitudeType,
    pub entity_get_opinion_attitude_to_player_as_string: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut CCharString),
    pub entity_get_opinion_of_player: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinion) -> c_float,
    pub entity_set_opinion_reaction_mask: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_set_opinion_reaction_mask_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_set_opinion_deed_mask: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_set_opinion_deed_mask_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_set_opinion_deed_type_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionDeedType, bool),
    pub entity_set_opinion_attitude_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionAttitudeType, bool),
    pub entity_set_opinion_reaction_enabled: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionReactionType, bool),
    pub entity_set_personality_override: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_set_personality_override_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_clear_personality_override: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub entity_set_as_opinion_source: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub entity_set_as_opinion_source_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub entity_unset_as_opinion_source: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub opinion_source_set_as_exclusive: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub opinion_source_set_as_attention_grabbing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub entity_post_opinion_deed_to_all: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionDeedType),
    pub entity_post_opinion_deed_to_recipient: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionDeedType, *const CScriptThing),
    pub entity_post_opinion_deed_to_recipient_village: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionDeedType, *const CScriptThing),
    pub entity_post_opinion_deed_keep_searching_for_witnesses: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionDeedType, *const CScriptThing) -> c_long,
    pub remove_opinion_deed_still_searching_for_witnesses: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
    pub is_deed_witnessed: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> bool,
    pub can_thing_be_seen_by_other_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing) -> bool,
    pub can_thing_be_nearly_seen_by_other_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing) -> bool,
    pub can_thing_be_smelled_by_other_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing) -> bool,
    pub can_thing_be_heard_by_other_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing) -> bool,
    pub is_thing_aware_of_other_thing_in_any_way: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing) -> bool,
    pub entity_set_as_aware_of_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
    pub entity_set_sound_radius: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_set_smell_radius: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_set_sight_radius: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_set_extended_sight_radius: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_set_give_up_chase_radius: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
    pub entity_get_hearing_radius: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_float,
    pub manually_trigger_trap: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub manually_reset_trap: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub set_time_of_day: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_time_of_day: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub set_time_as_stopped: extern "thiscall" fn(*mut CGameScriptInterface, bool, *mut c_long),
    pub fast_forward_time_to: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float),
    pub is_time_of_day_between: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long) -> bool,
    pub get_day_of_week: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_day_count: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_world_frame: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_constant_fps: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_active_quest_name: extern "thiscall" fn(*mut CGameScriptInterface) -> CCharString,
    pub transition_to_theme: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub reset_to_default_theme: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub transition_to_theme_all_internals: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub reset_to_default_theme_all_internals: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub transition_to_theme_externals: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub reset_to_default_theme_externals: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub set_environment_theme_weight_all_channels: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub set_environment_theme_weight_all_internals: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub set_environment_theme_weight_externals: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
    pub set_sound_themes_as_enabled_for_region: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub set_all_sounds_as_muted: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub radial_blur_fade_to: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, c_float, c_float, c_float, c_float, c_float) -> *mut c_void,
    pub radial_blur_fade_to_2: extern "thiscall" fn(*mut CGameScriptInterface, c_float, C3DVector, c_float, C3DVector, c_float, c_float, c_float) -> *mut c_void,
    pub radial_blur_fade_out: extern "thiscall" fn(*mut CGameScriptInterface, c_float, *mut c_void),
    pub is_radial_blur_fade_active: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub cancel_radial_blur_fade: extern "thiscall" fn(*mut CGameScriptInterface),
    pub radial_blur_set_center_world_pos: extern "thiscall" fn(*mut CGameScriptInterface, *mut c_void, *const C3DVector),
    pub displacement_monochrome_effect_colour_fade_to: extern "thiscall" fn(*mut CGameScriptInterface, c_float, *const CRGBFloatColour) -> c_void,
    pub displacement_monochrome_effect_colour_fade_out: extern "thiscall" fn(*mut CGameScriptInterface, c_float, *mut c_void),
    pub screen_filter_fade_to: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, c_float, c_float, c_float, *const CRGBFloatColour, *const cxx::StdVector<CScreenFilterSThingByPass>) -> c_void,
    pub screen_filter_fade_out: extern "thiscall" fn(*mut CGameScriptInterface, c_float, *mut c_void),
    pub set_thing_and_carried_items_not_affected_by_screen_filter: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, *mut c_void),
    pub un_set_thing_and_carried_items_not_affected_by_screen_filter: extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing),
    pub is_gift_romantic: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_gift_friendly: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_gift_offensive: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
    pub is_thing_a_bed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_thing_a_chest: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_thing_a_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_thing_smashable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub is_thing_searchable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub apply_script_brush: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub enable_decals: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub play_criteria_sound_on_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> c_ulong,
    pub play_sound_on_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString) -> c_ulong,
    pub is_sound_playing: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong) -> bool,
    pub stop_sound: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong),
    pub play_sound_at_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, *const CCharString) -> c_ulong,
    pub play_2d_sound: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> c_ulong,
    pub enable_sounds: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub override_music: extern "thiscall" fn(*mut CGameScriptInterface, EMusicSetType, bool, bool),
    pub stop_override_music: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub cache_music_set: extern "thiscall" fn(*mut CGameScriptInterface, EMusicSetType),
    pub enable_danger_music: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub is_danger_music_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub start_countdown_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_countdown_timer: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub auto_save_check_point: extern "thiscall" fn(*mut CGameScriptInterface),
    pub auto_save_quest_start: extern "thiscall" fn(*mut CGameScriptInterface),
    pub auto_save: extern "thiscall" fn(*mut CGameScriptInterface),
    pub set_saving_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub is_saving_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_save_game_marker_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector),
    pub reset_to_front_end: extern "thiscall" fn(*mut CGameScriptInterface),
    pub set_guild_seal_recall_location: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float),
    pub get_guild_seal_recall_pos: extern "thiscall" fn(*mut CGameScriptInterface) -> C3DVector,
    pub get_guild_seal_recall_angle_xy: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub set_readable_object_text: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CWideString),
    pub set_readable_object_text_tag: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub get_formatted_string: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const cxx::StdVector<CWideString>) -> CWideString,
    pub get_text_string: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> CWideString,
    pub add_rumour_category: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub add_new_rumour_to_category: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub remove_rumour_category: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
    pub set_category_activity: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub add_gossip_village: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub add_gossip_faction_to_category: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
    pub set_is_gossip_for_player: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, bool),
    pub set_is_gossip_for_player_2: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
    pub update_online_score_archery: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub update_online_score_chicken_kick: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub update_online_score_chapel_or_temple: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub update_online_score_fishing_compo: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub update_score_fishing_competition: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_best_time_pairs: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_best_time_sorting: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub get_best_score_blackjack: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_best_score_coin_golf_oak_vale: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_best_score_coin_golf_snow_spire: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_best_score_shove_ha_penny: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub get_best_time_guess_the_addition: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub is_hero_in_tavern_game: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub get_num_houses_owned: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub start_sneaking: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_steal_duration: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
    pub set_useable_by_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_owned_by_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_tavern_table_available_for_use: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_is_thing_turncoatable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_is_thing_force_pushable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_is_thing_lightningable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub set_is_thing_epic_spellable: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub is_thing_turncoated: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
    pub add_creature_scripted_mode: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
    pub remove_creature_scripted_mode: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub force_ships_visible: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_sleeping_position_and_orientation_from_bed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing, *mut C3DVector, *mut C3DVector) -> bool,
    pub set_bed_availability: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
    pub repopulate_village: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
    pub smash_all_windows_within_radius_of_point: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float),
    pub set_residency: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> CScriptThing,
    pub set_thanking_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
    pub get_thanking_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
    pub reset_thanking_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_ignoring_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
    pub get_ignoring_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
    pub reset_ignoring_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_wander_centre_point: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, C3DVector),
    pub get_wander_centre_point: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> C3DVector,
    pub reset_wander_centre_point: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_wander_min_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_wander_min_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_wander_min_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_wander_max_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_wander_max_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_wander_max_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_gossip_counter: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
    pub get_gossip_counter: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
    pub reset_gossip_counter: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_max_gossip_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
    pub get_max_gossip_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
    pub reset_max_gossip_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_warning_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
    pub get_warning_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
    pub reset_warning_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_beer_request_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
    pub get_beer_request_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
    pub reset_beer_request_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_scripting_state_group: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, EScriptingStateGroups),
    pub get_scripting_state_group: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> EScriptingStateGroups,
    pub reset_scripting_state_group: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_max_hero_reaction_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_max_hero_reaction_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_max_hero_reaction_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_action_frequency: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
    pub get_action_frequency: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
    pub reset_action_frequency: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_action_frequency_variation: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_action_frequency_variation: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_action_frequency_variation: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
    pub get_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
    pub reset_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_face_hero_for_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_face_hero_for_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_face_hero_for_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_target_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
    pub get_target_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
    pub reset_target_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_follow_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_follow_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_follow_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_attack_hero_on_sight: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_attack_hero_on_sight: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_attack_hero_on_sight: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_time_to_spend_harassing_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
    pub get_time_to_spend_harassing_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
    pub reset_time_to_spend_harassing_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_combat_nearby_enemy_fleeing_break_off_range: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_combat_nearby_enemy_fleeing_break_off_range: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_combat_nearby_enemy_fleeing_break_off_range: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_combat_nearby_break_off_range: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_combat_nearby_break_off_range: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_combat_nearby_break_off_range: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_steal_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_steal_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_steal_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_recover_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_recover_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_recover_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_take_stealable_item_to_random_destination: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_take_stealable_item_to_random_destination: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_take_stealable_item_to_random_destination: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_kill_self_and_stealable_item_after_reaching_destination: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_kill_self_and_stealable_item_after_reaching_destination: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_kill_self_and_stealable_item_after_reaching_destination: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_allowed_to_follow: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_allowed_to_follow: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_allowed_to_follow: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_table_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
    pub get_table_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
    pub reset_table_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_seat_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
    pub get_seat_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
    pub reset_seat_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_disable_head_looking: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_disable_head_looking: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_disable_head_looking: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_is_pushable_by_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_is_pushable_by_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_is_pushable_by_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_look_for_finite_time: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_look_for_finite_time: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_look_for_finite_time: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_avoid_region_exits: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
    pub get_avoid_region_exits: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
    pub reset_avoid_region_exits: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_targeting_distance_offset: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
    pub get_targeting_distance_offset: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
    pub reset_targeting_distance_offset: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
    pub set_player_using_melee_dummies: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_player_using_melee_dummies: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_player_using_ranged_dummies: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_player_using_ranged_dummies: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_player_using_will_dummies: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_player_using_will_dummies: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_cheap_head_looking: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_cheap_head_looking: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_quit_tavern_game: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_quit_tavern_game: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_prize_tavern_table: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_prize_tavern_table: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_betting_active: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_betting_active: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_betting_accept: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_betting_accept: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_betting_amount: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
    pub get_betting_amount: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
    pub set_count_bet_money_down: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_count_bet_money_down: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_spot_the_addition_beaten: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_spot_the_addition_beaten: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_global_targeting_distance_offset: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_global_targeting_distance_offset: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub set_trading_price_mult: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
    pub get_trading_price_mult: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
    pub set_boasting_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
    pub get_boasting_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
    pub set_active_gossip_categories: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, bool),
    pub get_active_gossip_categories: extern "thiscall" fn(*mut CGameScriptInterface) -> *const cxx::StdMap<CCharString, bool>,
    pub get_active_gossip_categories_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *mut bool,
    pub get_active_gossip_categories_size: extern "thiscall" fn(*mut CGameScriptInterface) -> i32,
    pub clear_active_gossip_categories: extern "thiscall" fn(*mut CGameScriptInterface),
    pub get_is_gossip_for_player: extern "thiscall" fn(*mut CGameScriptInterface) -> *const cxx::StdMap<CCharString, bool>,
    pub get_is_gossip_for_player_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *mut bool,
    pub get_is_gossip_for_player_size: extern "thiscall" fn(*mut CGameScriptInterface) -> i32,
    pub clear_is_gossip_for_player: extern "thiscall" fn(*mut CGameScriptInterface),
    pub set_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString, c_long),
    pub get_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *const cxx::StdVector<CCharString>,
    pub get_gossip_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, c_long) -> CCharString,
    pub get_gossip_size: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> i32,
    pub clear_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub remove_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub add_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub add_gossip_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString),
    pub set_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString, c_long),
    pub get_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *const cxx::StdVector<CCharString>,
    pub get_gossip_villages_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, c_long) -> CCharString,
    pub get_gossip_villages_size: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> i32,
    pub clear_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub remove_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub add_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub add_gossip_villages_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString),
    pub set_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString, c_long),
    pub get_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *const cxx::StdVector<CCharString>,
    pub get_gossip_factions_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, c_long) -> CCharString,
    pub get_gossip_factions_size: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> i32,
    pub clear_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub remove_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub add_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
    pub add_gossip_factions_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString),
    pub c_game_script_interface_destructor: extern "thiscall" fn(*mut CGameScriptInterface),
}

#[derive(Debug)]
#[repr(C)]
pub enum EScriptAIPriority {
	SCRIPT_AI_PRIORITY_LOWEST = 0,
	SCRIPT_AI_PRIORITY_LOW = 1,
	SCRIPT_AI_PRIORITY_MEDIUM = 2,
	SCRIPT_AI_PRIORITY_HIGH = 3,
	SCRIPT_AI_PRIORITY_HIGHEST = 4,
}

#[derive(Debug)]
#[repr(C)]
pub enum EHeroAbility {
	HERO_ABILITY_NULL = 0,
	HERO_ABILITY_FORCE_PUSH = 1,
	HERO_ABILITY_TIME_SPELL = 2,
	HERO_ABILITY_ENFLAME_SPELL = 3,
	HERO_ABILITY_PHYSICAL_SHIELD_SPELL = 4,
	HERO_ABILITY_TURNCOAT_SPELL = 5,
	HERO_ABILITY_DRAIN_LIFE_SPELL = 6,
	HERO_ABILITY_RAISE_DEAD_SPELL = 7,
	HERO_ABILITY_BERSERK = 8,
	HERO_ABILITY_DOUBLE_STRIKE = 9,
	HERO_ABILITY_SUMMON_SPELL = 10,
	HERO_ABILITY_LIGHTNING_SPELL = 11,
	HERO_ABILITY_BATTLE_CHARGE = 12,
	HERO_ABILITY_ASSASSIN_RUSH = 13,
	HERO_ABILITY_HEAL_LIFE_SPELL = 14,
	HERO_ABILITY_GHOST_SWORD_SPELL = 15,
	HERO_ABILITY_FIREBALL_SPELL = 16,
	HERO_ABILITY_MULTI_ARROW = 17,
	HERO_ABILITY_DIVINE_WRATH_SPELL = 18,
	HERO_ABILITY_UNHOLY_POWER_SPELL = 19,
	MAX_NUMBER_OF_HERO_ABILITIES = 20,
}

#[derive(Debug)]
#[repr(C)]
pub enum ETextGroupSelectionMethod {
	GROUP_SELECT_NONE = 0,
	GROUP_SELECT_RANDOM = 1,
	GROUP_SELECT_RANDOM_NO_REPEAT = 2,
	GROUP_SELECT_SEQUENTIAL = 3,
	GROUP_SELECT_FIRST_ONLY = 4,
}

#[derive(Debug)]
#[repr(C)]
pub enum ETutorialCategory {
	TUTORIAL_CATEGORY_NONE = 0,
	TUTORIAL_CATEGORY_ABILITY_ASSIGNING = 1,
	TUTORIAL_CATEGORY_ABILITY_CYCLING = 2,
	TUTORIAL_CATEGORY_BASIC_OBJECTS = 3,
	TUTORIAL_CATEGORY_BED = 4,
	TUTORIAL_CATEGORY_BOASTING = 5,
	TUTORIAL_CATEGORY_CAMERA = 6,
	TUTORIAL_CATEGORY_CHARITY_SHOP = 7,
	TUTORIAL_CATEGORY_CHEST = 8,
	TUTORIAL_CATEGORY_COMBAT_MULTIPLIER = 9,
	TUTORIAL_CATEGORY_CREATURE_DROP = 10,
	TUTORIAL_CATEGORY_DYING = 11,
	TUTORIAL_CATEGORY_DEMON_DOOR = 12,
	TUTORIAL_CATEGORY_DOOR = 13,
	TUTORIAL_CATEGORY_EXPERIENCE = 14,
	TUTORIAL_CATEGORY_EXPERIENCE_SPENDING = 15,
	TUTORIAL_CATEGORY_EXPRESSION = 16,
	TUTORIAL_CATEGORY_FLIRTING = 17,
	TUTORIAL_CATEGORY_FLOURISHING_MOVE = 18,
	TUTORIAL_CATEGORY_GOLDMARKERS = 19,
	TUTORIAL_CATEGORY_GUILD_SEAL = 20,
	TUTORIAL_CATEGORY_INTERACTING = 21,
	TUTORIAL_CATEGORY_INVENTORY = 22,
	TUTORIAL_CATEGORY_INVENTORY_ASSIGNING = 23,
	TUTORIAL_CATEGORY_LEVELLING_UP = 24,
	TUTORIAL_CATEGORY_MORALITY = 25,
	TUTORIAL_CATEGORY_MOVEMENT = 26,
	TUTORIAL_CATEGORY_QUEST = 27,
	TUTORIAL_CATEGORY_QUEST_CARD = 28,
	TUTORIAL_CATEGORY_RENOWN = 29,
	TUTORIAL_CATEGORY_TAKING_QUESTS = 30,
	TUTORIAL_CATEGORY_TELEPORTING = 31,
	TUTORIAL_CATEGORY_TRADE_ITEM = 32,
	TUTORIAL_CATEGORY_SEARCHING = 33,
	TUTORIAL_CATEGORY_SNEAK = 34,
	TUTORIAL_CATEGORY_BUILDING_OWNERSHIP = 35,
	TUTORIAL_CATEGORY_FISHING_GAME = 36,
	TUTORIAL_CATEGORY_ORACLE_GAME = 37,
	TUTORIAL_CATEGORY_WORLD_MAP = 38,
	TUTORIAL_CATEGORY_ALCOHOL = 39,
	TUTORIAL_CATEGORY_AUGMENTATION = 40,
	TUTORIAL_CATEGORY_ARMOUR = 41,
	TUTORIAL_CATEGORY_BOMB = 42,
	TUTORIAL_CATEGORY_CLOTHES = 43,
	TUTORIAL_CATEGORY_FOOD = 44,
	TUTORIAL_CATEGORY_FISHING_ROD = 45,
	TUTORIAL_CATEGORY_GIFT = 46,
	TUTORIAL_CATEGORY_HAIRSTYLE = 47,
	TUTORIAL_CATEGORY_POTION = 48,
	TUTORIAL_CATEGORY_RESURRECTION_PHIAL = 49,
	TUTORIAL_CATEGORY_SILVER_KEY = 50,
	TUTORIAL_CATEGORY_SPADE = 51,
	TUTORIAL_CATEGORY_TATTOO = 52,
	TUTORIAL_CATEGORY_TROPHY = 53,
	TUTORIAL_CATEGORY_WEAPON = 54,
	TUTORIAL_CATEGORY_WEAPON_LEGENDARY = 55,
	TUTORIAL_CATEGORY_APOLOGY = 56,
	TUTORIAL_CATEGORY_BATTLE_CRY = 57,
	TUTORIAL_CATEGORY_BELCH = 58,
	TUTORIAL_CATEGORY_EVIL_LAUGH = 59,
	TUTORIAL_CATEGORY_FART = 60,
	TUTORIAL_CATEGORY_FLIRT = 61,
	TUTORIAL_CATEGORY_FOLLOW = 62,
	TUTORIAL_CATEGORY_GIGGLE = 63,
	TUTORIAL_CATEGORY_HEROIC_STANCE = 64,
	TUTORIAL_CATEGORY_MIDDLE_FINGER = 65,
	TUTORIAL_CATEGORY_PELVIC_THRUST = 66,
	TUTORIAL_CATEGORY_PICKLOCK = 67,
	TUTORIAL_CATEGORY_PICKPOCKET = 68,
	TUTORIAL_CATEGORY_SHIT = 69,
	TUTORIAL_CATEGORY_SNEER = 70,
	TUTORIAL_CATEGORY_STEAL = 71,
	TUTORIAL_CATEGORY_THANKS = 72,
	TUTORIAL_CATEGORY_VICTORY_PUMP = 73,
	TUTORIAL_CATEGORY_WAIT = 74,
	TUTORIAL_CATEGORY_COCK_A_DOODLE_DO = 75,
	TUTORIAL_CATEGORY_CROTCH_GRAB = 76,
	TUTORIAL_CATEGORY_KISS_MY_ASS = 77,
	TUTORIAL_CATEGORY_FLAMENCO = 78,
	TUTORIAL_CATEGORY_COSSACK = 79,
	TUTORIAL_CATEGORY_AIR_GUITAR = 80,
	TUTORIAL_CATEGORY_BALLET = 81,
	TUTORIAL_CATEGORY_SATURDAY_NIGHT_FEVER = 82,
	TUTORIAL_CATEGORY_TAP = 83,
	TUTORIAL_CATEGORY_Y = 84,
	TUTORIAL_CATEGORY_M = 85,
	TUTORIAL_CATEGORY_C = 86,
	TUTORIAL_CATEGORY_A = 87,
	TUTORIAL_CATEGORY_CRIME_WEAPONOUT = 88,
	TUTORIAL_CATEGORY_CRIME_TRESPASSING = 89,
	TUTORIAL_CATEGORY_CRIME_VANDALISM = 90,
	TUTORIAL_CATEGORY_CRIME_THEFT = 91,
	TUTORIAL_CATEGORY_CRIME_ASSAULT = 92,
	TUTORIAL_CATEGORY_CRIME_GBH = 93,
	TUTORIAL_CATEGORY_CRIME_MURDER = 94,
	TUTORIAL_CATEGORY_COUNT = 95,
}

#[derive(Debug)]
#[repr(C)]
pub enum EMorality {
	MORALITY_SUPER_EVIL = 0,
	MORALITY_VERY_EVIL = 1,
	MORALITY_EVIL = 2,
	MORALITY_NEUTRAL = 3,
	MORALITY_GOOD = 4,
	MORALITY_VERY_GOOD = 5,
	MORALITY_SUPER_GOOD = 6,
}

#[derive(Debug)]
#[repr(C)]
pub enum EHeroTrainableStatType {
	HERO_STAT_STRENGTH_PHYSIQUE = 0,
	HERO_STAT_STRENGTH_HEALTH = 1,
	HERO_STAT_STRENGTH_TOUGHNESS = 2,
	HERO_STAT_SKILL_SPEED = 3,
	HERO_STAT_SKILL_ACCURACY = 4,
	HERO_STAT_SKILL_STEALTH = 5,
	HERO_STAT_WILL_WEAPON_MAGIC = 6,
	HERO_STAT_WILL_ABILITY_MAGIC = 7,
	HERO_STAT_WILL_PURE_MAGIC = 8,
	HERO_STAT_WILL_MAGIC_POWER = 9,
	NUMBER_OF_TRAINABLE_HERO_STATS = 10,
}

#[derive(Debug)]
#[repr(C)]
pub enum EHeroTitle /* stored as int32_t */ {
	TITLE_NONE = 0,
	TITLE_REAPER = 1,
	TITLE_SHADOWHUNTER = 2,
	TITLE_MALEFICUS = 3,
	TITLE_DEATHBRINGER = 4,
	TITLE_ASSASSIN = 5,
	TITLE_NECROMANCER = 6,
	TITLE_AVATAR = 7,
	TITLE_PILGRIM = 8,
	TITLE_LIBERATOR = 9,
	TITLE_PALADIN = 10,
	TITLE_DRUID = 11,
	TITLE_RANGER = 12,
	TITLE_RUNEMASTER = 13,
	TITLE_HOOD = 14,
	TITLE_GLADIATOR = 15,
	TITLE_SABRE = 16,
	TITLE_ARROWDODGER = 17,
	TITLE_PIEMASTER = 18,
	TITLE_CHICKEN_CHASER = 19,
	TITLE_ARSEFACE = 20,
	TITLE_JACK = 21,
	TITLE_MAZE = 22,
	TITLE_SCARLET_ROBE = 23,
	TITLE_SCYTHE = 24,
	TITLE_THUNDER = 25,
	TITLE_WHISPER = 26,
	TITLE_TWINBLADE = 27,
	TITLE_BRIAR_ROSE = 28,
	TITLE_LADY_GREY = 29,
	TITLE_GUILDMASTER = 30,
	TITLE_SCORPION_SLAYER = 31,
	TITLE_DEATH_BRINGER = 32,
}

#[derive(Debug)]
#[repr(C)]
pub enum EGameAction {
	GAME_ACTION_NULL = 0,
	GAME_ACTION_LOCK_TARGET = 1,
	GAME_ACTION_OPEN_INVENTORY = 2,
	GAME_ACTION_OPEN_IN_GAME_MENU = 3,
	GAME_ACTION_TOGGLE_MINI_MAP = 4,
	GAME_ACTION_PAUSE = 5,
	GAME_ACTION_INTERACT = 6,
	GAME_ACTION_BLOCK = 7,
	GAME_ACTION_SPECIAL_ATTACK = 8,
	GAME_ACTION_ATTACK = 9,
	GAME_ACTION_FIRE_RANGED_WEAPON = 10,
	GAME_ACTION_UNARMED_ATTACK = 11,
	GAME_ACTION_ARMED_MELEE_ATTACK = 12,
	GAME_ACTION_UNSHEATHE_MELEE_WEAPON = 13,
	GAME_ACTION_UNSHEATHE_RANGED_WEAPON = 14,
	GAME_ACTION_SHEATHE_MELEE_WEAPON = 15,
	GAME_ACTION_SHEATHE_RANGED_WEAPON = 16,
	GAME_ACTION_TOGGLE_SILENT_MOVE = 17,
	GAME_ACTION_TOGGLE_CINEMATIC_AND_USER_CAMERA = 18,
	GAME_ACTION_TOGGLE_FIRST_PERSON_VIEW = 19,
	GAME_ACTION_SKIP_PAST_TEXT = 20,
	GAME_ACTION_SKIP_CUT_SCENE = 21,
	GAME_ACTION_ANSWER_QUESTION_YES = 22,
	GAME_ACTION_ANSWER_QUESTION_NO = 23,
	GAME_ACTION_ANSWER_QUESTION_THIRD = 24,
	GAME_ACTION_INVENTORY_SELECT = 25,
	GAME_ACTION_ATTRACT_EXPERIENCE_ORBS = 26,
	GAME_ACTION_ROTATE_GUI_SCREENS_LEFT = 27,
	GAME_ACTION_ROTATE_GUI_SCREENS_RIGHT = 28,
	GAME_ACTION_JUMP = 29,
	GAME_ACTION_SPRINT = 30,
	GAME_ACTION_RUN = 31,
	GAME_ACTION_SNEAK = 32,
	GAME_ACTION_INVENTORY_A = 33,
	GAME_ACTION_INVENTORY_B = 34,
	GAME_ACTION_INVENTORY_X = 35,
	GAME_ACTION_INVENTORY_Y = 36,
	GAME_ACTION_INVENTORY_UP = 37,
	GAME_ACTION_INVENTORY_DOWN = 38,
	GAME_ACTION_INVENTORY_LEFT = 39,
	GAME_ACTION_INVENTORY_RIGHT = 40,
	GAME_ACTION_INVENTORY_WHITE = 41,
	GAME_ACTION_TAVERN_GAMES_INSTRUCTIONS = 42,
	GAME_ACTION_FISHING_REEL_IN = 43,
	GAME_ACTION_FISHING_CANCEL = 44,
	GAME_ACTION_TOGGLE_FIRST_PERSON_TARGETING = 45,
	GAME_ACTION_FIRST_PERSON_TARGET_LOCK = 46,
	GAME_ACTION_FIRST_PERSON_ZOOM_IN = 47,
	GAME_ACTION_GENERAL_LEAVE_PLAYER_MODE = 48,
	GAME_ACTION_DEBUG_JUMP_1 = 49,
	GAME_ACTION_DEBUG_JUMP_2 = 50,
	GAME_ACTION_DEBUG_CAMERA = 51,
	GAME_ACTION_DEBUG_SHIFT = 52,
	GAME_ACTION_TAKE_PHOTO_FOR_PHOTOJOURNAL = 53,
	GAME_ACTION_ASSIGNABLE_SPECIAL_MOVE = 54,
	GAME_ACTION_QUICK_ACCESS_ITEM = 55,
	GAME_ACTION_CONTEXT_SENSITIVE_ITEM = 56,
	GAME_ACTION_CYCLE_THROUGH_SPELLS = 57,
	GAME_ACTION_COIN_GOLF_CANCEL_AIM = 58,
	GAME_ACTION_CONFIRM_RESET_TO_FRONT_END = 59,
	GAME_ACTION_MOVEMENT = 60,
	GAME_ACTION_CAMERA_ROTATE = 61,
	GAME_ACTION_CAMERA_ROTATE_LEFT = 62,
	GAME_ACTION_CAMERA_ROTATE_RIGHT = 63,
	GAME_ACTION_CAMERA_ZOOM_IN = 64,
	GAME_ACTION_CAMERA_ZOOM_OUT = 65,
	GAME_ACTION_ORACLE_MINIGAME_UP = 66,
	GAME_ACTION_ORACLE_MINIGAME_DOWN = 67,
	GAME_ACTION_ORACLE_MINIGAME_LEFT = 68,
	GAME_ACTION_ORACLE_MINIGAME_RIGHT = 69,
	GAME_ACTION_ORACLE_MINIGAME_QUIT = 70,
	GAME_ACTION_MOVE_MOUSE_ON_GUI = 71,
	GAME_ACTION_TOGGLE_LIVE_GUI = 72,
	GAME_ACTION_OPEN_EXPRESSION_MENU = 73,
	GAME_ACTION_TOGGLE_DEACTIVATE_LOCK_TARGET = 74,
	GAME_ACTION_FIRST_PERSON_LOOKAROUND = 75,
	GAME_ACTION_INVENTORY_UNSELECT = 76,
	GAME_ACTION_CAMERA_MOVE_DOUBLE_AXIS = 77,
	GAME_ACTION_CHARGE_GUILD_SEAL = 78,
	GAME_ACTION_TAVERN_GAME_MOVEMENT = 79,
	GAME_ACTION_TAVERN_GAME_CAMERA = 80,
	GAME_ACTION_TAVERN_GAME_ACTION_BUTTON = 81,
	GAME_ACTION_TAVERN_GAME_ALTERNATE_BUTTON = 82,
	GAME_ACTION_TAVERN_GAME_QUIT = 83,
	GAME_ACTION_PROJECTILE_TARGETING_ANALOGUE_ZOOM = 84,
	GAME_ACTION_TOGGLE_PASSIVE_AGGRESSIVE_MODE = 85,
	GAME_ACTION_ACTIVATE_SPELL_MODE = 86,
	GAME_ACTION_EXPRESSION_SHIFT = 87,
	GAME_ACTION_SCROLL_DESCRIPTION_UP = 88,
	GAME_ACTION_SCROLL_DESCRIPTION_DOWN = 89,
	GAME_ACTION_OPEN_WEAPONS_MENU = 90,
	GAME_ACTION_OPEN_CLOTHING_MENU = 91,
	GAME_ACTION_OPEN_ITEMS_MENU = 92,
	GAME_ACTION_OPEN_CURRENT_QUESTS_MENU = 93,
	GAME_ACTION_CYCLE_THROUGH_SPELLS_KEYBOARD = 94,
	GAME_ACTION_TOGGLE_KILL_EVERYTHING_MODE = 95,
	GAME_ACTION_OPEN_MAGIC_MENU = 96,
	GAME_ACTION_OPEN_EXPRESSIONS_MENU = 97,
	GAME_ACTION_OPEN_PERSONALITY_MENU = 98,
	GAME_ACTION_OPEN_LOGBOOK_MENU = 99,
	GAME_ACTION_OPEN_MAP_MENU = 100,
	GAME_ACTION_SCROLL_MENU = 101,
	GAME_ACTION_ZOOM_MAP_OUT = 102,
	GAME_ACTION_ZOOM_MAP_IN = 103,
	GAME_ACTION_SCROLL_MAP_LEFT = 104,
	GAME_ACTION_SCROLL_MAP_RIGHT = 105,
	GAME_ACTION_SCROLL_MAP_DOWN = 106,
	GAME_ACTION_SCROLL_MAP_UP = 107,
	GAME_ACTION_OPTIONS_SLIDER_LEFT = 108,
	GAME_ACTION_OPTIONS_SLIDER_RIGHT = 109,
	GAME_ACTION_DIGITAL_ANALOGUE_ZOOM_IN = 110,
	GAME_ACTION_DIGITAL_ANALOGUE_ZOOM_OUT = 111,
	GAME_ACTION_TOGGLE_VIEW_HERO_MODE = 112,
	GAME_ACTION_CENTRE_CAMERA = 113,
	GAME_ACTION_BETTING = 114,
	GAME_ACTION_COUNT = 115,
}

#[derive(Debug)]
#[repr(C)]
pub enum EMiniGameType {
	MINIGAME_NONE = 0,
	MINIGAME_FISHING = 1,
	MINIGAME_DIGGING = 2,
	MINIGAME_PICKPOCKET = 3,
	MINIGAME_PICKLOCK = 4,
	MINIGAME_STEAL = 5,
	MINIGAME_TROPHY = 6,
	MINIGAME_ORACLE = 7,
}

#[derive(Debug)]
#[repr(C)]
pub enum EDoorTriggerType {
	DOOR_TRIGGER_ON_PERSON = 0,
	DOOR_TRIGGER_MANUAL = 1,
}

#[derive(Debug)]
#[repr(C)]
pub enum ECrime {
	CRIME_NONE = 0,
	CRIME_WEAPON_OUT = 1,
	CRIME_TRESPASS = 2,
	CRIME_VANDALISM = 3,
	CRIME_LOCKPICKING = 4,
	CRIME_PICK_POCKETS = 5,
	CRIME_THEFT = 6,
	CRIME_ASSAULT = 7,
	CRIME_GBH = 8,
	CRIME_MURDER = 9,
	CRIME_LAST = 10,
}

#[derive(Debug)]
#[repr(C)]
pub enum ETargetingType {
	TARGETING_NULL = 0,
	TARGETING_USABLE = 1,
	TARGETING_TALKABLE = 2,
	TARGETING_STAB = 4,
	TARGETING_SHOOTABLE = 8,
	TARGETING_MELEE = 16,
	TARGETING_ZTARGETING = 32,
}

#[derive(Debug)]
#[repr(C)]
pub enum ESex {
	SEX_NULL = 0,
	SEX_MALE = 1,
	SEX_FEMALE = 2,
	NO_OF_SEXES = 3,
}

#[derive(Debug)]
#[repr(C)]
pub enum EOpinion {
	OPINION_MORALITY = 0,
	OPINION_RENOWN = 1,
	OPINION_SCARINESS = 2,
	OPINION_AGREEABLENESS = 3,
	OPINION_ATTRACTIVENESS = 4,
	OPINION_LAST = 5,
}

#[derive(Debug)]
#[repr(C)]
pub enum EOpinionDeedType {
	OPINION_DEED_TYPE_NONE = 0,
	OPINION_DEED_TYPE_CRIME_WEAPON_OUT = 1,
	OPINION_DEED_TYPE_CRIME_TRESPASS_THIRD = 2,
	OPINION_DEED_TYPE_CRIME_VANDALISM = 3,
	OPINION_DEED_TYPE_CRIME_LOCKPICKING = 4,
	OPINION_DEED_TYPE_CRIME_PICK_POCKETS = 5,
	OPINION_DEED_TYPE_CRIME_THEFT = 6,
	OPINION_DEED_TYPE_CRIME_ASSAULT = 7,
	OPINION_DEED_TYPE_CRIME_GBH = 8,
	OPINION_DEED_TYPE_CRIME_MURDER = 9,
	OPINION_DEED_TYPE_CRIME_TRESPASS_FIRST = 10,
	OPINION_DEED_TYPE_CRIME_TRESPASS_SECOND = 11,
	OPINION_DEED_TYPE_EXPRESSION_HEROIC_STANCE = 12,
	OPINION_DEED_TYPE_EXPRESSION_FLIRT = 13,
	OPINION_DEED_TYPE_EXPRESSION_APOLOGY_NO_CRIME = 14,
	OPINION_DEED_TYPE_EXPRESSION_SNEER = 15,
	OPINION_DEED_TYPE_EXPRESSION_EVIL_LAUGH = 16,
	OPINION_DEED_TYPE_EXPRESSION_BATTLE_CRY = 17,
	OPINION_DEED_TYPE_EXPRESSION_PELVIC_THRUST = 18,
	OPINION_DEED_TYPE_EXPRESSION_MIDDLE_FINGER = 19,
	OPINION_DEED_TYPE_EXPRESSION_BELCH = 20,
	OPINION_DEED_TYPE_EXPRESSION_FART = 21,
	OPINION_DEED_TYPE_EXPRESSION_VICTORY_PUMP = 22,
	OPINION_DEED_TYPE_EXPRESSION_CLAP = 23,
	OPINION_DEED_TYPE_EXPRESSION_GIGGLE = 24,
	OPINION_DEED_TYPE_EXPRESSION_SHIT = 25,
	OPINION_DEED_TYPE_EXPRESSION_THANKS = 26,
	OPINION_DEED_TYPE_EXPRESSION_COCK_A_DOODLE_DO = 27,
	OPINION_DEED_TYPE_EXPRESSION_CROTCH_GRAB = 28,
	OPINION_DEED_TYPE_EXPRESSION_KISS_MY_ASS = 29,
	OPINION_DEED_TYPE_EXPRESSION_FLAMENCO = 30,
	OPINION_DEED_TYPE_EXPRESSION_COSSACK = 31,
	OPINION_DEED_TYPE_EXPRESSION_AIR_GUITAR = 32,
	OPINION_DEED_TYPE_EXPRESSION_BALLET = 33,
	OPINION_DEED_TYPE_EXPRESSION_SATURDAY_NIGHT_FEVER = 34,
	OPINION_DEED_TYPE_EXPRESSION_TAP = 35,
	OPINION_DEED_TYPE_EXPRESSION_Y = 36,
	OPINION_DEED_TYPE_EXPRESSION_M = 37,
	OPINION_DEED_TYPE_EXPRESSION_C = 38,
	OPINION_DEED_TYPE_EXPRESSION_A = 39,
	OPINION_DEED_TYPE_EXPRESSION_THREATEN_SMALL = 40,
	OPINION_DEED_TYPE_EXPRESSION_THREATEN_LARGE = 41,
	OPINION_DEED_TYPE_SCRIPT_ACTION_ANNOYING_SMALL = 42,
	OPINION_DEED_TYPE_SCRIPT_ACTION_ANNOYING_LARGE = 43,
	OPINION_DEED_TYPE_SCRIPT_ACTION_NICE_SMALL = 44,
	OPINION_DEED_TYPE_SCRIPT_ACTION_NICE_LARGE = 45,
	OPINION_DEED_TYPE_FOLLOWER_ACCEPT = 46,
	OPINION_DEED_TYPE_FOLLOWER_REFUSE = 47,
	OPINION_DEED_TYPE_FOLLOWER_DISMISSED = 48,
	OPINION_DEED_TYPE_FOLLOWER_QUIT = 49,
	OPINION_DEED_TYPE_FOLLOWER_ENEMYSPOTTED = 50,
	OPINION_DEED_TYPE_FOLLOWER_IDLEEXCITED = 51,
	OPINION_DEED_TYPE_FOLLOWER_IDLEBORED = 52,
	OPINION_DEED_TYPE_WITNESSED_ASSAULT_OR_GBH = 53,
	OPINION_DEED_TYPE_TOO_FREQUENT_OTHER_DEED = 54,
	OPINION_DEED_TYPE_SHOW_TROPHY_EVIL = 55,
	OPINION_DEED_TYPE_SHOW_TROPHY_GOOD = 56,
	OPINION_DEED_TYPE_SHOW_TROPHY_BORED = 57,
	OPINION_DEED_TYPE_KILL_BAD_GUY = 58,
	OPINION_DEED_TYPE_MURDER_SPOUSE = 59,
	OPINION_DEED_TYPE_RECEIVE_GIFT_ROMANTIC = 60,
	OPINION_DEED_TYPE_RECEIVE_GIFT_FRIENDLY = 61,
	OPINION_DEED_TYPE_RECEIVE_GIFT_OFFENSIVE = 62,
	OPINION_DEED_TYPE_RECEIVE_WEDDING_RING = 63,
	OPINION_DEED_TYPE_BOAST_ANTICIPATION = 64,
	OPINION_DEED_TYPE_BOAST_ENCOURAGE_FIRST = 65,
	OPINION_DEED_TYPE_BOAST_ENCOURAGE_MIDDLE = 66,
	OPINION_DEED_TYPE_BOAST_ENCOURAGE_FINAL = 67,
	OPINION_DEED_TYPE_BOAST_WELL_WISHES = 68,
	OPINION_DEED_TYPE_BOAST_ANNOYED_NO_BOASTING = 69,
	OPINION_DEED_TYPE_COMMENT_AT_HERO = 70,
	OPINION_DEED_TYPE_COMMENT_TO_SELF = 71,
	OPINION_DEED_TYPE_COMMENT_ABOUT_HERO = 72,
	OPINION_DEED_TYPE_GENERIC_INCOMPREHENSION = 73,
	OPINION_DEED_TYPE_HIGH_PRIORITY_INCOMPREHENSION = 74,
	OPINION_DEED_TYPE_HUSBAND_RAGE = 75,
	OPINION_DEED_TYPE_TAVERN_GAME_WIN = 76,
	OPINION_DEED_TYPE_INDOORS_GREETING = 77,
	OPINION_DEED_TYPE_APOLOGY_ACCEPTED = 78,
	OPINION_DEED_TYPE_APOLOGY_REFUSED = 79,
	OPINION_DEED_TYPE_WIFE_GREETED = 80,
	OPINION_DEED_TYPE_WIFE_TIME_SINCE_SEEING = 81,
	OPINION_DEED_TYPE_WIFE_GIFT_RECEIVE_ALREADY_GOT = 82,
	OPINION_DEED_TYPE_WIFE_JUSTMARRIED = 83,
	OPINION_DEED_TYPE_WIFE_GIFT_WANTED = 84,
	OPINION_DEED_TYPE_WIFE_WITNESSED_FLIRT = 85,
	OPINION_DEED_TYPE_WIFE_HOUSE_DRESSING_GOOD = 86,
	OPINION_DEED_TYPE_WIFE_HOUSE_DRESSING_BAD = 87,
	OPINION_DEED_TYPE_WIFE_DIVORCE_WARNING = 88,
	OPINION_DEED_TYPE_WIFE_DIVORCE_OCCURRED = 89,
	OPINION_DEED_TYPE_WIFE_SEX_OFFER_TO_GO_TO_BED = 90,
	OPINION_DEED_TYPE_WIFE_SEX_COMMENT_AFTERWARDS = 91,
	OPINION_DEED_TYPE_LAST = 92,
}

#[derive(Debug)]
#[repr(C)]
pub enum EScriptingStateGroups {
	ESSG_NONE = 0,
	ESSG_PERFORM_ACTION_PHYSICAL = 1,
	ESSG_PERFORM_ACTION_VERBAL = 2,
	ESSG_PERFORM_ACTION_AURAL = 3,
	ESSG_WANDER_NEAR = 4,
	ESSG_FOLLOW_PATH = 5,
	ESSG_FOLLOW_RANDOM = 6,
	ESSG_FOLLOW_NEAREST = 7,
	ESSG_WALK_TO_RANDOM = 8,
	ESSG_WALK_TO_NEAREST_DIFFERENT = 9,
	ESSG_RUN_AT_HERO_AND_ATTACK_UNTIL_DEAD = 10,
}

/// Unknown variants.
#[derive(Debug)]
#[repr(C)]
pub enum ECameraOp {
    UNKNOWN = 0,
}

/// Unknown variants.
#[derive(Debug)]
#[repr(C)]
pub enum EScriptVillageAttitude {
    UNKNOWN = 0,
}

#[derive(Debug)]
#[repr(C)]
pub enum EOpinionPostDeedType {
	OPINION_POST_DEED_TYPE_NULL = 0,
	OPINION_POST_DEED_TYPE_RECIPIENT = 1,
	OPINION_POST_DEED_TYPE_WITNESSES = 2,
	OPINION_POST_DEED_TYPE_VILLAGE = 4,
	OPINION_POST_DEED_TYPE_GUARDS = 16,
	OPINION_POST_DEED_TYPE_RECIPIENT_AND_WITNESSES = 3,
	OPINION_POST_DEED_TYPE_ALL = 7,
}

/// Unknown variants.
#[derive(Debug)]
#[repr(C)]
pub enum ECutsceneBehaviour {
    UNKNOWN = 0
}

/// Unknown variants.
#[derive(Debug)]
#[repr(C)]
pub enum ECreatureGroup {
    UNKNOWN = 0
}

#[derive(Debug)]
#[repr(C)]
pub enum EOpinionAttitudeType {
	OPINION_ATTITUDE_TYPE_NONE = 0,
	OPINION_ATTITUDE_TYPE_RESPECT = 1,
	OPINION_ATTITUDE_TYPE_AWE = 2,
	OPINION_ATTITUDE_TYPE_DISDAIN = 3,
	OPINION_ATTITUDE_TYPE_FEAR = 4,
	OPINION_ATTITUDE_TYPE_FRIENDLINESS = 5,
	OPINION_ATTITUDE_TYPE_WORSHIP = 6,
	OPINION_ATTITUDE_TYPE_RIDICULOUS = 7,
	OPINION_ATTITUDE_TYPE_OFFENSIVE = 8,
	OPINION_ATTITUDE_TYPE_AGREEABLE = 9,
	OPINION_ATTITUDE_TYPE_UGLY = 10,
	OPINION_ATTITUDE_TYPE_ATTRACTED = 11,
	OPINION_ATTITUDE_TYPE_LOVE = 12,
	// OPINION_ATTITUDE_TYPE_WIFE_FIRST = 13,
	OPINION_ATTITUDE_TYPE_WIFE_LOVE = 13,
	OPINION_ATTITUDE_TYPE_WIFE_LIKE = 14,
	OPINION_ATTITUDE_TYPE_WIFE_NEUTRAL = 15,
	OPINION_ATTITUDE_TYPE_WIFE_DISLIKE = 16,
	OPINION_ATTITUDE_TYPE_WIFE_HATE = 17,
	OPINION_ATTITUDE_TYPE_LAST = 18,
}

#[derive(Debug)]
#[repr(C)]
pub enum EOpinionReactionType {
	OPINION_REACTION_TYPE_NONE = 0,
	OPINION_REACTION_TYPE_ATTITUDE_RESPECT = 1,
	OPINION_REACTION_TYPE_ATTITUDE_AWE = 2,
	OPINION_REACTION_TYPE_ATTITUDE_DISDAIN = 3,
	OPINION_REACTION_TYPE_ATTITUDE_FEAR = 4,
	OPINION_REACTION_TYPE_ATTITUDE_FRIENDLINESS = 5,
	OPINION_REACTION_TYPE_ATTITUDE_WORSHIP = 6,
	OPINION_REACTION_TYPE_ATTITUDE_RIDICULOUS = 7,
	OPINION_REACTION_TYPE_ATTITUDE_OFFENSIVE = 8,
	OPINION_REACTION_TYPE_ATTITUDE_AGREEABLE = 9,
	OPINION_REACTION_TYPE_ATTITUDE_UGLY = 10,
	OPINION_REACTION_TYPE_ATTITUDE_ATTRACTED = 11,
	OPINION_REACTION_TYPE_ATTITUDE_LOVE = 12,
	OPINION_REACTION_TYPE_ATTITUDE_WIFE_LOVE = 13,
	OPINION_REACTION_TYPE_ATTITUDE_WIFE_LIKE = 14,
	OPINION_REACTION_TYPE_ATTITUDE_WIFE_NEUTRAL = 15,
	OPINION_REACTION_TYPE_ATTITUDE_WIFE_DISLIKE = 16,
	OPINION_REACTION_TYPE_ATTITUDE_WIFE_HATE = 17,
	OPINION_REACTION_TYPE_ANGRY_POINT = 18,
	OPINION_REACTION_TYPE_BACK_AWAY = 19,
	OPINION_REACTION_TYPE_BELLY_LAUGH = 20,
	OPINION_REACTION_TYPE_BOO = 21,
	OPINION_REACTION_TYPE_BOWING_LARGE = 22,
	OPINION_REACTION_TYPE_BOWING_SMALL = 23,
	OPINION_REACTION_TYPE_CALLING_CHILDREN = 24,
	OPINION_REACTION_TYPE_CHEER_LARGE = 25,
	OPINION_REACTION_TYPE_CHEER_SMALL = 26,
	OPINION_REACTION_TYPE_CLAP_LARGE = 27,
	OPINION_REACTION_TYPE_CLAP_SMALL = 28,
	OPINION_REACTION_TYPE_COMMENT_ABOUT_HERO = 29,
	OPINION_REACTION_TYPE_COMMENT_AT_HERO = 30,
	OPINION_REACTION_TYPE_COMMENT_TO_SELF = 31,
	OPINION_REACTION_TYPE_COWER = 32,
	OPINION_REACTION_TYPE_DISMISS = 33,
	OPINION_REACTION_TYPE_FLEE = 34,
	OPINION_REACTION_TYPE_FOLLOW_CLOSE = 35,
	OPINION_REACTION_TYPE_FOLLOW_FAR = 36,
	OPINION_REACTION_TYPE_FRIENDLY_GREET = 37,
	OPINION_REACTION_TYPE_GET_OUT = 38,
	OPINION_REACTION_TYPE_GROVEL_LARGE = 39,
	OPINION_REACTION_TYPE_GROVEL_SMALL = 40,
	OPINION_REACTION_TYPE_HERO_IMITATION_PLAY = 41,
	OPINION_REACTION_TYPE_HEROTITLE_AT_HERO = 42,
	OPINION_REACTION_TYPE_HEROTITLE_TO_SELF = 43,
	OPINION_REACTION_TYPE_HIDE = 44,
	OPINION_REACTION_TYPE_MARRIAGE_COMMENT = 45,
	OPINION_REACTION_TYPE_OFFER_GIFT_FRIENDLY = 46,
	OPINION_REACTION_TYPE_OFFER_GIFT_WORSHIP = 47,
	OPINION_REACTION_TYPE_PICK_FIGHT = 48,
	OPINION_REACTION_TYPE_POINT = 49,
	OPINION_REACTION_TYPE_POINT_LAUGH = 50,
	OPINION_REACTION_TYPE_RESPECTFUL_GREET = 51,
	OPINION_REACTION_TYPE_SHAKE_FIST = 52,
	OPINION_REACTION_TYPE_SHOUT_AWE = 53,
	OPINION_REACTION_TYPE_SNIGGER = 54,
	OPINION_REACTION_TYPE_THUMBS_DOWN = 55,
	OPINION_REACTION_TYPE_WATCH = 56,
	// OPINION_REACTION_TYPE_WIFE_FIRST = 57,
	OPINION_REACTION_TYPE_WIFE_FEELING_LOVE = 57,
	OPINION_REACTION_TYPE_WIFE_GENERAL_LOVE = 58,
	OPINION_REACTION_TYPE_WIFE_TOHERSELF_LOVE = 59,
	OPINION_REACTION_TYPE_WIFE_CLOTHING_LOVE = 60,
	OPINION_REACTION_TYPE_WIFE_FEELING_LIKE = 61,
	OPINION_REACTION_TYPE_WIFE_GENERAL_LIKE = 62,
	OPINION_REACTION_TYPE_WIFE_TOHERSELF_LIKE = 63,
	OPINION_REACTION_TYPE_WIFE_CLOTHING_LIKE = 64,
	OPINION_REACTION_TYPE_WIFE_FEELING_NEUTRAL = 65,
	OPINION_REACTION_TYPE_WIFE_GENERAL_NEUTRAL = 66,
	OPINION_REACTION_TYPE_WIFE_TOHERSELF_NEUTRAL = 67,
	OPINION_REACTION_TYPE_WIFE_CLOTHING_NEUTRAL = 68,
	OPINION_REACTION_TYPE_WIFE_FEELING_DISLIKE = 69,
	OPINION_REACTION_TYPE_WIFE_GENERAL_DISLIKE = 70,
	OPINION_REACTION_TYPE_WIFE_TOHERSELF_DISLIKE = 71,
	OPINION_REACTION_TYPE_WIFE_CLOTHING_DISLIKE = 72,
	OPINION_REACTION_TYPE_WIFE_FEELING_HATE = 73,
	OPINION_REACTION_TYPE_WIFE_GENERAL_HATE = 74,
	OPINION_REACTION_TYPE_WIFE_TOHERSELF_HATE = 75,
	OPINION_REACTION_TYPE_WIFE_CLOTHING_HATE = 76,
	OPINION_REACTION_TYPE_WIFE_JUSTMARRIED = 77,
	OPINION_REACTION_TYPE_WIFE_GIFT_WANTED = 78,
	// OPINION_REACTION_TYPE_WIFE_LAST = 79,
	OPINION_REACTION_TYPE_LAST = 79,
}

#[derive(Debug)]
#[repr(C)]
pub enum EMusicSetType {
	MUSIC_SET_NULL = 1,
	MUSIC_SET_FRESCO_WEDDING = 2,
	MUSIC_SET_CHAPEL_OF_EVIL = 3,
	MUSIC_SET_GUILD = 4,
	MUSIC_SET_GUILD_DAY = 5,
	MUSIC_SET_GUILD_NIGHT = 6,
	MUSIC_SET_HALL_OF_HEROES = 7,
	MUSIC_SET_TEMPLE_OF_LIGHT = 8,
	MUSIC_SET_ARENA_FANFARE = 9,
	MUSIC_SET_BANDIT_CAMP = 10,
	MUSIC_SET_BOWERSTONE = 11,
	MUSIC_SET_CAVES = 12,
	MUSIC_SET_DARKWOOD = 13,
	MUSIC_SET_GRAVEYARD = 14,
	MUSIC_SET_GRAVEYARD_PASSAGE = 15,
	MUSIC_SET_GREATWOOD = 16,
	MUSIC_SET_HAUNTED_HOUSE = 17,
	MUSIC_SET_LOOKOUT_POINT = 18,
	MUSIC_SET_OAKVALE = 19,
	MUSIC_SET_WITCHWOOD = 20,
	MUSIC_SET_QUEST_SUCCEEDED = 21,
	MUSIC_SET_QUEST_FAILED = 22,
	MUSIC_SET_BOSS = 23,
	MUSIC_SET_DANGER = 24,
	MUSIC_SET_PRISON = 25,
	MUSIC_SET_HOOK_COAST = 26,
	MUSIC_SET_KNOTHOLE_GLADE = 27,
	MUSIC_SET_EXECUTION_TREE = 28,
	MUSIC_SET_GIBBET_WOODS = 29,
	MUSIC_SET_KRAKEN_CHAMBER = 30,
	MUSIC_SET_INTERLUDE = 31,
	MUSIC_SET_DANGER_ONLY = 32,
	MUSIC_SET_DRAGON = 33,
	MUSIC_SET_ARENA_FIGHT = 34,
	MUSIC_SET_ARENA_FANFARE_01 = 35,
	MUSIC_SET_ARENA_FANFARE_02 = 36,
	MUSIC_SET_ARENA_FANFARE_03 = 37,
	MUSIC_SET_ARENA_FANFARE_04 = 38,
	MUSIC_SET_ARENA_FANFARE_05 = 39,
	MUSIC_SET_ARENA_FANFARE_06 = 40,
	MUSIC_SET_ARENA_FANFARE_07 = 41,
	MUSIC_SET_ARENA_FANFARE_08 = 42,
	MUSIC_SET_ARENA_FANFARE_09 = 43,
	MUSIC_SET_ARENA_FANFARE_10 = 44,
	MUSIC_SET_CUTSCENE_DEAD_DAD = 45,
	MUSIC_SET_CUTSCENE_FEET = 46,
	MUSIC_SET_CUTSCENE_GUILD_CEREMONY = 47,
	MUSIC_SET_CUTSCENE_TWINBLADE = 48,
	MUSIC_SET_CUTSCENE_THERESA_01 = 49,
	MUSIC_SET_CUTSCENE_THERESA_02 = 50,
	MUSIC_SET_CUTSCENE_WIZARD_BATTLE_INTRO = 51,
	MUSIC_SET_CUTSCENE_WIZARD_BATTLE_OUTRO = 52,
	MUSIC_SET_CUTSCENE_PRISON_MOTHER = 53,
	MUSIC_SET_CUTSCENE_JACK_BOSS_FIGHT_INTRO = 54,
	MUSIC_SET_CUTSCENE_JACK_BOSS_FIGHT_OUTRO = 55,
	MUSIC_SET_CUTSCENE_JACK_CAPTURES = 56,
	MUSIC_SET_CUTSCENE_GUILD_ARRIVAL = 57,
	MUSIC_SET_CUTSCENE_COLLECT_FIREHEART = 58,
	MUSIC_SET_CUTSCENE_DRAGON_DEATH = 59,
	MUSIC_SET_CUTSCENE_DRAGON_FIGHT_INTRO = 60,
	MUSIC_SET_CUTSCENE_DRAGON_FIGHT_OUTRO_EVIL = 61,
	MUSIC_SET_CUTSCENE_DRAGON_FIGHT_OUTRO_GOOD = 62,
	MUSIC_SET_CUTSCENE_DRAGON_FIGHT_OUTRO_CHOICE = 63,
	MUSIC_SET_CUTSCENE_ONE_YEAR_LATER = 64,
	MUSIC_SET_CUTSCENE_ORACLE_AWAKENS = 65,
	MUSIC_SET_CUTSCENE_SCYTHE_MESSAGE = 66,
	MUSIC_SET_CUTSCENE_SOUL2_MOTHER_INTRO = 67,
	MUSIC_SET_CUTSCENE_SOUL2_MOTHER_SUCCESS = 68,
	MUSIC_SET_CUTSCENE_SOUL3_GUILDMASTER_INTRO = 69,
	MUSIC_SET_CUTSCENE_SUMMON_SHIP_INTRO = 70,
	MUSIC_SET_CUTSCENE_SUMMON_SHIP_OUTRO = 71,
	MUSIC_SET_CUTSCENE_SOUL3_GUILDMASTER_OUTRO_EVIL = 72,
	MUSIC_SET_CUTSCENE_SOUL3_GUILDMASTER_OUTRO_GOOD = 73,
	MUSIC_SET_CUTSCENE_GATE_OPENS_BRIAR_ROSE = 74,
	MUSIC_SET_CUTSCENE_GATE_OPENS_SCYTHE = 75,
	MUSIC_SET_INTRO = 76,
	NO_MUSIC_SETS = 77,
}

#[derive(Debug)]
#[repr(C)]
pub enum ECategory {
	CATEGORY_QUEST = 0,
	CATEGORY_STORY = 1,
	CATEGORY_TUTORIAL = 2,
	CATEGORY_BASICS = 3,
	CATEGORY_OBJECTS = 4,
	CATEGORY_TOWNS = 5,
	CATEGORY_HERO = 6,
	CATEGORY_COMBAT = 7,
}

#[derive(Debug)]
#[repr(C)]
pub enum EClothingCoversArea {
	COVERS_BODY_AREA_NULL = 0,
	COVERS_BODY_AREA_FEET = 1,
	COVERS_BODY_AREA_LEGS = 2,
	COVERS_BODY_AREA_ARSE = 4,
	COVERS_BODY_AREA_BODY = 8,
	COVERS_BODY_AREA_HEAD = 16,
	COVERS_BODY_AREA_ARMS = 32,
	COVERS_BODY_AREA_HANDS = 64,
	COVERS_BODY_AREA_FACE = 128,
	COVERS_BODY_AREA_MOUSTACHE = 256,
	COVERS_BODY_AREA_HORN = 512,
}
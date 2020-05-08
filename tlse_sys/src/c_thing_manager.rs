#[derive(Debug)]
#[repr(C)]
pub struct CThingManager {
    // pub c_class_factory: CClassFactory,
    // pub vmt: *mut (),
    // pub component: *const CMainGameComponent,
    // pub definition_manager: *const CGameDefinitionManager,
    // pub world: *mut CWorld,
    // pub world_map: *mut CWorldMap,
    // pub world_seed: c_ulong,
    // pub player_manager: *mut CPlayerManager,
    // pub navigation_manager: *mut CNavigatorManager,
    // pub p_environment: *const CEnvironment,
    // pub thing_type_info: cxx::StdVector<CThingTypeInfo>,
    // pub dead_thing_list: cxx::StdVector<*mut CThing>,
    // pub non_scripted_entities_pause_mode: bool,
    // pub all_entities_pause_mode: bool,
    // pub first_render_of_frame: bool,
    // pub uid_to_thing_map: cxx::StdMap<u64, *mut CThing, cxx::StdLess<u64>>,
    // pub current_unique_id: u64,
    // pub current_local_thing_unique_id: u64,
    // pub draw_combat_collision_debug: bool,
    // pub draw_attitude_debug: bool,
    // pub max_thing_draw_dist: c_long,
    // pub global_thing_loading_behavior: self::EGlobalThingLoadingBehavior,
    // pub render_list: ThingLList,
    // pub update_lists: cxx::StdVector<ThingLList>,
    // pub use_map_specific_uids: bool,
    // pub update_and_render_all_flags: bool,
    // pub serialising_game_state: bool,
    // pub currently_loading: bool,
    // pub loading_entities_from_save_game: bool,
    // pub entitiy_runtime_persisitence: n_thing_manager::CEntityRuntimePersistence,
    // pub error_flag: bool,
    // pub npc_name_map: cxx::StdMap<u64, cxx::StdPair<CCharString, CWideString>, cxx::StdLess<u64>>,
    // pub serialise_version_number: c_long,
    // pub level_being_loaded: c_long,
    // pub current_selection_name: CCharString,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum EGlobalThingLoadingBehaviour {
    LOAD_ON_STARTUP = 1,
    LOAD_PER_LEVEL = 2,
}
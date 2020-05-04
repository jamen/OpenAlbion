#![allow(dead_code)]
#![allow(unused)]

use std::os::raw::{c_double,c_float,c_long,c_ulong,c_void,c_uchar};

use winapi::shared::ntdef::WCHAR;

use crate::cxx;
use crate::n_game_text;
use crate::{
    CASoundBank,
    CCharString,
    CCountedPointer,
    CDefinitionManager,
    CDiskFileWin32,
    CDisplayEngine,
    CEventPackageFileHeader,
    CFontBank,
    CFrameRateSmoother,
    CGame,
    CGameComponent,
    CGameEvent,
    CGameEventDispatch,
    CGameEventPackage,
    CGameEventPackageSet,
    CGamePlayerInterface,
    CGenericVar,
    CGraphicDataBank,
    CInterpolationInfo,
    CLUA,
    CMainGameComponentInit,
    CNetworkClient,
    CPlayerManager,
    CRoughFrameCounter,
    CWideString,
    CWorld,
};

#[repr(C)]
pub struct CMainGameComponent {
    pub vmt: usize,
    pub c_game_component: CGameComponent,
    pub p_sound_bank: *mut CASoundBank,
    pub p_text_bank: CCountedPointer<n_game_text::CDataBank>,
    pub p_player_manager: cxx::BoostScopedPtr<CPlayerManager>,
    pub p_player_interface: cxx::BoostScopedPtr<CGamePlayerInterface>,
    pub p_world: cxx::BoostScopedPtr<CWorld>,
    pub p_display_engine: cxx::BoostScopedPtr<CDisplayEngine>,
    pub p_lua: CCountedPointer<CLUA>,
    pub force_update_tick: bool,
    pub force_update_tick_speed_multiplier: c_float,
    pub force_update_tick_speed_desired_framerate: c_float,
    pub force_update_no_failed_updates: c_long,
    pub first_world_frame_update: bool,
    /// Could be absent in retail?
    pub current_server_frame: c_long,
    /// Could be absent in retail?
    pub input_server_frame: c_long,
    pub last_game_turn_force_rendered: c_long,
    pub current_frame_start_game_time: c_double,
    pub game_start_time: c_double,
    pub last_frame_render_duration: c_double,
    pub last_interpolation_info: CInterpolationInfo,
    // Cannot figure this out. Temporary fix.
    // pub event_package_set: CGameEventPackageSet,
    pub event_package_set: [u8; 14868],
    /// Could be absent in retail? The class is still present, so there's a good chance it is.
    pub client: CNetworkClient,
    pub no_render_frames_since_last_game_update: c_ulong,
    pub world_seed: c_ulong,
    pub local_seed: c_ulong,
    pub p_debug_font: CCountedPointer<CFontBank>,
    pub p_cut_scene_main_font: CCountedPointer<CFontBank>,
    pub event_package_file: CDiskFileWin32,
    pub loading_event_packages: bool,
    pub saving_event_packages: bool,
    pub event_package_file_header: CEventPackageFileHeader,
    pub frame_rate_smoother: CFrameRateSmoother,
    pub last_render_frame_start_time: c_double,
    pub time_passed_since_last_update: c_float,
    pub last_update_time: c_float,
    pub world_update_turn: bool,
    pub rough_fps_counter: CRoughFrameCounter,
    pub next_component_to_run: *mut CGameComponent,
    pub p_main_graphic_bank: CCountedPointer<CGraphicDataBank>,
    pub init_structure: CMainGameComponentInit,
    pub initialised: bool,
    pub allow_render: bool,
    pub rendered: bool,
    pub debug_no_frames_unable_to_render: c_long,
}

impl CMainGameComponent {
    fn update(&self) -> c_void { unimplemented!() }
    fn get_inputs(&self) -> c_void { unimplemented!() }
    fn render(&self) -> c_void { unimplemented!() }
    fn update_from_event_package_set(&self, event_package_set: *const CGameEventPackageSet) -> c_void { unimplemented!() }
    fn process_event_package(&self, event_package: *const CGameEventPackage) -> c_void { unimplemented!() }
    fn process_event(&self, event: *const CGameEvent) -> c_void { unimplemented!() }
    fn event_is_system_event(&self, event: *const CGameEvent) -> bool { unimplemented!() }
    fn init_definitions(&self, x: bool) -> bool { unimplemented!() }
    fn init_player_manager(&self) -> c_void { unimplemented!() }
    fn create_players(&self) -> c_void { unimplemented!() }
    fn initialise_text(&self) -> c_void { unimplemented!() }
    fn limit_fps(&self) -> c_void { unimplemented!() }
    fn check_sync(&self, event_package: *const CGameEventPackage) -> c_void { unimplemented!() }
    fn init_update_and_render_all_entities(&self) -> c_void { unimplemented!() }
    fn init_particle_engine(&self) -> c_void { unimplemented!() }
    fn init_graphics(&self) -> bool { unimplemented!() }
    fn get_event_package_set_from_save(&self, event_package_set: *mut CGameEventPackageSet) -> bool { unimplemented!() }
    fn add_event_package_set_to_save(&self, event_package_set: *const CGameEventPackageSet) -> c_void { unimplemented!() }
    fn init_event_package_loading(&self, x: *mut WCHAR) -> bool { unimplemented!() }
    fn init_event_package_saving(&self, x: *mut WCHAR) -> bool { unimplemented!() }
    fn uninit_event_package_loading(&self) -> c_void { unimplemented!() }
    fn uninit_event_package_saving(&self) -> c_void { unimplemented!() }
    fn update_average_frame_duration(&self, x: c_double) -> c_void { unimplemented!() }
    fn get_average_frame_duration(&self) -> c_float { unimplemented!() }
    fn get_current_frame_finish_time_approximation(&self) -> c_double { unimplemented!() }
    fn get_current_frame_start_game_time(&self) -> c_double { unimplemented!() }
    fn get_game_start_time(&self) -> c_double { unimplemented!() }
    fn get_current_game_time(&self) -> c_double { unimplemented!() }
    fn convert_gt_to_wf(&self, x: c_double) -> c_double { unimplemented!() }
    fn get_render_interpolate(&self) -> c_float { unimplemented!() }
    fn get_predicted_time_since_last_render_frame(&self) -> c_double { unimplemented!() }
    fn get_last_frame_render_duration(&self) -> c_double { unimplemented!() }
    fn get_game_time_of_next_present_completion(&self) -> c_double { unimplemented!() }
    fn force_render(&self, x: c_long) -> c_void { unimplemented!() }
    fn init_world(&self) -> c_void { unimplemented!() }
    fn init_display_engine(&self) -> c_void { unimplemented!() }
    fn init_sound(&self) -> c_void { unimplemented!() }
    fn initialise_fonts(&self) -> c_void { unimplemented!() }
    fn init_lua(&self, x: *const CCharString) -> c_void { unimplemented!() }
    fn post_init(&self) -> c_void { unimplemented!() }
    fn validate_definitions(&self) -> c_void { unimplemented!() }
    fn shutdown(&self) -> c_void { unimplemented!() }
    fn get_frame_difference_from_current(&self, x: c_long) -> c_long { unimplemented!() }
    fn begin_input_saving(&self, x: *const CWideString) -> c_void { unimplemented!() }
    fn begin_input_loading(&self, x: *const CWideString) -> c_void { unimplemented!() }
    fn get_world(&self) -> *mut CWorld { unimplemented!() }
    fn peek_world(&self) -> *const CWorld { unimplemented!() }
    fn get_sample_bank(&self) -> *mut CASoundBank { unimplemented!() }
    fn peek_sample_bank(&self) -> *const CASoundBank { unimplemented!() }
    fn get_player_manager(&self) -> *mut CPlayerManager { unimplemented!() }
    fn peek_player_manager(&self) -> *const CPlayerManager { unimplemented!() }
    fn peek_event_dispatch_table_entry(&self, x: c_long) -> *const CGameEventDispatch { unimplemented!() }
    fn is_initialised(&self) -> bool { unimplemented!() }
    fn has_rendered(&self) -> bool { unimplemented!() }
    fn is_controller_disconnected(&self) -> bool { unimplemented!() }
    fn c_main_game_component_1(&self, x: *const CMainGameComponent) -> c_void { unimplemented!() }
    fn c_main_game_component_2(&self, x: *mut CGame, y: *const CMainGameComponent) -> c_void { unimplemented!() }
    /// Virtual function
    fn c_main_game_component_destructor(&self) -> c_void { unimplemented!() }
    /// Virtual function
    fn init(&self) -> c_void { unimplemented!() }
    /// Virtual function
    fn run(&self, x: *mut *mut CMainGameComponent) -> bool { unimplemented!() }
    fn update_regular(&self) -> c_void { unimplemented!() }
    fn update_forced_tick_speed(&self) -> c_void { unimplemented!() }
    fn start_letter_box_mode(&self, a: c_float, b: c_float, c: c_float) -> c_void { unimplemented!() }
    fn end_letter_box_mode(&self) -> c_void { unimplemented!() }
    fn is_display_engine_initialised(&self) -> bool { unimplemented!() }
    fn get_display_engine(&self) -> *mut CDisplayEngine { unimplemented!() }
    fn peek_display_engine(&self) -> *const CDisplayEngine { unimplemented!() }
    fn pre_change_resolution(&self) -> c_void { unimplemented!() }
    fn post_change_resolution(&self, a: c_ulong, b: c_ulong, c: c_ulong) -> c_void { unimplemented!() }
    /// Virtual function
    fn change_texture_colour_depth(&self, x: c_long) -> c_void { unimplemented!() }
    fn change_max_texture_size(&self, x: c_ulong) -> c_void { unimplemented!() }
    /// Virtual function
    fn set_quit(&self) -> c_void { unimplemented!() }
    fn set_next_component(&self, x: *const CGameComponent) -> c_void { unimplemented!() }
    fn set_exclusive_mode(&self, x: bool) -> c_void { unimplemented!() }
    fn set_display_mode(&self, a: c_ulong, b: c_ulong, c: c_uchar) -> c_void { unimplemented!() }
    fn get_world_seed(&self) -> *mut c_ulong { unimplemented!() }
    fn get_local_seed(&self) -> *mut c_ulong { unimplemented!() }
    fn peek_world_seed(&self) -> c_ulong { unimplemented!() }
    fn peek_local_seed(&self) -> c_ulong { unimplemented!() }
    fn peek_main_font(&self) -> *const CFontBank { unimplemented!() }
    fn peek_debug_font(&self) -> *const CFontBank { unimplemented!() }
    fn peek_cut_scene_main_font(&self) -> *const CFontBank { unimplemented!() }
    fn is_time_for_server_update(&self, x: c_long) -> bool { unimplemented!() }
    fn signal_frame_update(&self) -> c_void { unimplemented!() }
    fn get_p_player_interface(&self) -> *mut CGamePlayerInterface { unimplemented!() }
    fn peek_definition_manager(&self) -> *const CDefinitionManager { unimplemented!() }
    fn get_last_fps(&self) -> c_float { unimplemented!() }
    fn is_editor_active(&self) -> bool { unimplemented!() }
    fn get_current_world_frame(&self) -> c_long { unimplemented!() }
    fn get_current_server_frame(&self) -> c_long { unimplemented!() }
    fn get_engine_graphics(&self) -> *mut CGraphicDataBank { unimplemented!() }
    fn peek_engine_graphics(&self) -> *const CGraphicDataBank { unimplemented!() }
    fn get_text_bank(&self) -> *const n_game_text::CDataBank { unimplemented!() }
    fn is_text_bank_initialised(&self) -> bool { unimplemented!() }
    fn get_time_since_last_render_frame(&self) -> c_float { unimplemented!() }
    fn get_time_passed_since_last_update(&self) -> c_float { unimplemented!() }
    /// Virtual function
    fn on_pre_device_reset(&self) -> c_void { unimplemented!() }
    /// Virtual function
    fn on_post_device_reset(&self) -> c_void { unimplemented!() }
    fn add_game_event(&self, event: *const CGameEvent) -> c_void { unimplemented!() }
    fn peek_last_interpolation_info(&self) -> *const CInterpolationInfo { unimplemented!() }
    /// CMainGameComponent* operator=(const CMainGameComponent*)
    fn assign(&self, x: *const CMainGameComponent) { unimplemented!() }
}

impl CMainGameComponent {
    fn app_reinit_func(x: *mut c_void) -> c_void { unimplemented!() }
    fn console_set_resolution(mgc: *mut CMainGameComponent, x: cxx::StdVector<CGenericVar>) -> c_void { unimplemented!() }
    fn console_force_update_tick_speed(mgc: *mut CMainGameComponent, x: cxx::StdVector<CGenericVar>) -> c_void  { unimplemented!() }
    fn get() -> *mut CMainGameComponent { unimplemented!() }
    fn get_constant_fps() -> c_long { unimplemented!() }
    fn convert_wf_to_seconds(x: c_float) -> c_float { unimplemented!() }
    fn convert_seconds_to_wf(x: c_float) -> c_float { unimplemented!() }
    fn generate_met_files_from_lut_files() -> c_void { unimplemented!() }
}
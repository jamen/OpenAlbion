//! Collection of function and data offsets.
//!
//! Some entries are unnamed but symbolized by something, i.e. "PARENT_OF_*" or "ADJACENT_OF_*".
//!
//! TODO: Categorize these into modules further or nah?

#![allow(dead_code)]

//
// Functions
//

pub const ENTRY: usize = 0x401067;
pub const WIN_MAIN: usize = 0x403480;
/// The top-most init function.
pub const GF_MAIN: usize = 0x402510;
pub const GF_RUN_INIT_SCRIPTS: usize = 0x413c50;
pub const C_SYSTEM_MANAGER_INIT: usize = 0x403b10;
pub const CF_GET_SYSTEM_MANAGER: usize = 0x9a4ec0;
pub const C_GAME__PLAY: usize = 0x412f90;

pub const C_MAIN_GAME_COMPONENT__INIT_GRAPHICS: usize = 0x416c8a;
// pub const C_MAIN_GAME_COMPONENT__INITIALISE_FONTS: usize = 0x416c8a;

pub const ZLIB_CRC32: usize = 0xc05fd0;
/// Very large function.
pub const C_CHAR_STRING__CONSTRUCTOR: usize = 0x99ebf0;
pub const C_CHAR_STRING__RUN_CUTSCENE_MACRO: usize = 0xcbfb7d;
pub const C_CHAR_STRING__UNASSIGN_STRING: usize = 0x99e9b0;
pub const C_CHAR_STRING__ALLOC_STRING_DATA: usize = 0x99ea60;

pub const C_BASIC_STRING__SET_STRING: usize = 0x9a0300;

/// Good candidates for modding.
pub const C_SCRIPT_INFO_MANAGER__INIT: usize = 0xcb5d80;
pub const C_SCRIPT_INFO_MANAGER__REGISTER_SCRIPTS: usize = 0xcd52d0;
pub const C_SCRIPT_INFO_MANAGER__REGISTER_DEFS: usize = 0xf2a0f0;

/// Parent of Direct3DCreate9 call.
pub const C_DISPLAY_MANAGER__INIT: usize = 0x9c0e50;
/// Adjacent of Direct3DCreate9 call.
pub const C_DISPLAY_MANAGER__CREATE_DEVICE: usize = 0x9bf7e0;
pub const C_RENDER_MANAGER_CORE__INIT: usize = 0xa0a940;
pub const C_SHADER_RENDER_MANAGER__INITIALISE: usize = 0x98acf0;

pub const C_WORLD__INIT: usize = 0x4a6e30;
pub const C_WORLD__LOAD_GAME_STATE_INTERNAL: usize = 0x4a21f0;
pub const C_WORLD__SAVE_GAME_STATE_INTERNAL: usize = 0x49f4c0;
pub const C_WORLD__ACTIVATE_WORLD: usize = 0x49f180;

pub const N_SCRIPT__C_GAMEFLOW_SCRIPT__DECLARE_GOSSIP_CATEGORIES: usize = 0xce6cf0;
/// Used in 0xe6068b patch
pub const N_SCRIPT__CV_BODY_GAURD_SCRIPT__C_BODY_GAURD__MAIN: usize = 0xe5fd40;

pub const C_CONSOLE__INITIALISE: usize = 0x9ed190;

pub const C_USER_PROFILE_MANAGER__IS_DEBUG_PROFILE: usize = 0x407030;

pub const C_NEW_FRONTEND_GAME_COMPONENT__COMPILE_DEFS: usize = 0x42f70b;
pub const C_NEW_FRONTEND_GAME_COMPONENT__INITIALISE_DEFS: usize = 0x42f722;

pub const N_GLOBAL_CONSOLE__INITIALISE: usize = 0x419d90;

pub const C_PLAYER_GUI__ADD_SCREEN_MESSAGE: usize = 0x44bb90;


//
// Data
//

/// Referenced lots of items. Maybe an entry into a lot of the game state.
pub const P_MAIN_GAME_COMPONENT: usize = 0x13b86a0;
pub const P_GUI_DEF: usize = 0x13b878c;

pub const G_EDIT: usize = 0x13b8605;
pub const G_DO_NOT_CALL_START_AUTO_SAVE_PROGRESS: usize = 0x13b89d1;
pub const G_DO_NOT_CALL_STOP_AUTO_SAVE_PROGRESS: usize = 0x13b89d0;
pub const MAYBE_HERO_STATS_DISPLAY: usize = 0x5ce0e6;
/// Maybe the frame count?
pub const C_WORLD__FRAME: usize = 0x13b89bc;
pub const G_ALLOW_DEBUG_PROFILE: usize = 0x1375741;
pub const G_SHOW_DEV_FRONTEND: usize = 0x13b8642;

pub const NO_STRINGS: usize = 0x13bd800;

pub const G_FULL_SCREEN: usize = 0x137544a;
pub const G_ANTIALIASING_ON: usize = 0x13b8607;
pub const G_ANTIALIASING_X9: usize = 0x13b8608;
pub const G_KICKOFF_SIZE: usize = 0x1375478;
pub const G_PUSH_BUFFER_SIZE: usize = 0x1375474;
pub const G_RESOLUTION_REFRESH_RATE: usize = 0x13b8628;
pub const G_Z_DEPTH_BUFFER: usize = 0x13b8624;
pub const G_RESOLUTION_WIDTH: usize = 0x1375468;
pub const G_RESOLUTION_DEPTH: usize = 0x1375460;
pub const G_PRESENT_IMMEDIATE: usize = 0x137545c;
pub const G_RESOLUTION_HEIGHT: usize = 0x1375464;
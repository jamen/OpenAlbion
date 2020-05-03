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
pub const GF_INITIALISE: usize = 0x4022b0;
pub const GF_UNINITIALISE: usize = 0x401b80;
pub const GF_RUN_INIT_SCRIPTS: usize = 0x413c50;
pub const GF_GET_BUILD_NUMBER_2: usize = 0x401f30;

pub const C_SYSTEM_MANAGER_INIT: usize = 0x403b10;
pub const C_SYSTEM_MANAGER__INITIALISE: usize = 0x9a6610;
pub const CF_GET_SYSTEM_MANAGER: usize = 0x9a4ec0;

pub const C_GAME__INITIALISE: usize = 0x413120;
pub const C_GAME__PLAY: usize = 0x412f90;

pub const C_MAIN_GAME_COMPONENT__C_MAIN_GAME_COMPONENT: usize = 0x418dca;
pub const C_MAIN_GAME_COMPONENT__VTABLE: usize = 0x122f180;
pub const C_MAIN_GAME_COMPONENT__INIT_GRAPHICS: usize = 0x416c8a;
pub const C_MAIN_GAME_COMPONENT__RUN: usize = 0x4189c2;
pub const C_MAIN_GAME_COMPONENT__UPDATE: usize = 0x418289;
pub const C_MAIN_GAME_COMPONENT__GENERATE_MET_FILES_FROM_LUG_FILES: usize = 0x418c3b;

pub const ZLIB_CRC32: usize = 0xc05fd0;
/// Very large function.
pub const C_CHAR_STRING__C_CHAR_STRING: usize = 0x99ebf0;
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

/// This is the regular frontend for the main game.
pub const C_FRONTEND_GAME_COMPONENT__C_FRONTEND_GAME_COMPONENT: usize = 0x496070;

/// This is the dev frontend.
pub const C_NEW_FRONTEND_GAME_COMPONENT__C_NEW_FRONTEND_GAME_COMPONENT: usize = 0x42ea8f;
pub const C_NEW_FRONTEND_GAME_COMPONENT__COMPILE_DEFS: usize = 0x42f70b;
pub const C_NEW_FRONTEND_GAME_COMPONENT__INITIALISE_DEFS: usize = 0x42f722;

pub const N_GLOBAL_CONSOLE__INITIALISE: usize = 0x419d90;
pub const C_PLAYER_GUI__ADD_SCREEN_MESSAGE: usize = 0x44bb90;

/// Parent of DirectInput8Create, C_INPUT_MANAGER, C_KEYBOARD, and C_MOUSE calls.
pub const C_INPUT_MANAGER_DX__C_INPUT_MANAGER_DX: usize = 0xa60050;
pub const C_INPUT_MANAGER__C_INPUT_MANAGER: usize = 0x9f5bd0;
pub const C_INPUT_EVENT__C_INPUT_EVENT: usize = 0xa04410;

pub const C_KEYBOARD_DX__C_KEYBOARD_DX: usize = 0xab64e0;
pub const C_KEYBOARD__C_KEYBOARD: usize = 0xa66930;
pub const C_KEYBOARD__VTABLE: usize = 0x129dba8;
/// This is empty for some reason. Try some hooks? Maybe only menu related.
pub const C_GUI_WINDOW__ON_KEY_PRESS: usize = 0xa66ad0;

pub const C_MOUSE_DX__C_MOUSE_DX: usize = 0xab5d00;
pub const C_MOUSE__C_MOUSE: usize = 0xa66f20;
pub const C_MOUSE__VTABLE: usize = 0x129dbc4;

pub const CTC_PHYSICS_BASE__SET_RH_SET: usize = 0xa673a0;



//
// Data
//

/// Referenced lots of items. Maybe an entry into a lot of the game state.
pub const P_MAIN_GAME_COMPONENT: usize = 0x13b86a0;
pub const P_GUI_DEF: usize = 0x13b878c;
/// This is a pointer to string data I believe.
pub const G_VERSION_STRING: usize = 0x13b85e0;

pub const GP_FONT_MANAGER: usize = 0x13b8394;
pub const GP_DISPLAY_MANAGER: usize = 0x13b8390;
pub const GP_RENDER_MANAGER: usize = 0x13b8384;
pub const GP_SHADER_RENDER_MANAGER: usize = 0x13b8380;
pub const C_SHADER_RENDER_MANAGER__SINGLETON_INSTANCE: usize = 0x13bc470;
pub const GP_INPUT_MANAGER: usize = 0x13b8388;
pub const GP_GRAPHICS_BANK_MANAGER: usize = 0x13b837c;
/// Set to value of G_DESIRED_TEXTURE_DEPTH
pub const G_TEXTURE_BIT_DEPTH: usize = 0x13b7d68;
pub const G_DESIRED_TEXTURE_DEPTH: usize = 0x1375470;
pub const G_MAX_TEXTURE_WIDTH: usize = 0x13b7d64;
pub const G_MAX_DESIRED_TEXTURE_SIZE: usize = 0x137546c;
pub const G_MAX_TEXTURE_HEIGHT: usize = 0x13b7d60;

pub const G_EDIT: usize = 0x13b8605;
pub const G_DO_NOT_CALL_START_AUTO_SAVE_PROGRESS: usize = 0x13b89d1;
pub const G_DO_NOT_CALL_STOP_AUTO_SAVE_PROGRESS: usize = 0x13b89d0;
pub const G_OLD_GAME_COMPONENT_TO_DELETE: usize = 0x13b7d58;

// pub const MAYBE_HERO_STATS_DISPLAY: usize = 0x5ce0e6;

/// Maybe the frame count?
pub const C_WORLD__FRAME: usize = 0x13b89bc;
pub const G_ALLOW_DEBUG_PROFILE: usize = 0x1375741;
pub const G_SHOW_DEV_FRONTEND: usize = 0x13b8642;

pub const VC_FILE_INSTALLER__P_CLASS: usize = 0x13ca818;

/// Set to false when G_ONLY_BUILD_STATIC_MAPS is disabled.
pub const N_GLOBAL_CONSOLE__ENABLE_PARTICLES: usize = 0x1375754;

/// Set to value of G_ALLOW_DATA_GENERATION.
pub const C_DEFINITION_MANAGER__CREATE_COMPILED_DEFS: usize = 0x138e188;
/// Set to value of G_ALLOW_DATA_GENERATION.
pub const C_DEFINITION_MANAGER__CREATE_CHECKSUMS: usize = 0x138e189;
/// Set to value of G_USE_COMPILED_DEFS.
pub const C_DEFINITION_MANAGER__USING_COMPILED_DEFS: usize = 0x13ca7d8;

pub const NO_STRINGS: usize = 0x13bd800;

pub const G_FULL_SCREEN: usize = 0x137544a;
pub const G_ANTIALIASING_ON: usize = 0x13b8607;
pub const G_ANTIALIASING_X9: usize = 0x13b8608;
pub const G_KICKOFF_SIZE: usize = 0x1375478;
pub const G_PUSH_BUFFER_SIZE: usize = 0x1375474;
pub const G_PRESENT_IMMEDIATE: usize = 0x13b8628;
pub const G_RESOLUTION_DEPTH: usize = 0x13b8624;
pub const G_RESOLUTION_REFRESH_RATE: usize = 0x1375468;
pub const G_RESOLUTION_HEIGHT: usize = 0x1375460;
pub const G_RESOLUTION_WIDTH: usize = 0x137545c;
pub const G_Z_DEPTH_BUFFER: usize = 0x1375464;
pub const G_FORCE_PRIMARY: usize = 0x13b8604;
pub const G_DELAY_EACH_FRAME_MS: usize = 0x13b8610;

pub const G_RUN_INI_SCRIPTS: usize = 0x137548f;
pub const G_IGNORE_TIMESTAMP_ON_INSTALL: usize = 0x13b860b;
pub const G_ALLOW_DATA_GENERATION: usize = 0x1375459;
pub const G_RUN_FROM_DVD: usize = 0x13b8615;
pub const G_PREVENT_INSTALLATION: usize = 0x1375447;
pub const G_LEFT_ALIGN_TEXT: usize = 0x13b861b;
pub const G_NO_HANGUL_WORD_WRAP: usize = 0x13b861c;
pub const G_DISABLE_CAPS_LOCK: usize = 0x13b864f;
pub const G_USE_RETAIL_BANKS: usize = 0x13b8616;
pub const G_USE_COMPILED_DEFS: usize = 0x13b8617;
pub const G_ALLOW_BACKGROUND_PROCESSING: usize = 0x13b861e;
pub const G_SKIP_CONFIG_DETECTION: usize = 0x13b861d;
pub const G_ONLY_BUILD_STATIC_MAPS: usize = 0x13b8648;

pub const G_SAVE_INPUTS: usize = 0x13b85f6;
pub const G_LOAD_INPUTS: usize = 0x13b85f5;
pub const G_INPUT_FILE_NAME: usize = 0x13b865c;

pub const G_CONFIG_OPTIONS__USE_ALTERNATE_RENDERING_TARGET_CLEAR: usize = 0x13750a4;
pub const G_CONFIG_OPTIONS__ENABLE_TEXTURE_TRANSFORMS: usize = 0x13750aa;
pub const G_CONFIG_OPTIONS__USE_FIXED_FUNCTION_TEXT: usize = 0x13750ab;

pub const RELAUNCH_SAVE_GAME: usize = 0x126c8a4;

pub const EMPTY_WIDE_STRING: usize = 0x122d70c;
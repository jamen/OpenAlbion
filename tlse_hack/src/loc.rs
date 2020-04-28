//! Collection of function and data offsets.
//!
//! Some entries are unnamed but symbolized by something, i.e. "PARENT_OF_*" or "ADJACENT_OF_*".
//!
//! TODO: Categorize these into modules further or nah?

#![allow(dead_code)]

pub mod func {
    pub const ENTRY: u32 = 0x00401067;
    pub const WIN_MAIN: u32 = 0x00403480;
    /// The top-most init function.
    pub const GF_MAIN: u32 = 0x00402510;
    pub const GF_RUN_INIT_SCRIPTS: u32 = 0x00413c50;
    pub const C_SYSTEM_MANAGER_INIT: u32 = 0x00403b10;
    pub const CF_GET_SYSTEM_MANAGER: u32 = 0x009a4ec0;
    pub const C_GAME__PLAY: u32 = 0x00412f90;

    pub const C_MAIN_GAME_COMPONENT__INIT_GRAPHICS: u32 = 0x00416c8a;
    pub const C_MAIN_GAME_COMPONENT__INITIALISE_FONTS: u32 = 0x00416c8a;

    pub const ZLIB_CRC32: u32 = 0x00c05fd0;
    /// Very large function.
    pub const C_CHAR_STRING__RUN_CUTSCENE_MACRO: u32 = 0x00cbfb7d;

    /// Good candidates for modding.
    pub const C_SCRIPT_INFO_MANAGER__INIT: u32 = 0x00cb5d80;
    pub const C_SCRIPT_INFO_MANAGER__REGISTER_SCRIPTS: u32 = 0x00cd52d0;
    pub const C_SCRIPT_INFO_MANAGER__REGISTER_DEFS: u32 = 0x00f2a0f0;

    /// Parent of Direct3DCreate9 call.
    pub const C_DISPLAY_MANAGER__INIT: u32 = 0x009c0e50;
    /// Adjacent of Direct3DCreate9 call.
    pub const C_DISPLAY_MANAGER__CREATE_DEVICE: u32 = 0x009bf7e0;
    pub const C_RENDER_MANAGER_CORE__INIT: u32 = 0x00a0a940;
    pub const C_SHADER_RENDER_MANAGER__INITIALISE: u32 = 0x0098acf0;

    pub const C_WORLD__LOAD_GAME_STATE_INTERNAL: u32 = 0x004a21f0;
    pub const C_WORLD__SAVE_GAME_STATE_INTERNAL: u32 = 0x0049f4c0;
    pub const C_WORLD__ACTIVATE_WORLD: u32 = 0x0049f180;

    pub const N_SCRIPT__C_GAMEFLOW_SCRIPT__DECLARE_GOSSIP_CATEGORIES: u32 = 0x00ce6cf0;
    /// Used in 0x00e6068b patch
    pub const N_SCRIPT__CV_BODY_GAURD_SCRIPT__C_BODY_GAURD__MAIN: u32 = 0x00e5fd40;

    pub const C_CONSOLE__INITIALISE: u32 = 0x009ed190;

    pub const C_USER_PROFILE_MANAGER__IS_DEBUG_PROFILE: u32 = 0x00407030;
}

pub mod data {
    /// Referenced lots of items. Maybe an entry into a lot of the game state.
    pub const P_MAIN_GAME_COMPONENT: u32 = 0x013b86a0;
    pub const G_EDIT: u32 = 0x013b8605;
    pub const G_DO_NOT_CALL_START_AUTO_SAVE_PROGRESS: u32 = 0x013b89d1;
    pub const G_DO_NOT_CALL_STOP_AUTO_SAVE_PROGRESS: u32 = 0x013b89d0;
    pub const MAYBE_HERO_STATS_DISPLAY: u32 = 0x005ce0e6;
    /// Maybe the frame count?
    pub const C_WORLD__FRAME: u32 = 0x013b89bc;
}

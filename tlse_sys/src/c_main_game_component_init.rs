use crate::{CWideString,CCharString};

#[repr(C)]
pub struct CMainGameComponentInit {
    pub initial_world_name: CWideString,
    pub initial_world_holy_site_name: CWideString,
    pub initial_quest_name: CCharString,
    pub save_game_name: CWideString,
}
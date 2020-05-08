use std::os::raw::c_long;

use crate::cxx;
use crate::{CBaseClassNonCopyable,CCharString,CMainGameComponent,CGameDefinitionManager,CPlayer};

/// TODO: This is behind a pointer so I've left it empty for now.
#[derive(Debug)]
#[repr(C)]
pub struct CPlayerManager {
    pub vmt: *mut (),
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub component: *mut CMainGameComponent,
    /// Apparently this is a forward declaration with no actual definition. See also CDefinitionManager.
    pub definition_manager: *const CGameDefinitionManager,
    pub players: cxx::StdVector<*mut CPlayer>,
    pub player_neutral: c_long,
    pub main_player: c_long,
    pub hero_swap_player_script_names: cxx::StdVector<CCharString>,
}
use crate::{CBaseClassNonCopyable,CInputProcessManager,CLinkedListPosition};

#[derive(Debug)]
#[repr(C)]
pub struct CAInputProcess {
    pub vmt: *mut (),
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub p_player_interface: CInputProcessManager,
    pub input_process_list_pos: CLinkedListPosition,
}

impl CAInputProcess {
}
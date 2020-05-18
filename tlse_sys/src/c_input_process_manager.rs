use std::os::raw::c_long;

use crate::{CAInputProcess,CBaseClassNonCopyable,CLinkedList,CProcessedInput};

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessManager {
    pub vmt: *mut (),
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub input_process_list: CLinkedList<CAInputProcess>,
    pub queued_processed_inputs: [CProcessedInput; 10],
    pub no_queued_processed_inputs: c_long,
}
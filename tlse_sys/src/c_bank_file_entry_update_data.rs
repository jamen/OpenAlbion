// use std::os::raw::{c_ulong,c_char};

// use crate::{CBankStateBlock,CCharString,CCountedPointer,CSmallVector};

/// TODO: I've left this empty for now because its behind a pointer.
#[derive(Debug)]
#[repr(C)]
pub struct CBankFileEntryUpdateData {
    // pub state_block_crc: c_ulong,
    // pub info_size: c_ulong,
    // pub info_buffer: *mut c_char,
    // pub filenames: CSmallVector<CCharString, 8>,
    // pub requires_update: bool,
    // pub exists: bool,
    // pub state_block: CCountedPointer<CBankStateBlock>,
}
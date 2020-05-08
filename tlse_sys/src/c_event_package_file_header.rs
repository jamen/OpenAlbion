use std::os::raw::c_long;

#[derive(Debug)]
#[repr(C)]
pub struct CEventPackageFileHeader {
    pub no_players: c_long,
}
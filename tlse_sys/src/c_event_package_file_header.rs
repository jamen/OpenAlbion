use std::os::raw::c_long;

#[repr(C)]
pub struct CEventPackageFileHeader {
    pub no_players: c_long,
}
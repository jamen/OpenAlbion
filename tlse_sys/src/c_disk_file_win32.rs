/// TODO: This is behind a pointer so I've left it empty for now.
#[derive(Debug)]
#[repr(C)]
pub struct CDiskFileWin32 {
    pub vmt: *mut (),
}
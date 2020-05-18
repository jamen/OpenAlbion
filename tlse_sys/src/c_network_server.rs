use crate::CInitBaseClass;

#[derive(Debug)]
#[repr(C)]
pub struct CNetworkServer {
    pub vmt: *mut (),
    pub c_init_base_class: CInitBaseClass,
}
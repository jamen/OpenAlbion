use std::os::raw::c_ulong;

use crate::CGameEventPackage;

#[repr(C)]
pub struct CGameEventPackageSet {
    pub no_packages: c_ulong,
    // pub packages: Box<[CGameEventPackage; 80400]>,
    pub packages: [CGameEventPackage; 826],
}
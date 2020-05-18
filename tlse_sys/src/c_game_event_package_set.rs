use std::os::raw::c_ulong;
use std::fmt;

use crate::CGameEventPackage;

#[repr(C)]
pub struct CGameEventPackageSet {
    pub packages_count: c_ulong,
    // pub packages: Box<[CGameEventPackage; 80400]>,
    pub packages: [CGameEventPackage; 50],
}

impl fmt::Debug for CGameEventPackageSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CGameEventPackageSet")
            .field("packages_count", &self.packages_count)
            .field("packages", &&self.packages[..])
            .finish()
    }
}
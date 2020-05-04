use crate::cxx::StdPair;

use crate::CArray;

#[repr(C)]
pub struct CVectorMap<K, V, C> {
    pub inherited_c_array: CArray<StdPair<K, V>>,
    pub compare: C,
    pub dirty: bool,
}

impl<K, V, C> CVectorMap<K, V, C> {
}
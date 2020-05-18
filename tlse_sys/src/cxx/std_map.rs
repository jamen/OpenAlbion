use crate::cxx;

#[derive(Debug)]
#[repr(C)]
pub struct StdMap<
    Key,
    T,
    Compare = cxx::StdLess<Key>,
    Alloc = cxx::StdAllocator<cxx::StdPair<Key, T>>
> {
    pub proxy: *mut (),
    pub comp: Compare,
    pub head: *mut StdMap<T, Key, Compare, Alloc>,
    pub _aloc_node: cxx::StdAllocator<StdMap<T, Key, Compare, Alloc>>,
    pub _alloc_value: Alloc,
}
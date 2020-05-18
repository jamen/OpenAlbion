use crate::CWideString;

#[derive(Debug)]
#[repr(C)]
pub struct CHeroLogBookEntry {
    pub name: CWideString,
    pub abbreviated_name: CWideString,
    pub content: CWideString,
}
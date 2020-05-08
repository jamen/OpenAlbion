use crate::CBankFile;

#[derive(Debug)]
#[repr(C)]
pub struct CBankFileAsync {
    pub c_bank_file: CBankFile,
}
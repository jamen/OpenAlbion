use crate::CBankFile;

#[repr(C)]
pub struct CBankFileAsync {
    pub inherited_c_bank_file: CBankFile,
}
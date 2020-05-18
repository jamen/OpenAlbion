
use crate::{
    CCountedPointer,
    NDisplayView,
};

#[derive(Debug)]
#[repr(C)]
pub struct CDisplayViewManager {
    pub p_current_view: CCountedPointer<NDisplayView::CViewBase>
}
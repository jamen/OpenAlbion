use crate::C3DVector;

#[derive(Debug)]
#[repr(C)]
pub struct CRightHandedSet {
    pub up: C3DVector,
    pub forward: C3DVector,
}
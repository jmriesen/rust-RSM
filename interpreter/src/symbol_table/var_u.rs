//New type wrapper so I can implement methods on VAR_U
//TODO decouple from ffi
use std::hash::Hash;
#[derive(Clone, Debug)]
pub struct VarU(pub ffi::VAR_U);

impl Hash for VarU {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.0.var_cu }.hash(state);
    }
}
impl Eq for VarU {}
impl PartialEq for VarU {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.var_cu == other.0.var_cu }
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {
    use super::VarU;

    #[must_use]
    pub fn var_u(var: &str) -> VarU {
        VarU(var.try_into().unwrap())
    }
}

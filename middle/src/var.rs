use crate::bindings::var_u;

impl var_u {
    pub fn as_array(&self) -> &[u8] {
        unsafe { &self.var_cu }
    }
}

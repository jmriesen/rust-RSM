use crate::bindings::var_u;

//TODO note can fail if string is longer then 32 characters.
impl From<&str> for var_u {
    fn from(var: &str) -> Self {
        let bytes = var.as_bytes();
        let mut buffer = [0u8; 32];
        buffer[..bytes.len()].copy_from_slice(bytes);
        Self { var_cu: buffer }
    }
}

impl var_u {
    pub fn as_array(&self) -> &[u8] {
        unsafe { &self.var_cu }
    }
}

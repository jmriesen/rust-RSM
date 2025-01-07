use super::{MAX_STR_LEN, Value};
use ffi::CSTRING;
#[cfg_attr(test, mutants::skip)]
impl Value {
    #[must_use]
    pub fn into_cstring(self) -> CSTRING {
        let mut buf = [0; MAX_STR_LEN + 1];
        buf[..self.0.len()].copy_from_slice(&self.0[..]);
        CSTRING {
            len: self.0.len().try_into().expect("Max var len < u16::max"),
            buf,
        }
    }
}
impl From<&CSTRING> for Value {
    #[cfg_attr(test, mutants::skip)]
    fn from(value: &CSTRING) -> Self {
        let data = &value.buf[..value.len as usize];
        Self(Vec::from(data))
    }
}

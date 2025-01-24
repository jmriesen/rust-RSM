use crate::{
    bindings::{CSTRING, MAX_STR_LEN},
    IntoC,
};
use value::Value;

impl IntoC for Value {
    type CType = CSTRING;
    #[must_use]
    fn into_c(self) -> CSTRING {
        let mut buf = [0; MAX_STR_LEN as usize + 1];
        let content = self.content();
        buf[..content.len()].copy_from_slice(&content);
        CSTRING {
            len: content.len().try_into().expect("Max var len < u16::max"),
            buf,
        }
    }
}
impl From<&CSTRING> for Value {
    fn from(value: &CSTRING) -> Self {
        let data = &value.buf[..value.len as usize];
        Self::try_from(data).expect("Max size should not be an issue")
    }
}

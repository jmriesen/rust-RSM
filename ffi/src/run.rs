use std::ffi::CStr;

use crate::{UCIS, UCI_TAB};
#[derive(Debug, PartialEq, Eq)]
pub struct Error(pub i32);

pub fn parse_env(env: &CStr, ucis: &[UCI_TAB; UCIS as usize]) -> Result<i32, Error> {
    let result =
        unsafe { crate::bindings::parse_env(env.as_ptr().cast_mut(), ucis.as_ptr().cast_mut()) };
    if result.is_error != 0 {
        Err(Error(result.error))
    } else {
        Ok(result.value)
    }
}

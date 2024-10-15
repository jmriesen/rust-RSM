use std::ffi::CStr;

use crate::{jobtab, u_char, UCIS, UCI_TAB};
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

pub enum StartType {
    Run = 1,
    Job = 2,
    Do = 3,
    Extrinsic = 4,
    Xecute = 5,
}

pub fn find_open_slot(
    job_table: &mut [jobtab],
    start_type: StartType,
    pid: i32,
) -> Option<&mut jobtab> {
    unsafe {
        crate::bindings::find_open_slot(
            job_table.as_mut_ptr(),
            job_table
                .len()
                .try_into()
                .expect("Provided job tab len is to large for C"),
            start_type as u_char,
            pid,
        )
        .as_mut()
    }
}

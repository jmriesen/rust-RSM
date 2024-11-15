use std::ffi::CStr;

use libc::gid_t;

use crate::{jobtab, set_tab_priv, u_char, UCIS, UCI_TAB};
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

/// See set_tab_priv;
/// The first value in the Error is the error code.
/// NOTE: If the second value in the error is true the caller should set the tab to null. (to mimic
/// the C code)
pub fn tab_calculate_privilage(
    current_user: i32,
    system_start_user: i32,
    maxjob: u32,
    current_groups: &[gid_t],
) -> Result<bool, (i32, bool)> {
    let result = unsafe {
        set_tab_priv(
            current_user,
            system_start_user,
            maxjob,
            current_groups
                .len()
                .try_into()
                .expect("You have generated a value outside of what C can handle"),
            current_groups.as_ptr(),
        )
    };
    if result.is_error == 1 {
        Err((result.error, result.value != 0))
    } else {
        Ok(result.value != 0)
    }
}

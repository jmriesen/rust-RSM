use std::sync::{Mutex, MutexGuard};

use crate::systab;

static SINGLETON: Mutex<()> = Mutex::new(());
//This is intended to be a light wriapper around the C api that handles most of the unsafe stuff.
pub struct GlobalGuard {
    guard: MutexGuard<'static, ()>,
}

impl Default for GlobalGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalGuard {
    pub fn new() -> Self {
        //The mutex becomes poisoned if the thread holding it panics
        //This will happen when tests fail
        SINGLETON.clear_poison();
        Self {
            guard: SINGLETON.lock().unwrap(),
        }
    }
    pub fn systab(&self) -> Option<&crate::SYSTAB> {
        unsafe { systab.as_ref() }
    }
}

impl Drop for GlobalGuard {
    fn drop(&mut self) {}
}

pub mod init {
    use super::*;
    use crate::{u_int, INIT_Start};
    use std::ffi::c_int;
    use std::ffi::CString;

    pub fn start(
        file_name: &str,
        jobs: u_int,
        gmb: u_int,
        rmb: u_int,
        add: u_int,
        _: &GlobalGuard,
    ) -> Result<(), c_int> {
        let file_name = CString::new(file_name).unwrap().into_raw();
        match unsafe { INIT_Start(file_name, jobs, gmb, rmb, add) } {
            0 => Ok(()),
            x => Err(x),
        }
    }
}

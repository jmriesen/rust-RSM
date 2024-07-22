use std::{
    ptr::{from_mut, from_ref},
    sync::{LockResult, Mutex, MutexGuard},
};

use crate::{ST_Get, ST_Init, ST_Kill, ST_Set, CSTRING, MVAR};

///controls access to table globals
static TABLE_MUTEX: Mutex<()> = Mutex::new(());

pub struct Table {
    _guard: LockResult<MutexGuard<'static, ()>>,
}

impl Table {
    pub fn new() -> Self {
        let temp = Self {
            _guard: TABLE_MUTEX.lock(),
        };
        unsafe { ST_Init() };
        temp
    }

    pub fn set(&mut self, var: &MVAR, data: &CSTRING) {
        unsafe { ST_Set(from_ref(var).cast_mut(), from_ref(data).cast_mut()) };
    }

    pub fn get(&self, var: &MVAR) -> Option<CSTRING> {
        let mut buf = [0; 65535];
        let len = unsafe { ST_Get(from_ref(var).cast_mut(), buf.as_mut_ptr()) };

        if len >= 0 {
            Some(CSTRING {
                buf,
                len: len as u16,
            })
        } else {
            None
        }
    }
    pub fn kill(&self, var: &MVAR) {
        unsafe { ST_Kill(from_ref(var).cast_mut()) };
    }
}

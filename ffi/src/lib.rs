/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
mod bindings;
pub mod interface;
pub mod run;
pub use bindings::*;
pub mod symbol_table;
use core::ptr::null_mut;
use std::{ffi::CString, path::Path};

//You need to grab this mutex before calling any function that affects
//C globals (almost all of the C functions)

impl Default for crate::bindings::PARTAB {
    fn default() -> Self {
        Self {
            job_table: null_mut(),
            src_ln: [0; 65535],
            vol: [null_mut()],
            jobtab: null_mut(),
            vol_fds: [0; 1],
            jnl_fds: [0; 1],
            debug: 0,
            strstk_start: null_mut(),
            strstk_last: null_mut(),
            varlst: null_mut(),
            checkonly: 0,
            errors: 0,
            sp: null_mut(),
            lp: null_mut(),
            ln: 0,
            src_var: MVAR {
                name: "TTTTT".try_into().unwrap(), // { var_q: 0 },
                volset: 0,
                uci: 0,
                slen: 0,
                key: [0; 256],
            },
        }
    }
}

impl Default for crate::bindings::SYSTAB {
    fn default() -> Self {
        Self {
            address: null_mut(),
            jobtab: null_mut(),
            maxjob: 0,
            sem_id: 0,
            historic: HISTORIC_DNOK as i32,
            precision: 0,
            max_tt: 0,
            tt: [TRANTAB {
                from_global: VAR_U { var_cu: [0; 32] },
                from_vol: 0,
                from_uci: 0,
                to_global: VAR_U { var_cu: [0; 32] },
                to_vol: 0,
                to_uci: 0,
            }; 64],
            start_user: 0,
            lockstart: null_mut(),
            locksize: 0,
            lockhead: null_mut(),
            lockfree: null_mut(),
            addoff: 0,
            addsize: 0,
            vol: [null_mut(); 1],
            last_blk_used: [0; 1],
        }
    }
}

impl var_u {
    pub fn as_array(&self) -> &[u8] {
        unsafe { &self.var_cu }
    }
}

pub fn shared_memory_key(file_path: &Path, system: i32) -> i32 {
    unsafe {
        libc::ftok(
            CString::new(file_path.as_os_str().as_encoded_bytes())
                .unwrap()
                .into_raw(),
            system,
        )
    }
}

pub fn shared_memory_id(file_path: &Path, system: i32) -> Result<i32, i32> {
    //TODO check that error handling is correct
    let temp = unsafe { libc::shmget(shared_memory_key(file_path, system), 0, 0) };
    if temp == -1 {
        Err(std::io::Error::last_os_error().raw_os_error().unwrap())
    } else {
        Ok(temp)
    }
}
pub struct SharedSegmentGuard(pub i32, pub *mut libc::c_void);

//TODO error handling
//Convert underlying logic to rust
pub fn util_share(file_path: &Path) -> SharedSegmentGuard {
    unsafe {
        UTIL_Share(
            file_path
                .as_os_str()
                .as_encoded_bytes()
                .as_ptr()
                .cast_mut()
                .cast(),
        )
    };
    SharedSegmentGuard(
        shared_memory_id(file_path, RSM_SYSTEM as i32).unwrap(),
        unsafe { systab.cast() },
    )
}

impl Drop for SharedSegmentGuard {
    fn drop(&mut self) {
        //TODO clean this up an handle cross platform support better

        use std::mem::MaybeUninit;
        let mut sbuf = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            //signal that the shared mem segment should be destroyed.
            libc::shmctl(dbg!(self.0), libc::IPC_RMID, &mut sbuf);
            //detaching shared meme segment.
            libc::shmdt(self.1);
        }
    }
}

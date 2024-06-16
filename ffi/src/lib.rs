mod bindings;
pub use bindings::*;
use core::ptr::null_mut;
use std::{ffi::CString, path::Path};
//mod glue_code;

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

pub fn shared_memory_id(file_path: &Path, system: i32) -> Result<i32, ()> {
    let temp = unsafe { libc::shmget(shared_memory_key(file_path, system), 0, 0) };
    if temp == -1 {
        Err(())
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
        let mut sbuf = libc::shmid_ds {
            shm_atime: 0,
            shm_cpid: 0,
            shm_ctime: 0,
            shm_dtime: 0,
            shm_internal: std::ptr::null_mut(),
            shm_lpid: 0,
            shm_nattch: 0,
            shm_perm: libc::ipc_perm {
                _key: 0,
                uid: 0,
                gid: 0,
                cuid: 0,
                cgid: 0,
                mode: 0,
                _seq: 0,
            },
            shm_segsz: 0,
        };
        unsafe {
            //signal that the shared mem segment should be destroyed.
            libc::shmctl(dbg!(self.0), libc::IPC_RMID, &mut sbuf);
            //detaching shared meme segment.
            libc::shmdt(self.1);
        }
    }
}

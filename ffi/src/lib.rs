mod bindings;
pub use bindings::*;
use core::ptr::null_mut;

impl Default for crate::bindings::PARTAB {
    fn default() -> Self {
        Self {
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
            ln: null_mut(),
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
            }; 8],
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

use crate::bindings::{u_char, u_short, PARTAB};

use std::ffi::{CStr, CString};

/// copies the compiled code back to C's comp and moves the comp/src pointer
/// This should be removed once the compile code has been converted from C to rust.
/// # Safety
/// This should only be called on the src/comp pointers that are provided by C.
#[allow(dead_code)]
unsafe fn sync_with_c(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    offset: usize,
    byte_code: &[u8],
) {
    use std::ptr::copy_nonoverlapping;
    // Copy over byte_code
    unsafe { copy_nonoverlapping((*byte_code).as_ptr(), *comp, byte_code.len()) }
    unsafe { *comp = (*comp).add(byte_code.len()) };

    //Move source ptr;
    unsafe { (*src) = (*src).add(offset) };
}

use super::*;

pub fn compile_string(value: &CStr) -> Vec<u8> {
    use std::iter::once;
    let bytes = value.to_bytes_with_nul();
    once(crate::bindings::OPSTR)
        //length does not include null bite.
        .chain(((bytes.len() - 1) as u_short).to_le_bytes())
        .chain(bytes.iter().copied())
        .collect()
}

#[allow(dead_code)]
pub fn parse_rust_to_c_ffi(
    src: &str,
    partab: &mut PARTAB,
    comp: &mut Vec<u8>,
    fnc: unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, *mut bindings::PARTAB),
) {
    let tmp = CString::new(src).unwrap();
    let mut source = tmp.as_ptr() as *mut u_char;
    let source_ptr = &mut source as *mut *mut u_char;

    let mut buff = [0u8; 200];
    let mut comp_c = &mut buff as *mut u_char;
    let comp_ptr = &mut comp_c as *mut *mut u_char;

    unsafe { fnc(source_ptr, comp_ptr, partab as *mut PARTAB) }

    let num = unsafe { comp_c.offset_from(buff.as_ptr()) };

    comp.extend(buff.into_iter().take(num as usize));
}

#[cfg(test)]
pub mod test {
    use std::sync::{LockResult, Mutex, MutexGuard};
    static GUARD: Mutex<()> = Mutex::new(());
    use super::*;

    #[allow(dead_code)]
    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }

    pub fn compile_c(
        src: &str,
        fn_c: unsafe extern "C" fn() -> (),
    ) -> (Vec<u8>, LockResult<MutexGuard<'_, ()>>) {
        use crate::bindings::{comp_ptr, partab, source_ptr, systab, SYSTAB};
        use std::io::Write;

        //TODO this is being leaked.
        let source = CString::new(dbg!(src)).unwrap();
        let source = source.into_raw() as *mut u8;

        //TODO Figure out how these different fields actually work.
        //Zeroing out for now so that I can avoid seg faults
        let mut sys_tab = SYSTAB::default();

        //TODO something about the value of 100 can cause the len to be calculated incorrectly.
        //attributing to some of the unsafe code in this test harness.
        //I think it should be fine for now, after all my end goal is to remove the unsafe C all together.
        const BUFFER_LEN: usize = 600;
        let mut compiled_original = [0u8; BUFFER_LEN];
        let _ = std::io::stdout().flush();
        let lock = GUARD.lock();

        let compile_stack_len = {
            unsafe { source_ptr = source };
            unsafe { comp_ptr = compiled_original.as_mut_ptr() };
            unsafe { partab = PARTAB::default() };
            unsafe { systab = &mut sys_tab as *mut SYSTAB };
            unsafe { fn_c() }
            unsafe { comp_ptr.offset_from(compiled_original.as_ptr()) }
        };

        (
            compiled_original[..compile_stack_len as usize].to_vec(),
            lock,
        )
    }
}

use crate::bindings::{HISTORIC_DNOK, MVAR, TRANTAB, VAR_U};
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

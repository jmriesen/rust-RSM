use crate::{
    bindings::{partab_struct, u_char, u_short, PARTAB},
    pest::Parser,
};
use pest::iterators::Pair;

use std::ffi::{CStr, CString};
pub unsafe fn sync_with_c(
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
pub fn parse_c_to_rust_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
    rule: Rule,
    parser: fn(Pair<Rule>, &mut partab_struct, &mut Vec<u8>),
) {
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
        .to_str()
        .unwrap();

    let variable = SyntaxParser::parse(rule, source).unwrap().next().unwrap();
    let offset = variable.as_str().len();
    let mut byte_code = vec![];
    parser(variable, unsafe { &mut *par_tab }, &mut byte_code);

    unsafe {
        sync_with_c(src, comp, offset, &byte_code);
    }
}

pub fn compile_string(value: &CStr) -> Vec<u8> {
    use std::iter::once;
    let bytes = value.to_bytes_with_nul();
    once(crate::bindings::OPSTR as u8)
        //length does not include null bite.
        .chain(((bytes.len() - 1) as u_short).to_le_bytes())
        .chain(bytes.iter().copied())
        .collect()
}

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
    use std::sync::Mutex;
    static guard: Mutex<()> = Mutex::new(());
    use super::*;

    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }

    pub fn compare_to_c(
        src: &str,
        rule: Rule,
        fun_r: fn(Pair<Rule>, &mut partab_struct, &mut Vec<u8>) -> (),
        fn_c: unsafe extern "C" fn() -> (),
    ) {
        use crate::bindings::{comp_ptr, source_ptr, systab, MVAR, SYSTAB, TRANTAB, VAR_U};
        use std::io::Write;

        //TODO this is being leaked.
        let source = CString::new(dbg!(src)).unwrap();
        let source = source.into_raw() as *mut u8;

        let mut par_tab = PARTAB {
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
                name: VAR_U { var_q: 0 },
                volset: 0,
                uci: 0,
                slen: 0,
                key: [0; 256],
            },
        };
        //TODO Figure out how these different fields actually work.
        //Zeroing out for now so that I can avoid seg faults
        let mut sys_tab = SYSTAB {
            address: null_mut(),
            jobtab: null_mut(),
            maxjob: 0,
            sem_id: 0,
            historic: crate::bindings::HISTORIC_DNOK as i32,
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
        };

        //TODO something about the value of 100 can cause the len to be calculated incorrectly.
        //attributing to some of the unsafe code in this test harness.
        //I think it should be fine for now, after all my end goal is to remove the unsafe C all together.
        const buffer_len: usize = 600;
        let mut compiled_original = [0u8; buffer_len];
        let _ = std::io::stdout().flush();
        let compile_stack_len = {
            let _lock = guard.lock();
            unsafe { source_ptr = source };
            unsafe { comp_ptr = compiled_original.as_mut_ptr() };
            unsafe { partab = par_tab.clone() };
            unsafe { systab = &mut sys_tab as *mut SYSTAB };
            unsafe { fn_c() }
            unsafe { comp_ptr.offset_from(compiled_original.as_ptr()) }
        };

        use core::ptr::null_mut;

        let mut byte_code = vec![];
        fun_r(
            dbg!(SyntaxParser::parse(rule, src))
                .unwrap()
                .next()
                .unwrap(),
            &mut par_tab,
            &mut byte_code,
        );

        assert_eq!(
            compiled_original[..compile_stack_len as usize],
            byte_code[..]
        );
        use crate::bindings::partab;
        unsafe { assert_eq!(any_as_u8_slice(&partab), any_as_u8_slice(&par_tab)) };
    }
}

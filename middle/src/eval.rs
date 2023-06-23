use super::*;
use crate::{
    bindings::{partab_struct, u_char, PARTAB},
    ffi::*,
    pest::Parser,
};
use pest::iterators::Pair;
use std::ffi::{CStr, CString};

pub fn parse_eval_ffi(src: &str, partab: &mut PARTAB, comp: &mut Vec<u8>) {
    parse_rust_to_c_ffi(src, partab, comp, crate::bindings::eval_temp)
}

#[no_mangle]
pub extern "C" fn parse_pattern_ffi(src: *mut *mut u_char, comp: *mut *mut u_char) {
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
        .to_str()
        .unwrap();
    let (offset, byte_code) = parse_pattern(source);
    unsafe {
        sync_with_c(src, comp, offset, &byte_code);
    }
}

fn parse_pattern(src: &str) -> (usize, Vec<u8>) {
    if let Ok(code) = SyntaxParser::parse(Rule::atom, src) {
        let code = code.as_str();
        let cstr = CString::new(code).unwrap();
        (code.len(), compile_string(&cstr))
    } else {
        //TODO I am ignoring errors for now.
        todo!()
    }
}

#[no_mangle]
pub unsafe extern "C" fn ncopy_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::Number, rust_ncopy)
}
fn rust_ncopy(number: Pair<Rule>, _partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let number = number.as_str();
    let sign = if number.chars().filter(|x| *x == '-').count() % 2 == 0 {
        ""
    } else {
        "-"
    };

    let mut number = number
        .trim_start_matches(['+', '-'])
        .trim_start_matches('0');
    if number.contains('.') {
        number = number.trim_end_matches('0').trim_end_matches('.');
    }
    let num = CString::new(format!("{}{}", sign, number)).unwrap();
    comp.extend(compile_string(&num))
}

#[cfg(test)]
pub mod test {
    use std::sync::Mutex;
    static guard: Mutex<()> = Mutex::new(());
    use super::*;
    use crate::{bindings::u_char, ffi::test::*};
    pub fn test_eval(src: &str) {
        use crate::bindings::{comp_ptr, source_ptr, MVAR, VAR_U};

        //TODO this is being leaked.
        let source = CString::new(dbg!(src)).unwrap();
        let mut source = source.into_raw() as *mut u8;

        let mut compiled_original = [0u8; 100];

        {
            let _lock = guard.lock();
            unsafe { source_ptr = source };
            unsafe { comp_ptr = compiled_original.as_mut_ptr() };
            unsafe { crate::bindings::eval() }
        }

        let mut compiled_new = [0u8; 100];
        let mut compile_new_ptr = compiled_new.as_mut_ptr();
        use core::ptr::null_mut;
        let mut par_tab_new = PARTAB {
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
        let partab_ptr = &mut par_tab_new;
        {
            let source_temp = &mut source as *mut *mut u_char;
            let comp_temp = &mut compile_new_ptr as *mut *mut u_char;
            use crate::bindings::eval_temp;
            unsafe { eval_temp(source_temp, comp_temp, partab_ptr as *mut PARTAB) }
        }
        assert_eq!(compiled_original, compiled_new);
        use crate::bindings::partab;
        unsafe { assert_eq!(any_as_u8_slice(&partab), any_as_u8_slice(&par_tab_new)) };
    }

    #[test]
    fn pattern_match() {
        test_eval("SomeString?.A");
        test_eval("SomeString?1.3A");
        test_eval("SomeString?.(8A,1(1N))");
        test_eval("SomeString?.2A");
        test_eval("SomeString?1.A");
    }
}

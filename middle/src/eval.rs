use std::ffi::{CStr, CString};

use crate::bindings::u_char;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pattern.pest"]
pub struct PatternParser;

#[no_mangle]
pub extern "C" fn parse_pattern(
    src: *mut *mut u_char,
    comp: *mut *mut u_char
){
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
    .to_str()
        .unwrap();
    if let Ok(code) = PatternParser::parse(Rule::atom, source) {
        let code = code.as_str();
        let cstr = CString::new(code).unwrap();
        let compiled = compile_string(&cstr);
        unsafe {std::ptr::copy_nonoverlapping::<u_char>(compiled.as_ptr(),*comp,compiled.len())}
        unsafe { *comp = (*comp).add(compiled.len()) };
        unsafe { (*src) = (*src).add(code.len()) };
    }
}
use crate::bindings::{u_short};
fn compile_string(value:&CStr)->Vec<u8>{
    use std::iter::once;
    let bytes = value.to_bytes_with_nul();
    once(crate::bindings::OPSTR as u8)
        //length does not include null bite.
        .chain(((bytes.len()-1) as u_short).to_le_bytes())
        .chain(bytes.iter().copied())
        .collect()
}

#[cfg(test)]
mod test{

    use crate::{bindings::u_char};
    fn test_eval(src: &str){
        use std::ffi::CString;
        use crate::bindings::source_ptr;
        use crate::bindings::comp_ptr;

        //TODO this is being leaked.
        let source = CString::new(dbg!(src)).unwrap();
        let mut source = source.into_raw() as *mut u8;

        let mut compiled_original = [0u8;1000];

        {
            unsafe { source_ptr = source };
            unsafe { comp_ptr = compiled_original.as_mut_ptr()};
            unsafe {crate::bindings::eval() }
        }

        let mut compiled_new = [0u8;1000];
        let mut compile_new_ptr = compiled_new.as_mut_ptr();
        use crate::bindings::{PARTAB,MVAR,VAR_U};
        use core::ptr::null_mut;
        let mut par_tab_new = PARTAB{
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
            src_var: MVAR{
                name: VAR_U{
                    var_q: 0
                },
                volset: 0,
                uci: 0,
                slen: 0,
                key: [0; 256],
            }
        };
        let partab_ptr = &mut par_tab_new;
        {
            let source_temp = &mut source as *mut *mut u_char;
            let comp_temp = &mut compile_new_ptr as *mut *mut u_char;
            use crate::bindings::eval_temp;
            unsafe {eval_temp(source_temp,comp_temp,partab_ptr as* mut PARTAB) }
        }
        assert_eq!(compiled_original,compiled_new);
        use crate::bindings::partab;
        unsafe{assert_eq!(any_as_u8_slice(&partab),any_as_u8_slice(&par_tab_new))};
    }
    #[test]
    fn pattern_match(){
        test_eval("SomeString?.A");
        test_eval("SomeString?1.3A");
        test_eval("SomeString?.(8A,1(1N))");
        test_eval("SomeString?.2A");
        test_eval("SomeString?1.A");
    }
    unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}


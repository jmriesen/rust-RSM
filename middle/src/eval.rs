use std::ffi::{CStr, CString};

use crate::bindings::{PARTAB,MVAR,VAR_U};

use crate::bindings::u_char;
use crate::ffi::*;

use pest::Parser;
use pest::iterators::Pair;

pub fn parse_eval_ffi(src:&str, partab: &mut PARTAB,comp:&mut Vec<u8>){
    let tmp = CString::new(src).unwrap();
    let mut source = tmp.as_ptr() as *mut u_char;
    let source_ptr = &mut source as *mut *mut u_char;

    let mut buff = [0u8;100];
    let mut comp_c = &mut buff as * mut u_char;
    let comp_ptr = &mut comp_c as *mut *mut u_char;

    unsafe {crate::bindings::eval_temp(source_ptr,comp_ptr,partab as* mut PARTAB) }

    let num = unsafe{comp_c.offset_from(buff.as_ptr())};

    comp.extend(buff.into_iter().take(num as usize));
}




#[derive(Parser)]
#[grammar = "pattern.pest"]
pub struct PatternParser;

#[no_mangle]
pub extern "C" fn parse_pattern_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char
){
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
    .to_str()
        .unwrap();
    let (offset,byte_code) = parse_pattern(source);
    unsafe{sync_with_c(src, comp, offset, &byte_code);}
}

fn parse_pattern(src: &str)->(usize,Vec<u8>){
    if let Ok(code) = PatternParser::parse(Rule::atom, src) {
        let code = code.as_str();
        let cstr = CString::new(code).unwrap();
        (code.len(),compile_string(&cstr))
    }else{
        //TODO I am ignoring errors for now.
        todo!()
    }
}

#[cfg(test)]
pub mod test{
    use super::*;
    use crate::{bindings::u_char,ffi::test::*};
    pub fn test_eval(src: &str){
        use std::ffi::CString;
        use crate::bindings::source_ptr;
        use crate::bindings::comp_ptr;

        //TODO this is being leaked.
        let source = CString::new(dbg!(src)).unwrap();
        let mut source = source.into_raw() as *mut u8;

        let mut compiled_original = [0u8;100];

        {
            unsafe { source_ptr = source };
            unsafe { comp_ptr = compiled_original.as_mut_ptr()};
            unsafe {crate::bindings::eval() }
        }

        let mut compiled_new = [0u8;100];
        let mut compile_new_ptr = compiled_new.as_mut_ptr();
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
}


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

    let mut buff = [0u8; 100];
    let mut comp_c = &mut buff as *mut u_char;
    let comp_ptr = &mut comp_c as *mut *mut u_char;

    unsafe { fnc(source_ptr, comp_ptr, partab as *mut PARTAB) }

    let num = unsafe { comp_c.offset_from(buff.as_ptr()) };

    comp.extend(buff.into_iter().take(num as usize));
}

#[cfg(test)]
pub mod test {
    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

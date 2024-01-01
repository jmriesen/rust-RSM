use crate::{bindings::partab_struct, ffi::*};
use std::ffi::CString;

/// TODO rerwrite this function
/// currently function both formats number into canonacl represntation and
/// compiles to [u8]. It should only do this first part.
/// Note in the C implemnation +9 is parsed as a unaray exprestion not a number.
pub fn ncopy(number: &str, _partab: &mut partab_struct, comp: &mut Vec<u8>) {
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
    if number.is_empty() {
        number = "0";
    }
    let num = CString::new(format!("{}{}", sign, number)).unwrap();
    comp.extend(compile_string(&num))
}

pub fn compile_string_literal(string: &str, comp: &mut Vec<u8>) {
    let string = string
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        //replace "" with " quote.
        .replace("\"\"", "\"");
    //strip off outer quotes.
    let inner = CString::new(string).unwrap();
    comp.extend(compile_string(&inner))
}

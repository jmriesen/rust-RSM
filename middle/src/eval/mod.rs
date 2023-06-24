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
    if number == ""{
        number = "0";
    }
    let num = CString::new(format!("{}{}", sign, number)).unwrap();
    comp.extend(compile_string(&num))
}
#[no_mangle]
pub unsafe extern "C" fn unary_op_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::UnaryOperator, unary_op)
}

fn unary_op(unaryExp : Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let mut unaryExp = unaryExp.into_inner();
    let op = unaryExp.next().unwrap();
    let op = match op.as_rule(){
        Rule::OPNOT   => bindings::OPNOT,
        Rule::OPPLUS  => bindings::OPPLUS,
        Rule::OPMINUS => bindings::OPMINUS,
        _ => unreachable!(),
    } as u8;

    let exp = unaryExp.next().unwrap();

    parse_rust_to_c_ffi(exp.as_str(), partab, comp, crate::bindings::atom_temp);

    comp.push(op);
}
#[no_mangle]
pub unsafe extern "C" fn parse_string_literal_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::String, literal)
}

fn literal(literal: Pair<Rule>, _partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let literal = literal.as_str()
        .strip_prefix("\"").unwrap()
        .strip_suffix("\"").unwrap()
    //replace "" with " quote.
        .replace("\"\"","\"");
    //strip off outer quotes.
    let inner = CString::new(literal).unwrap();
    comp.extend(compile_string(&inner))
}


#[cfg(test)]
pub mod test;

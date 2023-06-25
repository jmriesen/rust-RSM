use super::*;
use crate::{
    bindings::{partab_struct, u_char},
    ffi::*,
    pest::Parser, op_code::operator, localvar::parse_local_var,
};
use pest::iterators::Pair;
use std::ffi::{CStr, CString};

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
    if let Ok(code) = SyntaxParser::parse(Rule::Pattern, src) {
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

    atom(exp,partab,comp);

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

#[no_mangle]
pub unsafe extern "C" fn eval_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::Exp, eval)
}

pub fn eval(eval: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    dbg!(&eval);
    let mut eval = eval.into_inner();
    let exp = eval.next().unwrap();
    atom(exp,partab,comp);

    while let (Some(op),Some(exp)) = (eval.next(),eval.next()){
        let (_,op) = operator(op.as_str());
        match exp.as_rule(){
            Rule::Atom => {
                atom(exp,partab,comp);
            },
            Rule::Pattern => {
                let (_,byte_code) = parse_pattern(exp.as_str());
                comp.extend(byte_code);
            }
            _=> unreachable!()
        }

        comp.push(op as u8);
    }
}


pub fn atom(atom: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let atom = atom.into_inner().next().unwrap();

    //TODO will need to deal with inderection
    match atom.as_rule(){
        Rule::Variable => parse_local_var(atom, partab, comp),
        Rule::UnaryOperator => unary_op(atom, partab, comp),
        Rule::String => literal(atom,partab,comp),
        Rule::Number => rust_ncopy(atom, partab, comp),
        Rule::Exp => eval(atom, partab, comp),
        _=> unreachable!()
    }
}


#[cfg(test)]
pub mod test;

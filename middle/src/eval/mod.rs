use super::*;
use crate::dollar::intrinsic_var_op_code;
use crate::{
    bindings::{partab_struct, u_char},
    ffi::*,
    localvar::parse_local_var,
    op_code::operator,
};
use pest::iterators::Pair;
use std::ffi::CString;

fn pattern(pattern: Pair<Rule>, comp: &mut Vec<u8>) {
    let cstr = CString::new(pattern.as_str()).unwrap();
    comp.extend(compile_string(&cstr))
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
    if number.is_empty(){
        number = "0";
    }
    let num = CString::new(format!("{}{}", sign, number)).unwrap();
    comp.extend(compile_string(&num))
}

fn unary_op(unaryExp: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let mut unaryExp = unaryExp.into_inner();
    let op = unaryExp.next().unwrap();
    let op = match op.as_rule() {
        Rule::OPNOT => bindings::OPNOT,
        Rule::OPPLUS => bindings::OPPLUS,
        Rule::OPMINUS => bindings::OPMINUS,
        _ => unreachable!(),
    } as u8;

    let exp = unaryExp.next().unwrap();
    atom(exp, partab, comp);
    comp.push(op);
}

fn literal(literal: Pair<Rule>, _partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let literal = literal
        .as_str()
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        //replace "" with " quote.
        .replace("\"\"", "\"");
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
    let mut eval = eval.into_inner();
    atom(eval.next().unwrap(), partab, comp);

    while let (Some(op), Some(exp)) = (eval.next(), eval.next()) {
        match exp.as_rule() {
            Rule::Atom => atom(exp, partab, comp),
            Rule::Pattern => pattern(exp, comp),
            _ => unreachable!(),
        }

        operator(op, comp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn atom_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::Number, atom)
}

use crate::dollar::x_call;
pub fn atom(atom: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let atom = atom.into_inner().next().unwrap();

    //TODO will need to deal with inderection
    //TODO will need to deal dollar()
    match atom.as_rule() {
        Rule::Variable => parse_local_var(atom, partab, comp),
        Rule::UnaryOperator => unary_op(atom, partab, comp),
        Rule::String => literal(atom, partab, comp),
        Rule::Number => rust_ncopy(atom, partab, comp),
        Rule::Exp => eval(atom, partab, comp),
        Rule::IntrinsicVar => comp.push(intrinsic_var_op_code(atom)),
        Rule::Xcall => x_call(atom, partab, comp),
        x => {dbg!(x); unreachable!()},
    }
}

#[cfg(test)]
pub mod test;

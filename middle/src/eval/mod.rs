use super::*;
use crate::{
    bindings::partab_struct,
    dollar::intrinsic_var_op_code,
    ffi::*,
    function::intrinsic_function,
    localvar::{parse_local_var, VarTypes},
    op_code::operator,
    routine::extrinsic_function,
};
use pest::iterators::Pair;
use std::ffi::CString;

fn pattern(pattern: Pair<Rule>, comp: &mut Vec<u8>) {
    comp.extend(compile_string(&CString::new(pattern.as_str()).unwrap()));
}

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

/// TODO deprecate
fn literal(literal: Pair<Rule>, _partab: &mut partab_struct, comp: &mut Vec<u8>) {
    compile_string_literal(literal.as_str(), comp);
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

use crate::dollar::x_call;
pub fn atom(atom: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let atom = atom.into_inner().next().unwrap();

    match atom.as_rule() {
        Rule::Variable => parse_local_var(atom, partab, comp, VarTypes::Eval),
        Rule::UnaryOperator => unary_op(atom, partab, comp),
        Rule::String => literal(atom, partab, comp),
        Rule::Number => ncopy(atom.as_str(), partab, comp),
        Rule::Exp => eval(atom, partab, comp),
        Rule::IntrinsicVar => comp.push(intrinsic_var_op_code(atom)),
        Rule::Xcall => x_call(atom, partab, comp),
        Rule::IntrinsicFunction => intrinsic_function(atom, partab, comp),
        Rule::ExtrinsicFunction => extrinsic_function(atom, partab, comp, true),
        Rule::AtomInd => indirect_atom(atom, partab, comp, IndAtomContext::Eval),
        _ => {
            dbg!(atom);
            unreachable!()
        }
    }
}
pub enum IndAtomContext {
    Eval,
    Close,
}
impl IndAtomContext {
    fn op_code(self) -> u8 {
        match self {
            Self::Eval => crate::bindings::INDEVAL,
            Self::Close => crate::bindings::INDCLOS,
        }
    }
}

pub fn indirect_atom(
    atom: Pair<Rule>,
    partab: &mut partab_struct,
    comp: &mut Vec<u8>,
    context: IndAtomContext,
) {
    let atom = atom.into_inner().next().unwrap();
    self::atom(atom, partab, comp);
    comp.push(context.op_code());
}

#[cfg(test)]
pub mod test;

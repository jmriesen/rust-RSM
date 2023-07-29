use super::*;
use crate::{
    bindings::{partab_struct, u_char, var_u},
    eval::{atom, eval},
    ffi::parse_c_to_rust_ffi,
};
use pest::iterators::Pair;

#[no_mangle]
pub unsafe extern "C" fn parse_local_var_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::Variable, parse_local_var_eval);
}

fn parse_local_var_eval(variable: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    parse_local_var(variable,partab,comp,VarTypes::Eval);
}
pub enum VarTypes{
    Eval,
    Build,
    BuildNullable,
    For,
}

impl VarTypes{
    fn code(self)->u8{
        use VarTypes::*;
        (match self{
            Eval => crate::bindings::OPVAR,
            Build => crate::bindings::OPMVAR,
            BuildNullable => crate::bindings::OPMVARN,
            For => crate::bindings::CMFORSET,
        }) as u8
    }
}

pub fn parse_local_var(variable: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>,var_type:VarTypes) {
    let mut variable = variable.into_inner();

    let descriptor = variable.next().unwrap();
    let (variableType, name) = parse_var_descriptor(descriptor, partab, comp);

    let subscripts = variable.next().unwrap();
    let number_of_subscripts = subscripts
        .into_inner()
        .inspect(|i| eval(i.clone(), partab, comp))
        .count();

    comp.push(var_type.code());
    match variableType {
        crate::bindings::TYPVARGBL | crate::bindings::TYPVARNAM => {
            comp.push((variableType | number_of_subscripts as u32) as u8);
        }
        _ => {
            comp.push(variableType as u8);
            comp.push(number_of_subscripts as u8);
        }
    }

    if let Some(name) = name {
        comp.extend(name.as_array());
    }
}

fn parse_var_descriptor(
    descriptor: Pair<Rule>,
    partab: &mut partab_struct,
    comp: &mut Vec<u8>,
) -> (u32, Option<var_u>) {
    let variableType = match descriptor.as_rule() {
        Rule::GlobalVariable => crate::bindings::TYPVARGBL,
        Rule::LocalVariable => crate::bindings::TYPVARNAM,
        Rule::NakedVariable => crate::bindings::TYPVARNAKED,
        Rule::GlobalUciVariable => crate::bindings::TYPVARGBLUCI,
        Rule::GlobalUciEnvVariable => crate::bindings::TYPVARGBLUCIENV,
        Rule::IndirectVariable => crate::bindings::TYPVARIND,
        _ => unimplemented!(),
    };

    let (exps, name): (Vec<_>, Vec<_>) = descriptor
        .into_inner()
        .partition(|x| x.as_rule() == Rule::Atom);
    let mut exps = exps.into_iter().peekable();

    if variableType == crate::bindings::TYPVARIND {
        atom(exps.next().unwrap(), partab, comp);
        if exps.peek().is_some(){
            comp.push(crate::bindings::INDEVAL as u8);
        }else{
            comp.push(crate::bindings::INDMVAR as u8);
        }
    }

    for exp in exps {
        atom(exp, partab, comp);
        if variableType == crate::bindings::TYPVARIND{
        }
    }

    let name = name.iter().map(|x| x.as_str().into()).next();
    (variableType, name)
}

#[cfg(test)]
mod test {
    use crate::eval::test::test_eval;
    use rstest::rstest;

    #[rstest]
    #[case("SomeString")]
    #[case("^SomeString")]
    #[case("^[atom]varName")]
    #[case("^|atom|varName")]
    #[case("^[atom1,atom2]varName")]
    #[case("SomeString(sub1)")]
    #[case("^SomeString(sub1)")]
    #[case("^(sub1)")]
    #[case("^|atom|varName(sub1)")]
    #[case("^[atom1,atom2]varName(sub1)")]
    #[case("SomeString(sub1,sub2)")]
    #[case("^SomeString(sub1,sub2)")]
    #[case("^|atom|varName(sub1),sub2")]
    #[case("^[atom1,atom2]varName(sub1,sub2)")]
    #[case("@atom@(sub1,sub2)")]
    #[case("@varName")]
    //TODO index
    fn parse_variable(#[case] var: &str) {
        test_eval(var);
    }
}

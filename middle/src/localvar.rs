use super::*;
use crate::eval::atom;
use crate::eval::eval;
use crate::{
    bindings::{partab_struct, u_char},
    ffi::parse_c_to_rust_ffi,
};
use pest::iterators::Pair;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn parse_local_var_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
) {
    parse_c_to_rust_ffi(src, comp, par_tab, Rule::Variable, parse_local_var)
}

pub fn parse_local_var(variable: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let mut variable = variable.into_inner();

    let descriptor = variable.next().unwrap();
    let (variableType, name) = parse_var_descriptor(descriptor, partab, comp);

    let subscripts = variable.next().unwrap();
    let number_of_subscripts = subscripts
        .into_inner()
        .inspect(|i| eval(i.clone(), partab, comp))
        .count();

    comp.push(crate::bindings::OPVAR as u8);
    match variableType {
        crate::bindings::TYPVARGBL | crate::bindings::TYPVARNAM => {
            comp.push((variableType | number_of_subscripts as u32) as u8);
        }
        _ => {
            comp.push(variableType as u8);
            comp.push(number_of_subscripts as u8);
        }
    }

    if let Some(name) = name.map(|x| {
        x.into_bytes_with_nul()
            .into_iter()
            .chain(std::iter::repeat(0))
            .take(32)
    }) {
        comp.extend(name);
    }
}

pub fn parse_var_descriptor(
    descriptor: Pair<Rule>,
    partab: &mut partab_struct,
    comp: &mut Vec<u8>,
) -> (u32, Option<CString>) {
    let variableType = match descriptor.as_rule() {
        Rule::GlobalVariable => crate::bindings::TYPVARGBL,
        Rule::LocalVariable => crate::bindings::TYPVARNAM,
        Rule::NakedVariable => crate::bindings::TYPVARNAKED,
        Rule::GlobalUciVariable => crate::bindings::TYPVARGBLUCI,
        Rule::GlobalUciEnvVariable => crate::bindings::TYPVARGBLUCIENV,
        _ => unimplemented!(),
    };

    let (exps, name): (Vec<_>, Vec<_>) = descriptor
        .into_inner()
        .partition(|x| x.as_rule() == Rule::Atom);

    for exp in exps {
        atom(exp, partab, comp);
    }

    let name = name
        .iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .next();
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
    //TODO there seems to be special handling for intrinsics
    //#[case("$J")]
    #[case("SomeString(sub1)")]
    #[case("^SomeString(sub1)")]
    #[case("^(sub1)")]
    #[case("^|atom|varName(sub1)")]
    //TODO there seems to be wired handling for these variableTypes
    //#[case("^[atom1,atom2]varName(sub1)")]
    #[case("SomeString(sub1,sub2)")]
    #[case("^SomeString(sub1,sub2)")]
    #[case("^|atom|varName(sub1),sub2")]
    //TODO there seems to be wired handling for these variableTypes
    //#[case("^[atom1,atom2]varName(sub1,sub2)")]
    //TODO indirection
    //TODO index
    fn parse_variable(#[case] var: &str) {
        test_eval(var);
    }
}

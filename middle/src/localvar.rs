use libc::c_short;
use pest::Parser;
use pest::iterators::Pair;
use crate::bindings::u_char;
use crate::bindings::partab_struct;
use core::ffi::CStr;
use std::ffi::CString;
use crate::ffi::sync_with_c;
use crate::eval::parse_eval_ffi;

#[derive(Parser)]
#[grammar = "localvar.pest"]
pub struct LocalVar;

#[no_mangle]
pub unsafe extern "C" fn parse_local_var_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
)->c_short{
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
    .to_str()
        .unwrap();

    let variable = LocalVar::parse(Rule::Variable, source).unwrap().next().unwrap();
    let offset = variable.as_str().len();
    let mut byte_code = vec![];
    parse_local_var(variable,&mut *par_tab,&mut byte_code);

    unsafe{sync_with_c(src, comp, offset, &byte_code);}
    byte_code.len() as c_short
}

pub fn parse_local_var(variable:Pair<Rule>, partab: &mut partab_struct,comp:&mut Vec<u8>){
    let mut variable = variable.into_inner();

    let variable_descriptor = variable.next().unwrap();
    let variableType = match variable_descriptor.as_rule() {
        Rule::GlobalVariable =>crate::bindings::TYPVARGBL,
        Rule::LocalVariable =>crate::bindings::TYPVARNAM,
        Rule::NakedVariable =>crate::bindings::TYPVARNAKED,
        Rule::GlobalUciVariable => crate::bindings::TYPVARGBLUCI,
        Rule::GlobalUciEnvVariable =>crate::bindings::TYPVARGBLUCIENV,
        _=> unimplemented!()
    };

    let (exps,name):(Vec<_>,Vec<_>) = variable_descriptor
        .into_inner()
        .partition(|x| x.as_rule() == Rule::exp);

    for exp in exps{
        parse_eval_ffi(exp.as_str(),partab,comp);
    }
    let subscripts = variable.next()
        .into_iter();

    let number_of_subscripts = subscripts
        .flat_map(|x| x.into_inner())
        .inspect(|i| parse_eval_ffi(i.as_str(),partab,comp))
        .count();

    comp.push(crate::bindings::OPVAR as u8);
    match variableType {
        crate::bindings::TYPVARGBL| crate::bindings::TYPVARNAM =>{
            comp.push((variableType|number_of_subscripts as u32) as u8);
        },
        _ =>{
            comp.push(variableType as u8);
            comp.push(number_of_subscripts as u8);
        }
    }

    if let Some(name) = name.iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .map(|x| {
            x
                .into_bytes_with_nul()
                .into_iter()
                .chain(std::iter::repeat(0))
                .take(32)
        }
        ).next(){
            comp.extend(name);
        }
}

#[cfg(test)]
mod test{
    use crate::eval::test::test_eval;
    use rstest::rstest;

    #[rstest]
    #[case("SomeString")]
    #[case("^SomeString")]
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
    fn parse_variable(#[case] var:&str){
        test_eval(var);
    }


}

use super::*;
use crate::bindings::{partab_struct, var_u};
use pest::iterators::Pair;

use crate::{eval::eval, localvar::{parse_local_var,VarTypes}};

pub fn extrinsic_function(fn_call: Pair<'_, Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>,return_expected:bool) {
    let mut fn_call = fn_call.into_inner().peekable();

    //TODO indirection
    //TODO offsets
    let tag = fn_call.next_if(|x| x.as_rule() == Rule::Tag);
    let routine = fn_call.next_if(|x| x.as_rule() == Rule::Routine);
    let mut args = fn_call
        .map(|x| {
            let arg_type = x.as_rule();
            let arg = x.into_inner().next();
            match arg_type {
                //TODO indirection
                Rule::VarUndefined => comp.push(crate::bindings::VARUNDF as u8),
                Rule::ByVal => eval(arg.unwrap(), partab, comp),
                Rule::ByRef => {
                    parse_local_var(arg.unwrap(), partab, comp,VarTypes::Eval);
                    comp.push(crate::bindings::NEWBREF as u8);
                }
                _ => todo!(),
            }
        })
        .count() as u8
        + return_expected.then_some(129).unwrap_or(0);

    //TODO I think this is a bug in the C source code, (parse:134 missing args--;)
    //but for right now the C source is my source of truth.
    if args !=0 && !return_expected{
        args+=1;
    }

    let opcode = match (tag.is_some(), routine.is_some()) {
        (true, false) => crate::bindings::CMDOTAG,
        (false, true) => crate::bindings::CMDOROU,
        (true, true) => crate::bindings::CMDORT,
        _ => unreachable!(),
    };
    comp.push(opcode as u8);

    if let Some(routine) = routine.map(|x| x.as_str()) {
        let tag = var_u::from(routine);
        comp.extend(tag.as_array());
    }
    if let Some(tag) = tag.map(|x| x.as_str()) {
        let tag = var_u::from(tag);
        comp.extend(tag.as_array());
    }
    comp.push(args);
}

#[cfg(test)]
mod test {
    use crate::eval::test::test_eval;
    use rstest::rstest;

    #[rstest]
    #[case("$$tag")]
    #[case("$$tag")]
    #[case("$$tag^rou")]
    #[case("$$^rou")]
    #[case("$$tag(89)")]
    #[case("$$tag(89,87)")]
    #[case("$$tag(,87)")]
    //#[case("$$tag(.name)")]
    //#[case("$$tag(89,.name)")]

    fn extrinsic_call(#[case] fn_call: &str) {
        test_eval(fn_call);
    }
}

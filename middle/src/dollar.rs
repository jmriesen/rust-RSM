use super::*;
use crate::bindings::partab_struct;
use pest::iterators::Pair;

pub fn intrinsic_var_op_code(var: Pair<'_, Rule>) -> u8 {
    (match var.into_inner().next().unwrap().as_rule() {
        Rule::VarD => crate::bindings::VARD,
        Rule::VarEC => crate::bindings::VAREC,
        Rule::VarES => crate::bindings::VARES,
        Rule::VarET => crate::bindings::VARET,
        Rule::VarH => crate::bindings::VARH,
        Rule::VarI => crate::bindings::VARI,
        Rule::VarJ => crate::bindings::VARJ,
        Rule::VarK => crate::bindings::VARK,
        Rule::VarP => crate::bindings::VARP,
        Rule::VarQ => crate::bindings::VARQ,
        Rule::VarR => crate::bindings::VARR,
        Rule::VarS => crate::bindings::VARS,
        Rule::VarST => crate::bindings::VARST,
        Rule::VarSY => crate::bindings::VARSY,
        Rule::VarT => crate::bindings::VART,
        Rule::VarX => crate::bindings::VARX,
        Rule::VarY => crate::bindings::VARY,
        _ => unreachable!(),
    }) as u8
}
use crate::eval::eval;
pub fn x_call(x_call: Pair<'_, Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let mut x_call = x_call.into_inner();
    let name = x_call.next().unwrap();

    use crate::pest::Parser;
    let default = SyntaxParser::parse(Rule::Exp, "\"\"")
        .unwrap()
        .next()
        .unwrap();

    let arg1 = x_call.next().unwrap_or(default.clone());
    let arg2 = x_call.next().unwrap_or(default);
    eval(arg1, partab, comp);
    eval(arg2, partab, comp);

    let code = (match name.as_rule() {
        Rule::XcDIR => crate::bindings::XCDIR,
        Rule::XcHOST => crate::bindings::XCHOST,
        Rule::XcFILE => crate::bindings::XCFILE,
        Rule::XcERR => crate::bindings::XCERR,
        Rule::XcOPC => crate::bindings::XCOPC,
        Rule::XcSIG => crate::bindings::XCSIG,
        Rule::XcSPA => crate::bindings::XCSPA,
        Rule::XcVER => crate::bindings::XCVER,
        Rule::XcZWR => crate::bindings::XCZWR,
        Rule::XcE => crate::bindings::XCE,
        Rule::XcPAS => crate::bindings::XCPAS,
        Rule::XcV => crate::bindings::XCV,
        Rule::XcX => crate::bindings::XCX,
        Rule::XcXRSM => crate::bindings::XCXRSM,
        Rule::XcSETENV => crate::bindings::XCSETENV,
        Rule::XcGETENV => crate::bindings::XCGETENV,
        Rule::XcROUCHK => crate::bindings::XCROUCHK,
        Rule::XcFORK => crate::bindings::XCFORK,
        Rule::XcIC => crate::bindings::XCIC,
        Rule::XcWAIT => crate::bindings::XCWAIT,
        Rule::XcDEBUG => crate::bindings::XCDEBUG,
        Rule::XcCOMP => crate::bindings::XCCOMP,
        _ => unreachable!(),
    }) as u8;

    comp.push(code);
}

#[cfg(test)]
mod test {
    use crate::ffi::test::compile_c;
    use rstest::rstest;
    use crate::bindings;
    use crate::compile;

    #[rstest]
    #[case("$D")]
    #[case("$device")]
    #[case("$EC")]
    #[case("$ecode")]
    #[case("$ES")]
    #[case("$estack")]
    #[case("$ET")]
    #[case("etrap")]
    #[case("$H")]
    #[case("$horolog")]
    #[case("$I")]
    #[case("$io")]
    #[case("$J")]
    #[case("$job")]
    #[case("$K")]
    #[case("$key")]
    #[case("$P")]
    #[case("$principal")]
    #[case("$Q")]
    #[case("$quit")]
    #[case("$R")]
    #[case("$reference")]
    #[case("$S")]
    #[case("$storage")]
    #[case("$ST")]
    #[case("$stack")]
    #[case("$SY")]
    #[case("$system")]
    #[case("$T")]
    #[case("$test")]
    #[case("$X")]
    #[case("$Y")]
    fn intrinsic_var(#[case] var: &str) {
        {
            let source_code = format!("w {}", var);
            let (orignal, _lock) = compile_c(&source_code, bindings::parse);

            assert_eq!(orignal, compile(&source_code));
        }
    }

    /*
    #[rstest]
    #[case("$C(temp)")]
    #[case("$J(temp,temp2)")]
    fn intrinsic_var_with_sub(#[case] var: &str) {
        test_eval(var);
    }
    */

    #[rstest]
    #[case("$&%DIRECTORY")]
    #[case("$&%HOST")]
    #[case("$&%FILE")]
    #[case("$&%ERRMSG")]
    #[case("$&%OPCOM")]
    #[case("$&%SIGNAL")]
    #[case("$&%SPAWN")]
    #[case("$&%VERSION")]
    #[case("$&%ZWRITE")]
    #[case("$&E")]
    #[case("$&PASCHK")]
    #[case("$&V")]
    #[case("$&X")]
    #[case("$&XRSM")]
    #[case("$&%SETENV")]
    #[case("$&%GETENV")]
    #[case("$&%ROUCHK")]
    #[case("$&%FORK")]
    #[case("$&%IC")]
    #[case("$&%WAIT")]
    #[case("$&DEBUG")]
    #[case("$&%COMPRESS")]
    fn x_call(#[case] call: &str) {
        use core::iter::repeat;
        for num in 1..=2 {
            let args = repeat("10").take(num).collect::<Vec<_>>().join(",");
            let source_code = format!("w {}({})", call, args);
            let (orignal, _lock) = compile_c(&source_code, bindings::parse);
            let temp = compile(&source_code);

            assert_eq!(orignal, temp);
        }
    }
}

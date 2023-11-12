#[cfg(test)]
mod test {
    use crate::{bindings, test_compile_command, ffi::test::compile_c};
    use rstest::rstest;

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

            assert_eq!(orignal, test_compile_command(&source_code));
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
            let temp = test_compile_command(&source_code);

            assert_eq!(orignal, temp);
        }
    }
}

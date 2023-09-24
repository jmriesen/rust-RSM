use super::*;
use crate::bindings::{partab_struct, var_u};

use crate::{
    localvar::{VarTypes},
};

#[cfg(test)]
mod test {
    use crate::ffi::test::compile_c;
    use rstest::rstest;
    use crate::bindings;
    use crate::compile;

    #[rstest]
    #[case("$$tag()")]
    #[case("$$tag^rou()")]
    #[case("$$^rou()")]
    #[case("$$tag(89)")]
    #[case("$$tag(89,87)")]
    #[case("$$tag(,87)")]
    #[case("$$tag(,,,,)")]
    #[case("$$tag(.name)")]
    #[case("$$tag(89,.name)")]
    fn extrinsic_call(#[case] fn_call: &str) {
        let source_code = format!("w {}", fn_call);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);
        let temp = compile(&source_code);

        assert_eq!(orignal, temp);
    }
}

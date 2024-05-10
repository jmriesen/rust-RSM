#[cfg(test)]
mod test {
    use crate::{test_harness::test::compile_c, test_compile_command};
    use rstest::rstest;

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
        let (orignal, _lock) = compile_c(&source_code, ffi::parse);
        let temp = test_compile_command(&source_code);

        assert_eq!(orignal, temp);
    }
}
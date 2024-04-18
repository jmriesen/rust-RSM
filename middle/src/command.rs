#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{test_compile_command, test_harness::test::compile_c};

    #[test]
    fn multiple_commands() {
        let source_code = "w 9 w 8 w 7 w 6 w 5 w 4 w 3";
        let (orignal, _lock) = compile_c(source_code, ffi::parse);

        assert_eq!(orignal, test_compile_command(source_code));
    }

    //TODO comand with no args at the end of a line.
    //TODO use external parser to handle it.
    #[rstest]
    #[case("b  ")]
    #[case("b  b  b  ")]
    #[case("b:something  ")]
    #[case("b 1")]
    #[case("b 1,2")]
    #[case("b 1,2 b 2")]
    #[case("c 1,2")]
    #[case("c @1")]
    #[case("d  ")]
    #[case("d tag")]
    #[case("d tag:12")]
    #[case("d tag(90):12,tag^rou:0")]
    #[case("e  ")]
    #[case("e  w 1")]
    #[case("f  ")]
    #[case("f  b  b  ")]
    #[case("f  f  b  ")]
    #[case("f  f  f  b  ")]
    #[case("f x=1 ")]
    #[case("f x=1:2 ")]
    #[case("f x=1:2:3 ")]
    #[case("f x=1,2:3,4:5:6 ")]
    fn command_test(#[case] source_code: &str) {
        let (orignal, _lock) = compile_c(source_code, ffi::parse);
        let temp = test_compile_command(source_code);

        assert_eq!(orignal, temp);
    }

    #[rstest]
    #[case("w 90")]
    #[case("w !")]
    #[case("w #")]
    #[case("w ?9")]
    #[case("w ?@temp")]
    #[case("w 1,#,!,?@temp")]
    fn write_command(#[case] source_code: &str) {
        let (orignal, _lock) = compile_c(source_code, ffi::parse);
        let temp = test_compile_command(source_code);

        assert_eq!(orignal, temp);
    }
}

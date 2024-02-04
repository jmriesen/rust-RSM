use crate::{bindings, ffi::test::*, test_compile_command};
use rstest::rstest;

#[rstest]
#[case("SomeString?.A")]
#[case("SomeString?1.3A")]
#[case("SomeString?.(8A,1(1N))")]
#[case("SomeString?.2A")]
#[case("SomeString?1.A")]
#[case("SomeString?@var")]
fn parse_pattern(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, bindings::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}

#[rstest]
#[case("9")]
#[case("10000000")]
#[case("00000001")]
#[case("0.1")]
#[case("0.00001")]
#[case("0.0")]
#[case(".0000000")]
#[case("0.0000000")]
#[case("0.000010000")]
#[case("00000000.00000000")]
//TODO implement HISTORIC_EOK
//#[case("1E100")]
//#[case("1E-100")]
//#[case("1.90E-100")]
fn parse_number(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, bindings::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}

#[rstest]
#[case("\"Some string\"")]
#[case("\"Some numbers89097\"")]
#[case("\" string with quote\"\"quote\"\" some text\"")]
fn parse_string(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, bindings::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}

#[rstest]
#[case("+-+-+-+-+-234")]
#[case("-10000")]
#[case("--45")]
#[case("'45")]
#[case("-'-45")]
fn parse_unary_exp(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, bindings::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}
#[rstest]
#[case("98+9")]
#[case("-98\\var(7,9)")]
#[case("98+(something+9)")]
fn parse_binary(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, bindings::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}

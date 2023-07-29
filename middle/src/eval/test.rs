use super::*;
use crate::{ffi::test::*};

pub fn test_eval(src: &str) {
    compare_to_c(src,Rule::Exp, eval, crate::bindings::eval);
}

use rstest::rstest;

#[test]
fn pattern_match() {
    test_eval("SomeString?.A");
    test_eval("SomeString?1.3A");
    test_eval("SomeString?.(8A,1(1N))");
    test_eval("SomeString?.2A");
    test_eval("SomeString?1.A");
    test_eval("SomeString?@var");
}

#[rstest]
#[case("9")]
#[case("+-+-+-+-+-234")]
#[case("10000000")]
#[case("-10000")]
#[case("--45")]
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
    test_eval(num);
}

#[rstest]
#[case("\"Some string\"")]
#[case("\"Some numbers89097\"")]
#[case("\" string with quote\"\"quote\"\"\" some text\"")]
fn parse_string(#[case] num: &str) {
    test_eval(num);
}

#[rstest]
#[case("-98")]
#[case("'98")]
#[case("+98")]
fn parse_unary_exp(#[case] num: &str) {
    test_eval(num);
}

#[rstest]
#[case("98+9")]
#[case("-98\\var(7,9)")]
#[case("98+(something+9)")]
fn parse_exp(#[case] num: &str) {
    test_eval(num);
}

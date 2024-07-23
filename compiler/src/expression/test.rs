/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
use crate::{test_compile_command, test_harness::test::*};
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
    let (orignal, _lock) = compile_c(&source_code, ffi::parse);

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
    let (orignal, _lock) = compile_c(&source_code, ffi::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}

#[rstest]
#[case("\"Some string\"")]
#[case("\"Some numbers89097\"")]
#[case("\" string with quote\"\"quote\"\" some text\"")]
fn parse_string(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, ffi::parse);

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
    let (orignal, _lock) = compile_c(&source_code, ffi::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}
#[rstest]
#[case("98+9")]
#[case("-98\\var(7,9)")]
#[case("98+(something+9)")]
fn parse_binary(#[case] num: &str) {
    let source_code = format!("w {}", num);
    let (orignal, _lock) = compile_c(&source_code, ffi::parse);

    assert_eq!(orignal, test_compile_command(&source_code));
}

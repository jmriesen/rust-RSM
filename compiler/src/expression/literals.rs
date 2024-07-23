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
use std::ffi::CString;

use crate::test_harness::compile_string;

/// TODO rerwrite this function
/// currently function both formats number into canonacl represntation and
/// compiles to [u8]. It should only do this first part.
/// Note in the C implemnation +9 is parsed as a unaray exprestion not a number.
pub fn ncopy(number: &str, comp: &mut Vec<u8>) {
    //NOTE the C code also does bounds checking to prevent overflow.
    //This should not be needed since I am using a vector.
    let sign = if number.chars().filter(|x| *x == '-').count() % 2 == 0 {
        ""
    } else {
        "-"
    };

    let mut number = number
        .trim_start_matches(['+', '-'])
        .trim_start_matches('0');
    if number.contains('.') {
        number = number.trim_end_matches('0').trim_end_matches('.');
    }
    if number.is_empty() {
        number = "0";
    }
    let num = CString::new(format!("{}{}", sign, number)).unwrap();
    comp.extend(compile_string(&num))
}

pub fn compile_string_literal(string: &str, comp: &mut Vec<u8>) {
    let string = string
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        //replace "" with " quote.
        .replace("\"\"", "\"");
    //strip off outer quotes.
    let inner = CString::new(string).unwrap();
    comp.extend(compile_string(&inner))
}

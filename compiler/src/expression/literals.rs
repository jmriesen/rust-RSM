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
    use std::str::FromStr;
    use value::{Number, Value};
    let value = Value::from_str(number).expect("String was too large");
    let number = Value::from(Number::from(value));
    let bytes = Value::from(number).content().to_vec();
    let str_c = unsafe { CString::from_vec_unchecked(bytes) };
    comp.extend(compile_string(&str_c))
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

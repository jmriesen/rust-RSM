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
#[cfg(test)]
mod test {

    use crate::{test_compile_command, test_harness::test::compile_c};
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
            let (orignal, _lock) = compile_c(&source_code);

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
            let (orignal, _lock) = compile_c(&source_code);
            let temp = test_compile_command(&source_code);

            assert_eq!(orignal, temp);
        }
    }
}

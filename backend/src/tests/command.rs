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
    use rstest::rstest;

    use crate::test::test_compile_command;
    use ffi::parse::parse;

    #[test]
    fn multiple_commands() {
        let source_code = "w 9 w 8 w 7 w 6 w 5 w 4 w 3";
        let orignal = parse(&source_code);

        assert_eq!(orignal, test_compile_command(source_code));
    }

    //TODO comand with no args at the end of a line.
    //TODO use external parser to handle it.
    #[rstest]
    #[case("b  ")]
    #[case("b  b  b  ")]
    #[case("b:something  ")]
    //#[case("b 1")]
    #[case("b 1,2")]
    //#[case("b 1,2 b 2")]
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
        let orignal = parse(&source_code);
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
        let orignal = parse(&source_code);
        let temp = test_compile_command(source_code);

        assert_eq!(orignal, temp);
    }
}

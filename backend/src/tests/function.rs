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
    use core::ops::RangeInclusive;

    use crate::test::test_compile_command;
    use ffi::parse::parse;
    use rstest::rstest;

    #[test]
    fn select_test() {
        let source_code = "w $s(1:2,3:4,5:6)";
        let orignal = parse(source_code);
        let temp = test_compile_command(source_code);

        assert_eq!(orignal, temp);
    }
    #[rstest]
    #[case("View","V",2..=4)]
    #[case("Text","T",1..=1)]
    #[case("Translate","TR",2..=3)]
    #[case("Find","F",2..=3)]
    #[case("fnumber","Fn",2..=3)]
    #[case("Random","R",1..=1)]
    #[case("Reverse","Re",1..=1)]
    #[case("Piece","P",2..=4)]
    #[case("Justify","J",2..=3)]
    #[case("extract","E",1..=3)]
    #[case("ascii","a",1..=2)]
    #[case("char","c",1..=8)]
    //TODO test upper bounds of Char
    //currenrly getting segfale problby would need to increase the buffer.
    #[case("char","c",50..=50)]
    #[case("length","l",1..=2)]
    #[case("Stack","st",1..=2)]
    fn intrinsic_fun(
        #[case] full: &str,
        #[case] abbreviated: &str,
        #[case] range: RangeInclusive<usize>,
    ) {
        use core::iter::repeat;
        for val in range {
            let args = repeat("11011").take(val).collect::<Vec<_>>().join(",");

            {
                let source_code = format!("w ${}({})", full, args);
                let orignal = parse(&source_code);

                assert_eq!(orignal, test_compile_command(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let orignal = parse(&source_code);
                let temp = test_compile_command(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }

    #[rstest]
    #[case("Data","D",1..=1)]
    #[case("Get","G",1..=2)]
    #[case("increment","i",1..=2)]
    #[case("name","na",1..=2)]
    #[case("order","o",1..=2)]
    #[case("query","q",1..=2)]
    #[case("Next","n",1..=1)]
    #[case("Qlength","QL",1..=1)]
    #[case("QSUBSCRIPT","Qs",2..=2)]
    fn intrinsic_variable_fn(
        #[case] full: &str,
        #[case] abbreviated: &str,
        #[case] range: RangeInclusive<usize>,
    ) {
        use core::iter::repeat;
        for val in range {
            let args = repeat("variable").take(val).collect::<Vec<_>>().join(",");
            {
                let source_code = format!("w ${}({})", full, args);
                let orignal = parse(&source_code);

                assert_eq!(orignal, test_compile_command(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let orignal = parse(&source_code);
                let temp = test_compile_command(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }
}

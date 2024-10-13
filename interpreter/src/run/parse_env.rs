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
mod tests {
    use ffi::{run::Error, CSTRING, LABEL_BLOCK, UCIS, UCI_TAB, VAR_U};

    #[test]
    fn null_env() {
        let env = c"";
        let ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        assert_eq!(crate::bindings::run::parse_env(env, &ucis), Ok(1));
    }

    #[test]
    fn env_not_found_is_an_error() {
        let env = c"env name";
        let ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        assert_eq!(crate::bindings::run::parse_env(env, &ucis), Err(Error(2)));
    }

    fn copy_into(des: &mut ffi::VAR_U, src: &std::ffi::CStr) {
        let bytes = src.to_bytes();
        let raw = unsafe { &mut des.var_cu };
        let len = bytes.len().min(raw.len());
        raw[..len].copy_from_slice(&bytes[..len]);
    }

    #[test]
    fn env_is_found() {
        let mut ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        let env_one = c"env one";
        let env_two = c"env two";

        copy_into(&mut ucis[1].name, env_one);
        copy_into(&mut ucis[2].name, env_two);
        assert_eq!(crate::bindings::run::parse_env(env_one, &ucis), Ok(2));
        assert_eq!(crate::bindings::run::parse_env(env_two, &ucis), Ok(3));
    }

    #[test]
    fn env_name_is_trucated() {
        let mut ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        let max_var_len_plus_0 = c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let max_var_len_plus_1 = c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        copy_into(&mut ucis[1].name, max_var_len_plus_0);
        assert_eq!(
            crate::bindings::run::parse_env(max_var_len_plus_0, &ucis),
            Ok(2)
        );
        assert_eq!(
            crate::bindings::run::parse_env(max_var_len_plus_1, &ucis),
            Ok(2)
        );
    }
}

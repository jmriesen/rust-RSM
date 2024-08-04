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
use super::var_u::VarU;
use crate::key::Key;
use ffi::u_char;

#[derive(Clone)]
pub struct MVar {
    pub name: VarU,
    volset: u_char,
    uci: u_char,
    pub key: Key,
}

#[cfg_attr(test, mutants::skip)]
impl std::fmt::Debug for MVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = String::from_utf8(self.name.0.as_array().into()).unwrap();

        let mut builder = f.debug_struct("MVar");
        builder
            .field("name", &name)
            .field("key", &self.key)
            .field("volume set", &self.volset)
            .field("uci", &self.uci)
            .finish()
    }
}

impl std::fmt::Display for MVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.key.is_empty() {
            write!(f, "{}", self.name.0)
        } else {
            write!(
                f,
                "{}{}",
                self.name.0,
                //TODO handle non Ascii case
                String::from_utf8(self.key.string_key()).unwrap()
            )
        }
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use super::{MVar, VarU};
    use crate::{key::Key, symbol_table::var_u::helpers::var_u, value::Value};
    use arbitrary::Arbitrary;
    use ffi::{UCI_IS_LOCALVAR, VAR_U};

    #[must_use]
    pub fn var_m(name: &str, values: &[&str]) -> MVar {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = Key::new(&values).unwrap();

        //TODO All M vars are currently assumed to be local  have a vol set of 0;
        MVar {
            name: var_u(name),
            volset: Default::default(),
            uci: UCI_IS_LOCALVAR as u8,
            key,
        }
    }

    #[cfg_attr(test, mutants::skip)]
    impl<'a> Arbitrary<'a> for MVar {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let name: [u8; 32] = u.arbitrary()?;
            if name.is_ascii() && name.contains(&0) {
                Ok(MVar {
                    name: VarU(VAR_U { var_cu: name }),
                    volset: 0,
                    uci: 0,
                    //TODO implement arbitrary for key.
                    key: Key::empty(),
                })
            } else {
                Err(arbitrary::Error::IncorrectFormat)
            }
        }
    }

    impl MVar {
        #[must_use] pub fn into_cmvar(self) -> ffi::MVAR {
            let mut key = [0; 256];
            key[..self.key.len()].copy_from_slice(self.key.raw_keys());
            ffi::MVAR {
                name: self.name.0,
                volset: self.volset,
                uci: self.uci,
                slen: self.key.len() as u8,
                key,
            }
        }
    }
}

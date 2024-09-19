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
use crate::key::{self, NullableKey};
use ffi::{u_char, UCI_IS_LOCALVAR};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MVar<Key: key::Key> {
    pub name: VarU,
    volset: u_char,
    uci: u_char,
    pub key: Key,
}

impl<Key: key::Key> std::fmt::Display for MVar<Key> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.key.borrow().is_empty() {
            write!(f, "{}", self.name.0)
        } else {
            write!(
                f,
                "{}{}",
                self.name.0,
                //TODO Consider how this would work with page files
                String::from_utf8_lossy(&self.key.borrow().string_key())
            )
        }
    }
}

impl<Key: key::Key> MVar<Key> {
    pub fn new(name: VarU, key: Key) -> Self {
        //TODO All M vars are currently assumed to be local and have a vol set of 0;
        Self {
            name,
            key,
            uci: UCI_IS_LOCALVAR as u8,
            volset: 0,
        }
    }
    pub fn to_nullable(self) -> MVar<NullableKey> {
        MVar::<NullableKey> {
            name: self.name,
            volset: self.volset,
            uci: self.uci,
            key: self.key.into(),
        }
    }

    pub fn copy_new_key<NewKey: key::Key>(&self, key: NewKey) -> MVar<NewKey> {
        MVar::<NewKey> {
            name: self.name.clone(),
            volset: self.volset,
            uci: self.uci,
            key,
        }
    }

    #[cfg_attr(test, mutants::skip)]
    pub fn util_string_m_var(&self) -> Vec<u8> {
        assert_eq!(self.uci, UCI_IS_LOCALVAR as u8, "Unimplemented");
        assert_eq!(self.volset, 0, "Unimplemented");

        let mut string = vec![];
        string.extend(self.name.contents());
        let key = self.key.borrow();
        if !key.is_empty() {
            string.extend(key.string_key());
        }
        string
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::helpers::var_m_nullable;
    #[test]
    fn no_subscripts() {
        assert_eq!(format!("{}", var_m_nullable("foo", &[])), "foo");
    }

    #[test]
    fn subscripts() {
        assert_eq!(
            format!("{}", var_m_nullable("foo", &["sub1"])),
            "foo(\"sub1\")"
        );
        assert_eq!(
            format!("{}", var_m_nullable("foo", &["sub1", "sub2"])),
            "foo(\"sub1\",\"sub2\")"
        );
        assert_eq!(format!("{}", var_m_nullable("foo", &["3"])), "foo(3)");
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use super::*;
    use crate::{
        key::{NonNullableKey, NullableKey},
        value::Value,
        var_u::helpers::var_u,
    };
    use arbitrary::Arbitrary;
    use ffi::UCI_IS_LOCALVAR;

    #[must_use]
    pub fn var_m_nullable(name: &str, values: &[&str]) -> MVar<NullableKey> {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = NullableKey::new(&values).unwrap();

        MVar::new(var_u(name), key)
    }
    #[must_use]
    pub fn var_m(name: &str, values: &[&str]) -> MVar<NonNullableKey> {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = NonNullableKey::new(&values).unwrap();

        MVar::new(var_u(name), key)
    }

    #[cfg_attr(test, mutants::skip)]
    impl<'a, Key> Arbitrary<'a> for MVar<Key>
    where
        Key: Arbitrary<'a> + crate::key::Key,
    {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            Ok(MVar::new(VarU::arbitrary(u)?, Key::arbitrary(u)?))
        }
    }

    impl<Key: crate::key::Key> MVar<Key> {
        #[must_use]
        pub fn into_cmvar(self) -> ffi::MVAR {
            let (slen, key) = self.key.borrow().clone().into_ckey();
            ffi::MVAR {
                name: self.name.0,
                volset: self.volset,
                uci: self.uci,
                slen,
                key,
            }
        }
    }
}

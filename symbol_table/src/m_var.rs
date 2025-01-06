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
use serde::{Deserialize, Serialize};

use crate::{
    key::{self, KeyBound},
    value::Value,
};
const UCI_IS_LOCALVAR: u8 = 255;
use std::ffi::c_uchar;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MVar<Key: key::KeyType> {
    pub name: VarU,
    volset: c_uchar,
    uci: c_uchar,
    pub key: Key,
}

impl<Key: key::KeyType> std::fmt::Display for MVar<Key> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = Value::from(self.clone());
        write!(f, "{}", String::from_utf8_lossy(value.content()))
    }
}

impl<Key: key::KeyType> From<MVar<Key>> for Value {
    fn from(var: MVar<Key>) -> Self {
        assert_eq!(var.uci, UCI_IS_LOCALVAR, "Unimplemented");
        assert_eq!(var.volset, 0, "Unimplemented");

        let mut value = vec![];
        value.extend(var.name.contents());
        let key = var.key.borrow();
        if !key.is_empty() {
            key.push_to_vec(&mut value);
        }
        value
            .as_slice()
            .try_into()
            .expect("The longest variable + longest subscript should still fit in a value")
    }
}

impl<Key: key::KeyType> MVar<Key> {
    pub fn new(name: VarU, key: Key) -> Self {
        //TODO All M vars are currently assumed to be local and have a vol set of 0;
        Self {
            name,
            key,
            uci: UCI_IS_LOCALVAR,
            volset: 0,
        }
    }
    pub fn to_nullable(self) -> MVar<KeyBound> {
        MVar::<KeyBound> {
            name: self.name,
            volset: self.volset,
            uci: self.uci,
            key: self.key.into(),
        }
    }

    pub fn copy_new_key<NewKey: key::KeyType>(&self, key: NewKey) -> MVar<NewKey> {
        MVar::<NewKey> {
            name: self.name.clone(),
            volset: self.volset,
            uci: self.uci,
            key,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::test_helpers::var_m_nullable;
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

#[cfg_attr(test, mutants::skip)]
#[cfg(feature = "ffi")]
impl<Key: crate::key::KeyType> MVar<Key> {
    #[must_use]
    pub fn into_cmvar(self) -> ffi::MVAR {
        let (slen, key) = self.key.borrow().clone().into_ckey();
        ffi::MVAR {
            name: self.name.as_c(),
            volset: self.volset,
            uci: self.uci,
            slen,
            key,
        }
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use crate::{
        key::{Key, KeyBound},
        value::Value,
        var_u::test_helpers::var_u,
    };

    #[must_use]
    pub fn var_m_nullable(name: &str, values: &[&str]) -> MVar<KeyBound> {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = KeyBound::new(&values).unwrap();

        MVar::new(var_u(name), key)
    }
    #[must_use]
    pub fn var_m(name: &str, values: &[&str]) -> MVar<Key> {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = Key::new(&values).unwrap();

        MVar::new(var_u(name), key)
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use super::{MVar, VarU};
    use arbitrary::Arbitrary;

    #[cfg_attr(test, mutants::skip)]
    impl<'a, Key> Arbitrary<'a> for MVar<Key>
    where
        Key: Arbitrary<'a> + crate::key::KeyType,
    {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            Ok(MVar::new(VarU::arbitrary(u)?, Key::arbitrary(u)?))
        }
    }
}

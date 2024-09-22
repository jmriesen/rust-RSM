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

#![allow(clippy::module_name_repetitions)]
mod format;
mod internal;

use crate::value::Value;
use format::IntermediateRepresentation;

pub trait Key:
    std::borrow::Borrow<NullableKey> + Clone + Into<NullableKey> + PartialEq + Eq
{
}
impl Key for NullableKey {}
impl Key for NonNullableKey {}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NonNullableKey(NullableKey);

impl std::borrow::Borrow<NullableKey> for NonNullableKey {
    fn borrow(&self) -> &NullableKey {
        &self.0
    }
}
impl From<NonNullableKey> for NullableKey {
    fn from(value: NonNullableKey) -> Self {
        value.0
    }
}

impl NonNullableKey {
    pub fn new<'a>(values: impl IntoIterator<Item = &'a Value> + Clone) -> Result<Self, Error> {
        if values.clone().into_iter().any(|x| x == &Value::empty()) {
            Err(Error::SubKeyContainsNull)
        } else {
            Ok(Self(NullableKey::new(values)?))
        }
    }
}

/// Stores a list of keys.
//TODO Key max length is `MAX_KEY_SIZE` so I should be able to replace this with an array
#[derive(Eq, PartialEq, Clone)]
pub struct NullableKey(Vec<u8>);
impl NullableKey {
    pub fn new<'a>(values: impl IntoIterator<Item = &'a Value>) -> Result<Self, Error> {
        let mut key = Self(Vec::new());
        for value in values {
            key = key.push(value)?;
        }
        Ok(key)
    }

    //TODO consider replacing with Display
    #[must_use]
    pub fn string_key(&self) -> Vec<u8> {
        let mut out_put = vec![b'('];
        let mut iter = self.iter().map(IntermediateRepresentation::from);

        if let Some(sub_key) = iter.next() {
            sub_key.push_external_fmt(&mut out_put, true);
        }

        for sub_key in iter {
            out_put.push(b',');
            sub_key.push_external_fmt(&mut out_put, true);
        }

        out_put.push(b')');
        out_put
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    //Note I should probably remove this at some point.
    //It currently assumes there is at least one key in storage.
    #[must_use]
    pub fn key_extract(&self, quote_strings: bool) -> Vec<u8> {
        IntermediateRepresentation::from(self.iter().next().unwrap()).external_fmt(quote_strings)
    }

    #[must_use]
    pub fn iter(&self) -> Iter {
        Iter { tail: &self.0[..] }
    }
}

#[cfg_attr(test, mutants::skip)]
impl std::fmt::Debug for NullableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().map(Value::from))
            .finish()
    }
}

//This lint seems to be a false positive.
#[allow(clippy::into_iter_without_iter)]
impl<'a> IntoIterator for &'a NullableKey {
    type IntoIter = Iter<'a>;
    type Item = SubKey<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

//Represents one segment of a key
//If we have the Mvar x("a","b") "a" is one segment of the key ("a","b").
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct SubKey<'a>(&'a [u8]);
pub struct Iter<'a> {
    tail: &'a [u8],
}

impl<'a> From<SubKey<'a>> for Value {
    fn from(value: SubKey<'a>) -> Self {
        let mut data = Vec::new();
        IntermediateRepresentation::from(value).push_external_fmt(&mut data, false);
        Value::try_from(&data[..]).expect("max key len is < max Value len")
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    SubscriptToLarge,
    SubKeyContainsNull,
    KeyToLarge,
    SubKeyIsNull,
}

#[cfg(all(feature = "ffi", any(test, feature = "fuzzing")))]
pub mod a_b_testing;

#[cfg(test)]
mod tests {
    use super::*;
    use format::{MAX_INT_SEGMENT_SIZE, MAX_SUB_LEN};
    use internal::MAX_KEY_SIZE;

    fn generate_value(pattern: &str, count: usize) -> Value {
        pattern.repeat(count).as_str().try_into().unwrap()
    }

    #[test]
    fn subscript_max_size() {
        let result = NullableKey::new([&generate_value("a", MAX_SUB_LEN)]);
        assert!(result.is_ok());
    }
    #[test]
    fn subscript_that_is_to_large() {
        let result = NullableKey::new([&generate_value("a", MAX_SUB_LEN + 1)]);
        assert_eq!(result, Err(Error::SubscriptToLarge));
    }

    #[test]
    fn key_max_size() {
        let result = NullableKey::new([
            &generate_value("a", MAX_SUB_LEN),
            //NOTE -4 to account for the 2 null terminators + 2 type markers
            &generate_value("a", MAX_KEY_SIZE - MAX_SUB_LEN - 4),
        ]);
        assert!(result.is_ok());
    }

    #[test]
    fn key_that_is_to_large() {
        let result = NullableKey::new([
            &generate_value("a", MAX_SUB_LEN),
            &generate_value("a", MAX_SUB_LEN),
        ]);
        assert_eq!(result, Err(Error::KeyToLarge));
    }

    #[test]
    fn error_if_string_contains_null() {
        let result = NullableKey::new([&"a\0b".try_into().unwrap()]);
        assert_eq!(result, Err(Error::SubKeyContainsNull));
    }

    #[test]
    fn non_terminal_null() {
        let key = NullableKey::new(&[Value::empty()]).expect("trailing null is fine");
        let result = key.push(&"a".try_into().unwrap());
        assert_eq!(result, Err(Error::SubKeyIsNull));
    }

    #[test]
    fn build_key_int_to_large() {
        //NOTE I tried to put the mutants::skip attribute on 'MAX_INT_SEGMENT_SIZE'
        //but mutants were still being generated
        assert_eq!(MAX_INT_SEGMENT_SIZE, 63);
        let key = NullableKey::new([&"1"
            .repeat(MAX_INT_SEGMENT_SIZE + 1)
            .as_str()
            .try_into()
            .unwrap()])
        .unwrap();
        assert!(matches!(
            key.iter().next().unwrap().into(),
            IntermediateRepresentation::String(_)
        ));
    }

    #[test]
    fn key_cmp() {
        let keys: [NullableKey; 7] = ["", "-9.9", "-9", "0", "9", "9.9", "string"]
            .map(|x| NullableKey::new([&x.try_into().unwrap()]).unwrap());
        for [a, b] in keys.array_windows() {
            assert!(a < b);
        }
    }
}

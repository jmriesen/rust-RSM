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
#![allow(dead_code)]

mod format;
mod internal;
use format::IntermediateRepresentation;

use crate::value::Value;

/// Stores a list of keys.
//TODO Key max length is `MAX_KEY_SIZE` so I should be able to replace this with a array
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
        let mut keys = self
            .iter()
            .map(|x| IntermediateRepresentation::from_key_ref(x).external_fmt(true))
            .peekable();
        let non_empty = keys.peek().is_some();

        for key in keys {
            out_put.extend(key);
            out_put.push(b',');
        }
        if non_empty {
            //change the last ',' to a ')'.
            *out_put.last_mut().unwrap() = b')';
        } else {
            out_put.push(b')');
        }
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
        IntermediateRepresentation::from_key_ref(self.iter().next().unwrap())
            .external_fmt(quote_strings)
    }

    #[must_use]
    pub fn iter(&self) -> Iter {
        Iter { tail: &self.0[..] }
    }
}

//This lint seems to be a false positive.
#[allow(clippy::into_iter_without_iter)]
impl<'a> IntoIterator for &'a NullableKey {
    type IntoIter = Iter<'a>;
    type Item = Segment<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

//represents one segment of a key
//If we have the Mvar x("a","b")
//"a" is one segment of the key ("a","b").
//TODO consider making this private.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Segment<'a>(&'a [u8]);
pub struct Iter<'a> {
    tail: &'a [u8],
}

impl<'a> From<Segment<'a>> for Value {
    fn from(value: Segment<'a>) -> Self {
        Value::try_from(&IntermediateRepresentation::from_key_ref(value).external_fmt(false)[..])
            .expect("max key len is < max Value len")
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    SubscriptToLarge,
    SubKeyContainsNull,
    KeyToLarge,
    SubKeyIsNull,
}

#[cfg_attr(test, mutants::skip)]
impl std::fmt::Debug for NullableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("key");
        let value = String::from_utf8(self.string_key());
        match value {
            Ok(string) => builder.field("utf8", &string),
            Err(_) => builder.field("raw", &self.0),
        };
        builder.finish()
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod a_b_testing;

#[cfg(test)]
mod tests {
    use super::*;
    use ffi::{MAX_KEY_SIZE, MAX_SUB_LEN};
    use internal::MAX_INT_SEGMENT_SIZE;

    fn generate_value(pattern: &str, count: u32) -> Value {
        pattern.repeat(count as usize).as_str().try_into().unwrap()
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
            IntermediateRepresentation::from_key_ref(key.iter().next().unwrap()),
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

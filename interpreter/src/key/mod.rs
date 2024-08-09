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

mod internal;
use internal::ParsedKey;

use crate::value::Value;

/// Stores a list of keys.
//TODO Key max length is `MAX_KEY_SIZE` so I should be able to replace this with a array
#[derive(Eq, PartialEq, Clone)]
pub struct Key(Vec<u8>);
impl Key {
    pub fn new<'a>(values: impl IntoIterator<Item = &'a Value>) -> Result<Self, Error> {
        let mut key = Self(Vec::new());
        for value in values {
            key = key.push(value)?;
        }
        Ok(key)
    }

    #[must_use]
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn string_key(&self) -> Vec<u8> {
        let mut out_put = vec![b'('];
        let mut keys = self
            .iter()
            .map(|x| ParsedKey::from_key_ref(x).external_fmt(true))
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

    #[must_use]
    #[cfg(any(test, feature = "fuzzing"))]
    pub fn raw_keys(&self) -> &[u8] {
        &self.0[..]
    }

    //Note I should probably remove this at some point.
    //It currently assumes there is at least one key in storage.
    #[must_use]
    pub fn key_extract(&self, quote_strings: bool) -> Vec<u8> {
        ParsedKey::from_key_ref(self.iter().next().unwrap()).external_fmt(quote_strings)
    }

    #[must_use]
    pub fn iter(&self) -> Iter {
        Iter { tail: &self.0[..] }
    }
}

//This lint seems to be a false positive.
#[allow(clippy::into_iter_without_iter)]
impl<'a> IntoIterator for &'a Key {
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
        Value::try_from(&ParsedKey::from_key_ref(value).external_fmt(false)[..])
            .expect("max key len is < max Value len")
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    SubscriptToLarge,
    ContainsNull,
    KeyToLarge,
}

#[cfg_attr(test, mutants::skip)]
impl std::fmt::Debug for Key {
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

#[cfg_attr(test, mutants::skip)]
#[cfg(any(test, feature = "fuzzing"))]
pub mod a_b_testing {

    use crate::value::Value;
    use arbitrary::Arbitrary;
    use ffi::{
        symbol_table::{build_key, extract_key, string_key},
        ERRMLAST, ERRZ1, ERRZ5,
    };

    use super::{Error, Key};

    impl<'a> Arbitrary<'a> for Key {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let sub_keys = Vec::<Value>::arbitrary(u)?;
            Key::new(sub_keys.iter()).map_err(|_| arbitrary::Error::IncorrectFormat)
        }
    }

    //TODO all of these should be revamped to work on arrays of keys.
    pub fn build(value: &Value) {
        let key = super::Key::new([value]);
        let result = key.map(|x| x.0).map_err(|x| match x {
            Error::SubscriptToLarge => -((ERRZ1 + ERRMLAST) as i16),
            Error::ContainsNull => -((ERRZ5 + ERRMLAST) as i16),
            _ => unreachable!(),
        });
        assert_eq!(result, build_key(&value.clone().into_cstring()));
    }

    //TODO push key creation up to calling code.
    pub fn extract(string: &Value) -> Result<(), Error> {
        let key = super::Key::new([string])?;
        assert_eq!(key.key_extract(false), extract_key(key.raw_keys()).unwrap());
        Ok(())
    }

    //TODO push key creation up to calling code.
    pub fn string(keys: &[Value]) -> Result<(), Error> {
        let key_list = Key::new(keys)?;
        assert_eq!(
            key_list.string_key(),
            string_key(&key_list.0[..], i32::max_value())
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use a_b_testing::string;
    use ffi::{MAX_KEY_SIZE, MAX_SUB_LEN};
    use internal::MAX_INT_SEGMENT_SIZE;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("")]
    #[case(".")]
    #[case("1.")]
    #[case("test string")]
    #[case("0")]
    #[case("10")]
    #[case("10E")]
    #[case("10.4")]
    #[case(".4")]
    #[case("10.4E")]
    #[case("10.0")]
    #[case("010")]
    #[case("10.5.")]
    #[case("-")]
    #[case("-.")]
    #[case("-10")]
    #[case("-10E")]
    #[case("-10.4")]
    #[case("-.4")]
    #[case("-4.")]
    #[case("-10.4E")]
    #[case("-10.0")]
    #[case("-010")]
    #[case("-10.5.")]
    fn build_a_b_test_cases(#[case] input: &str) {
        super::a_b_testing::build(&input.try_into().unwrap());
        super::a_b_testing::extract(&input.try_into().unwrap())
            .expect("all of the test strings should produce valid keys");
    }

    #[test]
    fn subscript_max_size() {
        let result = Key::new([&"a"
            .repeat(MAX_SUB_LEN as usize)
            .as_str()
            .try_into()
            .unwrap()]);
        assert!(result.is_ok());
    }
    #[test]
    fn subscript_that_is_to_large() {
        let result = Key::new([&"a"
            .repeat(MAX_SUB_LEN as usize + 1)
            .as_str()
            .try_into()
            .unwrap()]);
        assert_eq!(result, Err(Error::SubscriptToLarge));
    }

    #[test]
    fn key_max_size() {
        let result = Key::new([
            &"a".repeat(MAX_SUB_LEN as usize)
                .as_str()
                .try_into()
                .unwrap(),
            //NOTE -4 to account for the 2 null terminators + 2 type markers
            &"a".repeat((MAX_KEY_SIZE - MAX_SUB_LEN - 4) as usize)
                .as_str()
                .try_into()
                .unwrap(),
        ]);
        assert!(result.is_ok());
    }

    #[test]
    fn key_that_is_to_large() {
        let result = Key::new([
            &"a".repeat(MAX_SUB_LEN as usize)
                .as_str()
                .try_into()
                .unwrap(),
            &"a".repeat(MAX_SUB_LEN as usize)
                .as_str()
                .try_into()
                .unwrap(),
        ]);
        assert_eq!(result, Err(Error::KeyToLarge));
    }

    #[test]
    fn error_if_string_contains_null() {
        let result = Key::new([&"a\0b".try_into().unwrap()]);
        assert_eq!(result, Err(Error::ContainsNull));
    }

    #[test]
    fn build_key_int_to_large() {
        //NOTE I tried to put the mutants::skip attribute on 'MAX_INT_SEGMENT_SIZE'
        //but mutants were still being generated
        assert_eq!(MAX_INT_SEGMENT_SIZE, 63);
        let key = Key::new([&"1"
            .repeat(MAX_INT_SEGMENT_SIZE + 1)
            .as_str()
            .try_into()
            .unwrap()])
        .unwrap();
        //TODO find a better way of testing this
        assert!(matches!(
            ParsedKey::from_key_ref(Segment(key.raw_keys())),
            ParsedKey::String(_)
        ));
    }

    #[rstest]
    #[case(&["only"])]
    #[case(&[""])]
    #[case(&["\""])]
    #[case(&["9"])]
    #[case(&["-9"])]
    #[case(&["-9.0"])]
    #[case(&["f","s"])]
    #[case(&["","s","9","-9"])]
    fn key_extract_string(#[case] raw_keys: &[&str]) {
        let keys = raw_keys
            .iter()
            .map(|x| (*x).try_into().unwrap())
            .collect::<Vec<_>>();
        matches!(string(&keys), Ok(()));
    }

    #[test]
    fn key_cmp() -> Result<(), Error> {
        let keys: [Key; 7] = ["", "-9.9", "-9", "0", "9", "9.9", "string"]
            .map(|x| Key::new([&x.try_into().unwrap()]).unwrap());
        for [a, b] in keys.array_windows() {
            assert!(a < b);
        }
        Ok(())
    }
}

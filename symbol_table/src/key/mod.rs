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

//TODO Key max length is `MAX_KEY_SIZE` so I should be able to replace this with an array

/// Keys represent a sequence of subscript that can be used to **store a value** withing a variable.
/// In the variable foo(1,"subscript","bar"), (1,"subscript","bar") is the Key.
///
/// There internal format has been optimized for sorting.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Key(KeyBound);

/// Keys represent a sequence of subscript that can be used to **specify a bound** withing a variable.
/// If the final subscript in a KeyBound is "", the key will be treated as a lower bound when going
/// forwards, and an upper bound while going backwards.
#[derive(Eq, PartialEq, Clone)]
pub struct KeyBound(Vec<u8>);

/// Represents one segment of a key.
/// If we have the MVar foo("a","b") "a" is one segment of the key ("a","b").
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct SubKey<'a>(&'a [u8]);
pub struct Iter<'a> {
    tail: &'a [u8],
}

/// Errors that can occur while trying to create a key.
#[derive(Debug, PartialEq)]
pub enum Error {
    SubscriptToLarge,
    SubKeyContainsNull,
    KeyToLarge,
    SubKeyIsNull,
}

pub mod conversions {
    use super::*;

    /// KeyType represents is used to represent either type of Key and is used as a genetic type bound
    /// to reduce code duplication.
    ///
    /// Mostly this type is used as a way of converting Keys into KeyBounds.
    /// The BTree API bound API kind of forces me to use store/interact with everything using the
    /// KeyBound type.
    pub trait KeyType:
        std::borrow::Borrow<KeyBound> + Clone + Into<KeyBound> + PartialEq + Eq
    {
    }
    impl KeyType for KeyBound {}
    impl KeyType for Key {}

    impl std::borrow::Borrow<KeyBound> for Key {
        fn borrow(&self) -> &KeyBound {
            &self.0
        }
    }
    impl From<Key> for KeyBound {
        fn from(value: Key) -> Self {
            value.0
        }
    }

    impl<'a> From<SubKey<'a>> for Value {
        fn from(value: SubKey<'a>) -> Self {
            IntermediateRepresentation::from(value)
                .external_fmt(false)
                .as_slice()
                .try_into()
                .expect("max key len is < max Value len")
        }
    }

    //This lint seems to be a false positive.
    #[allow(clippy::into_iter_without_iter)]
    impl<'a> IntoIterator for &'a KeyBound {
        type IntoIter = Iter<'a>;
        type Item = SubKey<'a>;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
}
pub use conversions::KeyType;

impl Key {
    pub fn new<'a>(values: impl IntoIterator<Item = &'a Value> + Clone) -> Result<Self, Error> {
        if values.clone().into_iter().any(|x| x == &Value::empty()) {
            Err(Error::SubKeyIsNull)
        } else {
            Ok(Self(KeyBound::new(values)?))
        }
    }

    pub const fn empty() -> Self {
        Self(KeyBound::empty())
    }
}

impl KeyBound {
    pub fn new<'a>(values: impl IntoIterator<Item = &'a Value>) -> Result<Self, Error> {
        let mut key = Self(Vec::new());
        for value in values {
            key = key.push(value)?;
        }
        Ok(key)
    }

    /// Appends self to the provided Vec.
    /// The data is appended in the external format.
    /// The format is (<subscript1>,<subscript2> ...)
    ///
    /// NOTE we are using a Vec<u8> rather then a string since the key could contain non UTF-8
    /// characters
    pub fn push_to_vec(&self,destination:&mut Vec<u8>) {
        destination.push(b'(');

        let mut iter = self.iter().map(IntermediateRepresentation::from);
        if let Some(sub_key) = iter.next() {
            sub_key.push_external_fmt(destination, true);
        }
        for sub_key in iter {
            destination.push(b',');
            sub_key.push_external_fmt(destination, true);
        }

        destination.push(b')');
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
    pub fn iter(&self) -> Iter {
        Iter { tail: &self.0[..] }
    }

    pub const fn empty() -> Self {
        Self(Vec::new())
    }
}

#[cfg_attr(test, mutants::skip)]
impl std::fmt::Debug for KeyBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().map(Value::from))
            .finish()
    }
}

#[cfg_attr(test, mutants::skip)]
#[cfg(feature = "fuzzing")]
mod fuzzing {
    use super::*;
    use arbitrary::Arbitrary;
    impl<'a> Arbitrary<'a> for Key {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let keys: Vec<_> = u.arbitrary()?;
            match Self::new(&keys) {
                Ok(key) => Ok(key),
                Err(_) => Err(arbitrary::Error::IncorrectFormat),
            }
        }
    }

    impl<'a> Arbitrary<'a> for KeyBound {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let keys: Vec<_> = u.arbitrary()?;
            match Self::new(&keys) {
                Ok(key) => Ok(key),
                Err(_) => Err(arbitrary::Error::IncorrectFormat),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use format::{MAX_INT_SEGMENT_SIZE, MAX_SUB_LEN};
    use internal::MAX_KEY_SIZE;
    use pretty_assertions::assert_eq;

    /// Generate a value by repeating the pattern count times.
    ///
    /// This was created just to reduce boiler plate.
    fn generate_value(pattern: &str, count: usize) -> Value {
        pattern.repeat(count).as_str().try_into().unwrap()
    }

    #[test]
    fn subscripts_have_a_max_size() {
        assert!(KeyBound::new([&generate_value("a", MAX_SUB_LEN)]).is_ok());
        assert_eq!(
            KeyBound::new([&generate_value("a", MAX_SUB_LEN + 1)]),
            Err(Error::SubscriptToLarge)
        );
    }

    #[test]
    fn keys_have_a_max_size() {
        //End marker and internal type marker each take one byte.
        const SUBSCRIPT_STORAGE_OVERHEAD: usize = 2;

        //NOTE I have to use multiple subscripts due to limit on subscript length.
        assert!(KeyBound::new([
            &generate_value("a", MAX_SUB_LEN),
            &generate_value(
                "a",
                MAX_KEY_SIZE 
                - SUBSCRIPT_STORAGE_OVERHEAD //Overhead for storing this key.
                - (MAX_SUB_LEN + SUBSCRIPT_STORAGE_OVERHEAD) // Size of last key + overhead.
            ),
        ])
        .is_ok());

        assert_eq!(
            KeyBound::new([
                &generate_value("a", MAX_SUB_LEN),
                &generate_value(
                    "a",
                    MAX_KEY_SIZE 
                    - SUBSCRIPT_STORAGE_OVERHEAD //Overhead for storing this key.
                    - (MAX_SUB_LEN + SUBSCRIPT_STORAGE_OVERHEAD) // Size of last key + overhead.
                    +1 // Pushing us over the limit.
                ),
            ]),
            Err(Error::KeyToLarge)
        );
    }

    #[test]
    fn subscript_values_can_not_contain_null_byte() {
        let result = KeyBound::new([&"a\0b".try_into().unwrap()]);
        assert_eq!(result, Err(Error::SubKeyContainsNull));
    }

    #[test]
    fn null_subscripts_are_only_valid_as_the_last_subscript_of_a_key_bound() {
        let value = generate_value("a", 1);
        assert!(KeyBound::new(&[Value::empty()]).is_ok());
        assert!(KeyBound::new(&[value.clone(), Value::empty(),]).is_ok());
        assert_eq!(
            KeyBound::new(&[Value::empty(), value]),
            Err(Error::SubKeyIsNull)
        );

        assert_eq!(Key::new(&[Value::empty()]), Err(Error::SubKeyIsNull));
    }

    #[test]
    fn extremely_large_numbers_are_stored_as_strings() {
        // This assert is just here to stop a mutation testing false positive.
        assert_eq!(MAX_INT_SEGMENT_SIZE, 63);

        let key = KeyBound::new([&generate_value("1", MAX_INT_SEGMENT_SIZE + 1)])
        .unwrap();
        assert!(matches!(
            key.iter().next().unwrap().into(),
            IntermediateRepresentation::String(_)
        ));
    }

    #[test]
    fn trailing_slash_leading_dots_and_zeros() {
        //Things that should be strings
        let strings = KeyBound::new(
            [".", "-.", "1.", ".10", "01", "0.1", "1.1.1", "string"]
                .map(|x| Value::try_from(x).unwrap())
                .iter(),
        )
        .unwrap();
        for string_sub_script in &strings {
            assert!(matches!(
                string_sub_script.into(),
                IntermediateRepresentation::String(_)
            ));
        }

        //Things that should **Not** be strings
        let non_strings = KeyBound::new(
            [".1", "10", ".01"]
                .map(|x| Value::try_from(x).unwrap())
                .iter(),
        )
        .unwrap();
        for non_string_sub_script in &non_strings {
            assert!(!matches!(
                non_string_sub_script.into(),
                IntermediateRepresentation::String(_)
            ));
        }
    }

    #[test]
    fn sorting_order_negative_positive_strings() {
        let keys: [KeyBound; 7] = ["", "-9.9", "-9", "0", "9", "9.9", "string"]
            .map(|x| KeyBound::new([&x.try_into().unwrap()]).unwrap());
        for [a, b] in keys.array_windows() {
            assert!(a < b);
        }
    }

    #[test]
    fn value_in_is_value_out() {
        let values: Vec<Value> = ["-9.9", "-9", "0", "9", "9.9", "string", ""]
            .map(|x| x.try_into().unwrap())
            .to_vec();
        let key = KeyBound::new(&values).unwrap();
        for (expected, actual) in values.iter().zip(key.iter()) {
            assert_eq!(expected, &actual.into())
        }
    }
}

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
use serde::{Deserialize, Serialize};
use std::iter;
const MAX_STR_LEN: usize = u16::MAX as usize - 1;
mod convertions;
pub use convertions::CreationError;

#[cfg(feature = "arbitrary")]
mod arbitrary;
mod number;
pub use number::Number;
/// An M Value.
///
/// # Type convertions:
/// In the M language only has one value type and the expression "a"+"b" is perfectly valid in
/// M. It evaluates to 0. However I have chosen not to implement math operations directly on the `Value` type.
///
/// The internal representation used during arithmetic is significantly different from the representation used
/// to store arbitrary values. Since converting between the two is nontrivial I want to be explicit
/// about when the convertions occurs.
/// ```
/// use value::{Value,Number};
/// let a :Number = "a".parse::<Value>().unwrap().into();
/// let b :Number = "b".parse::<Value>().unwrap().into();
/// let _ = a +b;
/// ```
///
/// # Errors:
/// A u16 is used to track the values length.
/// Trying to convert a byte sequence longer the `u16::MAX` -1 results in an error.
/// ```
/// use value::{Value,CreationError};
/// let long_string :String = std::iter::repeat("a").take(u16::MAX.into()).collect();
/// assert_eq!(long_string.parse::<Value>(), Err(CreationError::ExceededMaxStringLen))
/// ```

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value(Vec<u8>);

impl Value {
    /// Returns the raw value as a slice of u8s
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.0[..]
    }

    #[must_use]
    /// Creates a new empty Value
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Serialize value into a byte stream
    ///
    /// The first two bytes are the length represented as a little endian u16.
    /// The remaining bytes are content of the Value.
    ///
    /// NOTE: This is used to match the C ABI, and should only be used if you need to
    /// convert to a `ffi::CSTRING`
    pub fn as_bytes(&self) -> impl Iterator<Item = u8> {
        let len: u16 = self
            .0
            .len()
            .try_into()
            .expect("Max length of Value should fit in a u16");

        //Deconstructing the u16 explicitly to avoid lifetime issues
        let [first, second] = len.to_le_bytes();
        iter::once(first)
            .chain(iter::once(second))
            .chain(self.content().iter().cloned())
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg_attr(test, mutants::skip)]
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("Value");
        if let Ok(contents) = String::from_utf8(self.0.clone()) {
            builder.field("content_as_utf8", &contents);
        } else {
            builder.field("content", &self.content());
        }

        builder.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_bytes() {
        let content: Vec<u8> = (10..15).collect();
        let value = Value::try_from(&content[..]).unwrap();
        let expected = {
            let mut expected = content.clone();
            expected.insert(0, content.len() as u8);
            expected.insert(1, 0);
            expected
        };

        let bytes: Vec<_> = value.as_bytes().collect();
        assert_eq!(bytes, expected);
    }
}

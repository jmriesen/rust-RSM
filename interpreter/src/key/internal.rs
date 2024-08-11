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
use crate::{key::Segment, value::Value};
use ffi::{MAX_KEY_SIZE, MAX_SUB_LEN};

/// Keys are stored in a special format to facilitate sorting.
/// They start with a discriminant.
/// [0,0] -> null
/// [64,0] -> zero
/// [`INT_ZERO_POINT`+x, ..x bytes.., ..., 0] -> positive number; x is # of integer digits (base 10)
/// the digits of a negative number are stored as nines complement.
/// [`INT_ZERO_POINT`-x, ..x bytes.., ..., 0] -> negative number; x is # of integer digits (base 10)
/// [`STRING_FLAG`,..., 0] -> string (if numbers are to large they are stored as strings.

const STRING_FLAG: u8 = 0b1000_0000;
const INT_ZERO_POINT: u8 = 0b100_0000;

/// Only 7 bytes are allocated to store the size of the int portion of a number.
/// any number that takes more integer digits will be stored as a string.
pub const MAX_INT_SEGMENT_SIZE: usize = INT_ZERO_POINT as usize - 1;

use super::{Error, Iter, NullableKey};

use super::IntermediateRepresentation;

impl<'a> IntermediateRepresentation<'a> {
    pub fn from_key_ref(key: Segment<'a>) -> Self {
        let flag = key.0[0];
        let data = &key.0[1..key.0.len() - 1]; //don't include flag or end marker
        match flag {
            0 if data.is_empty() => Self::Null,
            x if x & STRING_FLAG != 0 => Self::String(data),
            x => {
                let non_negative = x & INT_ZERO_POINT != 0;
                let int_len = if non_negative {
                    (INT_ZERO_POINT - 1) & x
                } else {
                    INT_ZERO_POINT - 1 - x
                };

                let int_part = &data[0..int_len as usize];
                let dec_part = &data[int_len as usize..];

                if int_part.is_empty() && dec_part.is_empty() {
                    Self::Zero
                } else if non_negative {
                    Self::Positive { int_part, dec_part }
                } else {
                    Self::Negative {
                        int_part: Box::new(int_part.iter().copied()),
                        dec_part: Box::new(dec_part.iter().copied()),
                    }
                }
            }
        }
    }

    //TODO consider removing allocation.
    //There is nothing inherent to this method that requires allocation.
    pub fn external_fmt(self, quote_strings: bool) -> Vec<u8> {
        match self {
            IntermediateRepresentation::Null => {
                if quote_strings {
                    vec![b'"', b'"']
                } else {
                    vec![]
                }
            }
            IntermediateRepresentation::Zero => vec![b'0'],
            IntermediateRepresentation::Positive { int_part, dec_part } => {
                let mut key = Vec::from(int_part);
                if !dec_part.is_empty() {
                    key.push(b'.');
                    key.extend(dec_part);
                }
                key
            }
            IntermediateRepresentation::Negative { int_part, dec_part } => {
                let mut key = vec![b'-'];
                key.extend(int_part.map(|x| b'9' + b'0' - x));
                let mut dec_part = dec_part.peekable();
                if dec_part.peek().is_some() {
                    key.push(b'.');
                    key.extend(dec_part.map(|x| b'9' + b'0' - x));
                }
                key
            }
            IntermediateRepresentation::String(string) => {
                if quote_strings {
                    let mut output = vec![b'"'];
                    let ends_with_quote =
                        *string.last().expect("empty strings are handle as null") == b'"';
                    //strings are escaped by adding a second " so "\"" = """"
                    let segments = string.split_inclusive(|x| *x == b'"');
                    for segment in segments {
                        output.extend(segment);
                        output.push(b'"');
                    }

                    //if the last segment ends in a quote we need to
                    //add an extra one due to how split_inclusive works.
                    if ends_with_quote {
                        output.push(b'"');
                    }
                    output
                } else {
                    Vec::from(string)
                }
            }
        }
    }
}

impl NullableKey {
    pub fn push(mut self, src: &Value) -> Result<Self, Error> {
        if self.has_trailing_null() {
            Err(Error::SubKeyIsNull)
        } else {
            let internal_key = IntermediateRepresentation::try_from(src)?;
            let end_mark = match internal_key {
                IntermediateRepresentation::Null => {
                    self.0.push(b'\0');
                    None
                }
                IntermediateRepresentation::Zero => {
                    self.0.push(INT_ZERO_POINT);
                    None
                }
                IntermediateRepresentation::Positive { int_part, dec_part } => {
                    self.0.push(INT_ZERO_POINT + int_part.len() as u8);
                    self.0.extend(int_part);
                    self.0.extend(dec_part);
                    None
                }
                IntermediateRepresentation::Negative { int_part, dec_part } => {
                    self.0.push(INT_ZERO_POINT - 1 - int_part.len() as u8);
                    //TODO figure out how this complement is supposed to work.
                    self.0.extend(int_part);
                    self.0.extend(dec_part);
                    Some(255)
                }
                IntermediateRepresentation::String(contents) => {
                    self.0.push(STRING_FLAG);
                    self.0.extend(contents);
                    None
                }
            };
            //Adding end markers
            //NOTE Negatives use 255 as an end mark for some reason.
            //at some point when I understand 9's complement better I should see if I can remove this
            self.0.push(end_mark.unwrap_or(b'\0'));
            if self.len() > MAX_KEY_SIZE as usize {
                Err(Error::KeyToLarge)
            } else {
                Ok(self)
            }
        }
    }

    pub fn into_ckey(self) -> (u8, [u8; 256]) {
        let mut key = [0; 256];
        key[..self.len()].copy_from_slice(&self.0);
        (self.len() as u8, key)
    }

    #[must_use]
    pub fn is_sub_key_of(&self, key: &Self) -> bool {
        self.0[..key.len()] == key.0
    }

    fn has_trailing_null(&self) -> bool {
        self.len() >= 2 && self.0[self.0.len() - 2..] == [0, 0]
    }

    /// a trailing null is considered both the smallish and larges subkey value
    /// during specific operations.
    /// If the key contains a trailing "" this function will return a new key
    /// with the last sub keys value maximized.
    /// otherwise this is a no op.
    ///
    /// The YOU CAN NOT ASSUME THE RETURNED KEY IS VALID for SET or GET operations.
    /// This should only be used to create a bound
    #[must_use]
    pub fn wrap_null_tail(&self) -> std::borrow::Cow<Self> {
        if self.has_trailing_null() {
            let mut modified_key = self.clone();
            modified_key.0[self.len() - 2] = 255;
            std::borrow::Cow::Owned(modified_key)
        } else {
            std::borrow::Cow::Borrowed(self)
        }
    }

    ///Returns a new key that corresponds to the maximum subscript of the input key.
    /// THE RETURNED KEY IS NOT A VALID KEY FOR GET/SET OPERATIONS
    /// This should only be used to create a bound
    #[must_use]
    pub fn upper_subscript_bound(&self) -> NullableKey {
        let mut modified_key = self.0.clone();
        modified_key.push(255);
        modified_key.push(0);
        NullableKey(modified_key)
    }
}

impl<'a> std::iter::Iterator for Iter<'a> {
    type Item = Segment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let key_end_index = match self.tail.first() {
            //special handling for null.
            //since the flagged value == the end mark value
            //we need to manually split the string.
            Some(b'\0') => Some(2),
            Some(x) => {
                let end_mark = match *x {
                    x if x & STRING_FLAG != 0 => b'\0',
                    x if x & INT_ZERO_POINT != 0 => b'\0',
                    //negative numbers
                    _ => 255,
                };
                let index = self
                    .tail
                    .iter()
                    .enumerate()
                    //Find the index of the end mark
                    .find_map(|(i, x)| (*x == end_mark).then_some(i))
                    .expect("all keys must contain an endmark");
                Some(index + 1)
            }
            None => None,
        };

        if let Some(key_end) = key_end_index {
            let (key, tail) = self.tail.split_at(key_end);
            self.tail = tail;
            Some(Segment(&key[..key.len()]))
        } else {
            None
        }
    }
}

impl Ord for NullableKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let min_len = self.0.len().min(other.0.len());
        match self.0[..min_len].cmp(&other.0[..min_len]) {
            //NOTE If the prefixes are the same the longer one comes first.
            std::cmp::Ordering::Equal => self.0.len().cmp(&other.0.len()),
            x => x,
        }
    }
}

impl PartialOrd for NullableKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

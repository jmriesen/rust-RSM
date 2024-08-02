use std::borrow::Cow;

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

use super::{Error, Iter, Key};

/// represents a key parsed out into its individual parts.
pub enum ParsedKey<'a> {
    Null,
    Zero,
    Positive {
        int_part: &'a [u8],
        dec_part: &'a [u8],
    },
    Negative {
        //lastly converting to nines complement
        //dyn so I can provide a no op if the source bytes are already in nine's complement
        int_part: Box<dyn ExactSizeIterator<Item = u8> + 'a>,
        dec_part: Box<dyn ExactSizeIterator<Item = u8> + 'a>,
    },
    ///NOTE if a number is to large it is encoded as a string
    String(&'a [u8]),
}

impl<'a> ParsedKey<'a> {
    pub fn new(src: &'a Value) -> Result<Self, Error> {
        let contents = src.content();
        if contents.len() > MAX_SUB_LEN as usize {
            //TODO consider reevaluating where this should be enforced.
            //If the goal of this is to prevent a buffer overflow then it should really be owned by
            //KeyList
            Err(Error::SubscriptToLarge)
        } else if contents.is_empty() {
            Ok(Self::Null)
        } else if contents == [b'0'] {
            Ok(Self::Zero)
        } else if contents.contains(&b'\0') {
            Err(Error::ContainsNull)
        } else {
            //attempt to parse as a number
            let negative = contents.starts_with(&[b'-']);
            let mut parts = if negative { &contents[1..] } else { contents }.split(|x| *x == b'.');

            let int_part = parts
                .next()
                .expect("empty string case should have already been handled");
            let dec_part = parts.next();

            let multiple_decimal_points = parts.next().is_some();
            let trailing_dot = dec_part == Some(&[]);
            let dec_part = dec_part.unwrap_or_default();

            let leading_trailing_zeros =
                int_part.starts_with(&[b'0']) || dec_part.ends_with(&[b'0']);

            let is_numeric = |x: &[u8]| x.iter().all(u8::is_ascii_digit);
            let numeric = is_numeric(int_part) && is_numeric(dec_part);
            let contains_no_digits = int_part.is_empty() && dec_part.is_empty();

            if !numeric
                || trailing_dot
                || leading_trailing_zeros
                || multiple_decimal_points
                || contains_no_digits
                || int_part.len() > MAX_INT_SEGMENT_SIZE
            {
                Ok(Self::String(contents))
            } else if negative {
                Ok(Self::Negative {
                    int_part: Box::new(int_part.iter().map(|x| b'9' - x + b'0')),
                    dec_part: Box::new(dec_part.iter().map(|x| b'9' - x + b'0')),
                })
            } else {
                Ok(Self::Positive { int_part, dec_part })
            }
        }
    }

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
            ParsedKey::Null => {
                if quote_strings {
                    vec![b'"', b'"']
                } else {
                    vec![]
                }
            }
            ParsedKey::Zero => vec![b'0'],
            ParsedKey::Positive { int_part, dec_part } => {
                let mut key = Vec::from(int_part);
                if !dec_part.is_empty() {
                    key.push(b'.');
                    key.extend(dec_part);
                }
                key
            }
            ParsedKey::Negative { int_part, dec_part } => {
                let mut key = vec![b'-'];
                key.extend(int_part.map(|x| b'9' + b'0' - x));
                let mut dec_part = dec_part.peekable();
                if dec_part.peek().is_some() {
                    key.push(b'.');
                    key.extend(dec_part.map(|x| b'9' + b'0' - x));
                }
                key
            }
            ParsedKey::String(string) => {
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

impl Key {
    pub fn push(mut self, src: &Value) -> Result<Self, Error> {
        let internal_key = ParsedKey::new(src)?;
        let end_mark = match internal_key {
            ParsedKey::Null => {
                self.0.push(b'\0');
                None
            }
            ParsedKey::Zero => {
                self.0.push(INT_ZERO_POINT);
                None
            }
            ParsedKey::Positive { int_part, dec_part } => {
                self.0.push(INT_ZERO_POINT + int_part.len() as u8);
                self.0.extend(int_part);
                self.0.extend(dec_part);
                None
            }
            ParsedKey::Negative { int_part, dec_part } => {
                self.0.push(INT_ZERO_POINT - 1 - int_part.len() as u8);
                //TODO figure out how this complement is supposed to work.
                self.0.extend(int_part);
                self.0.extend(dec_part);
                Some(255)
            }
            ParsedKey::String(contents) => {
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

    #[must_use]
    pub fn is_sub_key_of(&self, key: &Self) -> bool {
        self.0[..key.len()] == key.0
    }

    /// a trailing null is considered both the smallish and larges subkey value
    /// during specific operations.
    /// If the key contains a trailing "" this function will return a new key
    /// with the last sub keys value maximized.
    /// otherwise this is a no op.
    ///
    /// This function has been marked unsafe sine the returned key should ONLY be used as a bound.
    /// The YOU CAN NOT ASSUME THE RETURNED KEY IS VALID for SET or GET operations.
    pub unsafe fn wrap_null_key(&self) -> std::borrow::Cow<Self> {
        if self.0[self.0.len() - 2..] == [0, 0] {
            let mut modified_key = self.clone();
            modified_key.0[self.len() - 2] = 255;
            std::borrow::Cow::Owned(modified_key)
        } else {
            std::borrow::Cow::Borrowed(self)
        }
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

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let min_len = self.0.len().min(other.0.len());
        match self.0[..min_len].cmp(&other.0[..min_len]) {
            //NOTE If the prefixes are the same the longer one comes first.
            std::cmp::Ordering::Equal => self.0.len().cmp(&other.0.len()),
            x => x,
        }
    }
}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

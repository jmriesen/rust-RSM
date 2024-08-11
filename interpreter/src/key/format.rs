use crate::value::Value;

use super::Error;

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
/// This modual is repsocible for understading how SubKeys are represented
/// and translate between the internal and external formats.
/// represents a key parsed out into its individual parts.
///
/// Note the reason this internal formatting is used is so A<B can be achieved by just using a single memcmp.
/// NOTE if a number is to large it is just stored using the string representation.
/// Internal format = External format
/// [0,0] = null
/// [64,0] = zero
/// [`INT_ZERO_POINT`+x, ..x bytes.., ..., 0] = positive number; x is # of integer digits (base 10)
/// the digits of a negative number are stored as nines complement.
/// [`INT_ZERO_POINT`-x, ..x bytes.., ..., 0] = negative number; x is # of integer digits (base 10)
/// [`STRING_FLAG`,..., 0] = string (if numbers are to large they are stored as strings.

const MAX_SUB_LEN: usize = 127;
const MAX_KEY_SIZE: usize = 255;

const STRING_FLAG: u8 = 0b1000_0000;
const INT_ZERO_POINT: u8 = 0b100_0000;

/// Only 7 bytes are allocated to store the size of the int portion of a number.
/// any number that takes more integer digits will be stored as a string.
pub const MAX_INT_SEGMENT_SIZE: usize = INT_ZERO_POINT as usize - 1;

/// An intermediate representation
/// There is one variant per schema used in this encoding scheme.
pub enum IntermediateRepresentation<'a> {
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

impl<'a> TryFrom<&'a Value> for IntermediateRepresentation<'a> {
    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        let contents = value.content();
        if contents.len() > MAX_SUB_LEN {
            //TODO consider reevaluating where this should be enforced.
            //If the goal of this is to prevent a buffer overflow then it should really be owned by
            //KeyList
            Err(Error::SubscriptToLarge)
        } else if contents.is_empty() {
            Ok(Self::Null)
        } else if contents == [b'0'] {
            Ok(Self::Zero)
        } else if contents.contains(&b'\0') {
            Err(Error::SubKeyContainsNull)
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
}

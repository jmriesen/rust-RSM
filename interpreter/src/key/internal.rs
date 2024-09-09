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
use crate::{key::SubKey, value::Value};

const MAX_KEY_SIZE: usize = 255;

use super::{format, Error, Iter, NonNullableKey, NullableKey};

use super::IntermediateRepresentation;

impl<'a> IntermediateRepresentation<'a> {
    pub fn external_fmt(self, quote_strings: bool) -> Vec<u8> {
        let mut tmp = Vec::new();
        self.push_external_fmt(&mut tmp, quote_strings);
        tmp
    }
}

impl TryFrom<NullableKey> for NonNullableKey {
    type Error = ();

    fn try_from(value: NullableKey) -> Result<Self, Self::Error> {
        if value.has_trailing_null() {
            Err(())
        } else {
            Ok(NonNullableKey(value))
        }
    }
}

impl NullableKey {
    pub fn push(self, src: &Value) -> Result<Self, Error> {
        if self.has_trailing_null() {
            Err(Error::SubKeyIsNull)
        } else {
            //Pulling Vec out of Self since I may break NullableKey invariants.
            let mut data = self.0;
            let sub_key = IntermediateRepresentation::try_from(src)?;
            sub_key.push_internal_fmt(&mut data);

            if data.len() > MAX_KEY_SIZE {
                Err(Error::KeyToLarge)
            } else {
                Ok(Self(data))
            }
        }
    }

    #[must_use]
    pub fn into_ckey(self) -> (u8, [u8; 256]) {
        let mut key = [0; 256];
        key[..self.len()].copy_from_slice(&self.0);
        (self.len() as u8, key)
    }

    #[must_use]
    pub fn is_sub_key_of(&self, key: &Self) -> bool {
        self.len() >= key.len() && self.0[..key.len()] == key.0
    }

    fn has_trailing_null(&self) -> bool {
        let null_len = format::NULL.len();
        self.len() >= null_len && self.0[self.0.len() - null_len..] == format::NULL
    }

    /// a trailing null is considered both the smallish and larges subkey value
    /// during specific operations.
    /// If the key contains a trailing "" this function will return a new key
    /// with the last sub keys value maximized.
    /// otherwise this is a no op.
    #[must_use]
    pub fn wrap_null_tail(&self) -> std::borrow::Cow<Self> {
        if self.has_trailing_null() {
            let mut modified_key = self.0.clone();
            for _ in 0..format::NULL.len() {
                modified_key.pop();
            }
            modified_key.extend(format::MAX_SUB_KEY);
            std::borrow::Cow::Owned(Self(modified_key))
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
        modified_key.extend(format::MAX_SUB_KEY);
        NullableKey(modified_key)
    }
    pub fn extract_sibling_sub_key<'a>(&self, other: &'a Self) -> Option<SubKey<'a>> {
        //NOTE This was not written to be preferment, just correct.
        //This should totally be possible to do without any additional allocations.
        let mut keys: Vec<_> = self.iter().collect();
        keys.pop();
        let parent = keys;
        let other: Vec<_> = other.iter().collect();

        //Only Extract if the parents are the same
        if other.len() > parent.len() && parent[..] == other[..parent.len()] {
            Some(other[parent.len()])
        } else {
            None
        }
    }
}

impl<'a> std::iter::Iterator for Iter<'a> {
    type Item = SubKey<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let key_end = IntermediateRepresentation::seek_key_end(self.tail)?;
        let (key, tail) = self.tail.split_at(key_end);
        self.tail = tail;
        Some(SubKey(&key[..key.len()]))
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

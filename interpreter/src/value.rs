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

use ffi::{CSTRING, MAX_STR_LEN};
use std::string::FromUtf8Error;

///This type represents the contents of an M Value.
///This can store arbitrary data but is most commonly strings.
///
///NOTE There is a `byte_maxs` of 65535 (just like 'CSTRINGS')
#[derive(Clone, PartialEq, Eq)] //NOTE keep the Manual Debug implementation in sync
pub struct Value(Vec<u8>);

impl Value {
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.0[..]
    }
    pub fn into_string(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.0)
    }

    #[must_use]
    pub fn into_cstring(self) -> CSTRING {
        let mut buf = [0; MAX_STR_LEN as usize + 1];
        buf[..self.0.len()].copy_from_slice(&self.0[..]);
        CSTRING {
            len: self.0.len() as u16,
            buf,
        }
    }
    #[must_use] pub const fn empty() -> Self {
        Self(Vec::new())
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<&CSTRING> for Value {
    fn from(value: &CSTRING) -> Self {
        let data = &value.buf[..value.len as usize];
        Self(Vec::from(data))
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() <= MAX_STR_LEN as usize {
            Ok(Self(Vec::from(value)))
        } else {
            Err(())
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("content", &self.content())
            .field("content_as_utf8", &self.clone().into_string())
            .finish()
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod utility {
    use arbitrary::Arbitrary;
    use ffi::MAX_STR_LEN;

    use super::Value;

    impl<'a> Arbitrary<'a> for Value {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let len: usize = u.int_in_range(0..=MAX_STR_LEN as usize)?;
            Ok(Self(Vec::from(u.bytes(len)?)))
        }
    }

    impl TryFrom<&str> for Value {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() <= MAX_STR_LEN as usize {
                Ok(Self(Vec::from(value.as_bytes())))
            } else {
                Err(())
            }
        }
    }
}

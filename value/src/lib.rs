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
pub static EMPTY: Value = Value::empty();
const MAX_STR_LEN: usize = 65534;

mod convertions;
#[cfg(feature = "ffi")]
mod ffi;

#[cfg(feature = "arbitrary")]
mod arbitrary;
mod number;
pub use number::Number;
///This type represents the contents of an M Value.
///This can store arbitrary data but is most commonly strings.
///
///NOTE There is a `byte_maxs` of 65535 (just like 'CSTRINGS')
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)] //NOTE keep the Manual Debug implementation in sync
pub struct Value(Vec<u8>);

impl Value {
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.0[..]
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
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

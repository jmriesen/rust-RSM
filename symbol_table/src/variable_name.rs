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

//NOTE I am specificity not using an ArraryString since it could contain non utf-8 data.
const MAX_VAR_NAME_SIZE: usize = 32;
use core::fmt;
use std::{hash::Hash, os::raw::c_uchar};

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use thiserror::Error;
/// The name of a M variable.
///
/// Any variable who's name starts with $ is considered an intrinsic.
/// The maximum length of a variable name is `MAX_VAR_NAME_SIZE`
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VariableName(ArrayVec<c_uchar, MAX_VAR_NAME_SIZE>);

/// All the ways in we can fail to build a variable name.
#[derive(Error, Debug)]
pub enum CreattionError {
    #[error("variable names are not allowed to contain null bytes")]
    ContainsNullByte,
    #[error("The max length of a name is {MAX_VAR_NAME_SIZE}")]
    ExceedsMaxLength,
}

impl VariableName {
    /// Intrinsics start with a '$' and are built in variables.
    pub fn is_intrinsic(&self) -> bool {
        self.contents()[0] == b'$'
    }

    /// Returns the raw representation.
    ///
    /// This should only be used inside of self, or from the ffi crate.
    pub fn contents(&self) -> &[u8] {
        &self.0[..]
    }
    /// Creates a new Variable Name.
    ///
    /// This can fail if bytes is to long.
    // NOTE I am not 100% sold on this API.
    // Be prepared for it to change
    pub fn new(bytes: &[u8]) -> Result<Self, CreattionError> {
        if bytes.contains(&0) {
            Err(CreattionError::ContainsNullByte)
        } else if bytes.len() > MAX_VAR_NAME_SIZE {
            Err(CreattionError::ExceedsMaxLength)
        } else {
            let mut array = ArrayVec::new();
            array.extend(bytes.iter().copied());
            Ok(Self(array))
        }
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0[..]).unwrap(),)
    }
}

#[cfg(test)]
pub mod test_helpers {
    use crate::VariableName;

    #[test]
    fn display() {
        assert_eq!(format!("{}", VariableName::new(b"foo").unwrap()), "foo")
    }
}

#[cfg(feature = "arbitrary")]
pub mod helpers {
    use arbitrary::Arbitrary;

    use super::{MAX_VAR_NAME_SIZE, VariableName};

    #[cfg_attr(test, mutants::skip)]
    impl<'a> Arbitrary<'a> for VariableName {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let len = u.int_in_range(1..=MAX_VAR_NAME_SIZE)?;
            Ok(VariableName(
                u.arbitrary_iter()?
                    //TODO figure out if these constraints are accurate
                    .filter(|x| x.is_ok_and(|x: u8| x.is_ascii()))
                    .filter(|x| x.is_ok_and(|x: u8| x != 0))
                    .take(len)
                    .collect::<Result<_, _>>()?,
            ))
        }
    }
}

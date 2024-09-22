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
use std::hash::Hash;

use arrayvec::ArrayVec;
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VarU(arrayvec::ArrayVec<std::os::raw::c_uchar, MAX_VAR_NAME_SIZE>);

impl VarU {
    pub fn is_intrinsic(&self) -> bool {
        self.contents()[0] == b'$'
    }

    pub fn contents(&self) -> &[u8] {
        &self.0[..]
    }
    //NOTE I am not 100% sold on this API.
    //Be prepared for it to change
    pub fn new(bytes: &[u8]) -> Result<Self, ()> {
        if !bytes.contains(&0) && bytes.len() <= MAX_VAR_NAME_SIZE {
            let mut array = ArrayVec::new();
            array.extend(bytes.iter().cloned());
            Ok(Self(array))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for VarU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO handle error case
        write!(f, "{}", std::str::from_utf8(&self.0[..]).unwrap(),)
    }
}

#[cfg(feature = "ffi")]
impl VarU {
    pub fn into_c(&self) -> ffi::VAR_U {
        use std::iter::repeat_n;
        let mut array = self.0.clone();
        array.extend(repeat_n(0, array.remaining_capacity()));
        ffi::VAR_U {
            var_cu: array
                .into_inner()
                .expect("we have allready fill the capasity with zeros"),
        }
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {
    use arbitrary::Arbitrary;

    use super::{VarU, MAX_VAR_NAME_SIZE};

    #[cfg_attr(test, mutants::skip)]
    impl<'a> Arbitrary<'a> for VarU {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let len = u.int_in_range(1..=MAX_VAR_NAME_SIZE)?;
            Ok(VarU(
                u.arbitrary_iter()?
                    //TODO figure out if these constraints are accurate
                    .filter(|x| x.is_ok_and(|x: u8| x.is_ascii()))
                    .filter(|x| x.is_ok_and(|x: u8| x != 0))
                    .take(len)
                    .collect::<Result<_, _>>()?,
            ))
        }
    }

    #[must_use]
    pub fn var_u(var: &str) -> VarU {
        VarU(var.bytes().take(MAX_VAR_NAME_SIZE).collect())
    }
}

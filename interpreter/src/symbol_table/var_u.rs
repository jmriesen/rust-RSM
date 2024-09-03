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
//New type wrapper so I can implement methods on VAR_U
//TODO decouple from ffi
use std::hash::Hash;
#[derive(Clone, Debug)]
pub struct VarU(pub ffi::VAR_U);

impl VarU {
    pub fn is_intrinsic(&self) -> bool {
        unsafe { self.0.var_cu[0] == b'$' }
    }
}

impl Hash for VarU {
    #[cfg_attr(test, mutants::skip)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.0.var_cu }.hash(state);
    }
}

impl Eq for VarU {}
impl PartialEq for VarU {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.var_cu == other.0.var_cu }
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {
    use arbitrary::Arbitrary;

    use super::VarU;

    #[must_use]
    pub fn var_u(var: &str) -> VarU {
        VarU(var.try_into().unwrap())
    }

    impl<'a> Arbitrary<'a> for VarU {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let name: [u8; 32] = u.arbitrary()?;
            if name.is_ascii() && name.contains(&0) {
                Ok(Self(ffi::VAR_U { var_cu: name }))
            } else {
                Err(arbitrary::Error::IncorrectFormat)
            }
        }
    }
}

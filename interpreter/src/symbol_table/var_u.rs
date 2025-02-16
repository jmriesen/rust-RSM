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

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug)]
pub struct VarU(pub ffi::VAR_U);

impl Serialize for VarU {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Ok(self.contents().to_vec().serialize(serializer)?)
    }
}

impl<'de> Deserialize<'de> for VarU {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<u8>::deserialize(deserializer)?;
        let mut var_cu = [0; 32];
        var_cu[..vec.len()].copy_from_slice(&vec[..]);
        Ok(Self(ffi::VAR_U { var_cu }))
    }
}

impl VarU {
    pub fn is_intrinsic(&self) -> bool {
        self.contents()[0] == b'$'
    }

    pub fn contents(&self) -> &[u8] {
        let internals = unsafe { &self.0.var_cu[..] };
        internals
            .split_once(|x| *x == 0)
            .map(|x| x.0)
            .unwrap_or(&[])
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
    use ffi::VAR_U;

    use super::VarU;

    #[cfg_attr(test, mutants::skip)]
    impl<'a> Arbitrary<'a> for VarU {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let mut var_cu: [u8; 32] = u.arbitrary()?;
            if var_cu.is_ascii() && var_cu.contains(&0) {
                for x in var_cu.iter_mut().skip_while(|x| **x != 0) {
                    *x = 0;
                }
                Ok(Self(VAR_U { var_cu }))
            } else {
                Err(arbitrary::Error::IncorrectFormat)
            }
        }
    }

    #[must_use]
    pub fn var_u(var: &str) -> VarU {
        VarU(var.try_into().unwrap())
    }
}

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
//TODO remove once this module is actually being used.
#![allow(dead_code)]

mod hash;
mod key;
mod m_var;
mod var_data;
mod var_u;

use crate::value::Value;
use ffi::{PARTAB, UCI_IS_LOCALVAR};
use hash::CreationError;
use key::{self, NonNullableKey};
pub use m_var::MVar;
pub use var_data::Direction;
use var_data::{DataResult, VarData};
use var_u::VarU;

impl hash::Key for VarU {
    fn error() -> Self {
        Self("$ECODE".try_into().expect("the error key is a valid VarU"))
    }
}

#[derive(Default, Debug)]
pub struct Table(hash::HashTable<VarU, VarData>);

impl Table {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    ///Gets a value that was stored in the symbol table.
    #[must_use]
    pub fn get(&self, var: &MVar<NonNullableKey>) -> Option<&Value> {
        self.0.locate(&var.name)?.value(&var.key)
    }

    /// Inserts a value into the symbol table
    pub fn set(&mut self, var: &MVar<NonNullableKey>, value: &Value) -> Result<(), CreationError> {
        self.0.create(var.name.clone())?.set_value(&var.key, value);
        Ok(())
    }

    pub fn kill(&mut self, var: &MVar<NonNullableKey>) {
        if let Some(data) = self.0.locate_mut(&var.name) {
            data.kill(&var.key);
            if !data.contains_data() {
                self.0.free(&var.name);
            }
        }
    }

    //NOTE not yet mutation tested
    fn keep(&mut self, vars: &[VarU], tab: &mut PARTAB) {
        //NOTE I am not sure how src_var is used, but this was done in the C code.
        tab.src_var = ffi::MVAR {
            uci: UCI_IS_LOCALVAR as u8,
            slen: 0,
            volset: 0,
            ..tab.src_var
        };
        //Keep anything from the passed in slice and all $ vars
        self.0
            .remove_if(|x| !(vars.contains(x) || x.is_intrinsic()));
    }

    #[must_use]
    pub fn data(&self, var: &MVar<NonNullableKey>) -> DataResult {
        self.0.locate(&var.name).map_or(
            DataResult {
                has_value: false,
                has_descendants: false,
            },
            |x| x.data(&var.key),
        )
    }

    //Returns a string representation of Key in the given MVar.
    #[must_use]
    pub fn query<Key: key::Key>(
        &self,
        var: &MVar<Key>,
        direction: Direction,
    ) -> Option<MVar<NonNullableKey>> {
        self.0
            .locate(&var.name)
            .and_then(|data| data.query(var.key.borrow(), direction))
            .map(|key| var.copy_new_key(key))
    }

    ///Returns the next `sub_key` for a given variable.
    #[must_use]
    pub fn order<Key: key::Key>(&self, var: &MVar<Key>, direction: Direction) -> Value {
        self.0
            .locate(&var.name)
            .and_then(|data| data.order(var.key.borrow(), direction))
            .map(std::convert::Into::into)
            .unwrap_or_default()
    }
}

#[cfg(test)]
pub mod tests;

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
#![feature(btree_cursors)]
#![feature(let_chains)]
#![feature(hash_extract_if)]
#![feature(slice_split_once)]
#![feature(array_windows)]
#![feature(iter_repeat_n)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
//I disagree with how this lint handles expects.
//It requires documentation on panics that should be unreachable.
#![allow(clippy::missing_panics_doc)]

//TODO go over and see what can be made private
mod hash;
pub mod key;
mod m_var;
mod var_data;
mod var_u;

use hash::CreationError;
use key::{Key, SubKey};
pub use m_var::MVar;
use value::Value;
pub use var_data::Direction;
use var_data::{DataResult, VarData};
use var_u::VarU;

impl hash::Key for VarU {
    fn error() -> Self {
        VarU::new(b"$ECODE").expect("the error key is a valid VarU")
    }
}

/// Stores the information required to restore new-ed variables to there previous state.
/// This Vec should be treated as a Stack, i.e. FILO.
type NewFrame = Vec<(VarU, VarData)>;

#[derive(Default, Debug)]
pub struct Table {
    table: hash::HashTable<VarU, VarData>,
    stack: Vec<NewFrame>,
}

impl Table {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    ///Gets a value that was stored in the symbol table.
    #[must_use]
    pub fn get(&self, var: &MVar<Key>) -> Option<&Value> {
        self.table.locate(&var.name)?.value(&var.key)
    }

    /// Inserts a value into the symbol table
    pub fn set(&mut self, var: &MVar<Key>, value: &Value) -> Result<(), CreationError> {
        self.table
            .create(var.name.clone())?
            .set_value(&var.key, value);
        Ok(())
    }

    pub fn kill(&mut self, var: &MVar<Key>) {
        if let Some(data) = self.table.locate_mut(&var.name) {
            data.kill(&var.key);
            if !(data.has_data() || self.attached(&var.name)) {
                self.table.free(&var.name);
            }
        }
    }

    //NOTE not yet mutation tested
    pub fn keep(&mut self, vars: &[VarU]) {
        //Keep anything from the passed in slice and all $ vars
        self.table
            .remove_if(|x| !(vars.contains(x) || x.is_intrinsic()));
    }

    #[must_use]
    pub fn data(&self, var: &MVar<Key>) -> DataResult {
        self.table.locate(&var.name).map_or(
            DataResult {
                has_value: false,
                has_descendants: false,
            },
            |x| x.data(&var.key),
        )
    }

    /// Returns the next record in the variable.
    #[must_use]
    pub fn query<K: key::KeyType>(&self, var: &MVar<K>, direction: Direction) -> Option<MVar<Key>> {
        self.table
            .locate(&var.name)
            .and_then(|data| data.query(var.key.borrow(), direction))
            .map(|key| var.copy_new_key(key))
    }

    /// Returns the next `sub_key` that is in `MVar` and at the same `sub_key` depth as the provided `MVar`.
    #[must_use]
    pub fn order<Key: key::KeyType>(
        &self,
        var: &MVar<Key>,
        direction: Direction,
    ) -> Option<SubKey> {
        self.table
            .locate(&var.name)
            .and_then(|data| data.order(var.key.borrow(), direction))
    }

    /// Adds a `NewFrame` to the variable stack.
    /// All subsequent calls to `new_var` will make use of this stack until another one has been
    /// pushed.
    pub fn push_new_frame(&mut self) {
        self.stack.push(NewFrame::default());
    }

    /// Pops a `NewFrame` off the stack.
    /// This will restore the values of all variables that were new-ed while that `NewFrame` was in
    /// use.
    pub fn pop_new_frame(&mut self) {
        let frame = self.stack.pop();
        if let Some(frame) = frame {
            //NOTE we are reversing the order so that we restore the variables in the opposite order in
            //which they were new-ed. This matters in the case where a variable was new-ed
            //multiple times.
            for (var, data) in frame.into_iter().rev() {
                //If there is data to restore OR the variable was new-ed before.
                if data.has_data() || self.attached(&var) {
                    let slot = self
                        .table
                        .locate_mut(&var)
                        .expect("The slot should already exists");
                    *slot = data;
                } else {
                    self.table.free(&var);
                }
            }
        }
    }

    /// News a set of variables.
    /// The current state of the new-ed variables will be stored in the current `NewFrame`.
    /// When the current `NewFrame` is popped from the stack, any new-ed variables will return to there
    /// pre new-ed state.
    ///
    /// PANICS this method will panic if the `SymbolTables` `NewFrame` stack is empty.
    ///
    /// NOTE if you new the same variable multiple times during the same `NewFrame` popping the frame will
    /// reset the variable state to what the state before the first call to `new_var` on the current
    /// `NewFrame`.
    ///
    /// NOTE A new-ed variable will take up space in the `SymbolTable` table even if the variables
    /// value is never set.
    pub fn new_var(&mut self, vars: &[&VarU]) -> Result<(), CreationError> {
        // Will panic if there is no current new_frame
        let current_frame = self
            .stack
            .last_mut()
            .expect("There must be a NewFrame in the stack before you can call new");
        for &var in vars {
            let slot = self.table.create(var.clone())?;
            current_frame.push((var.clone(), std::mem::take(slot)));
        }
        Ok(())
    }

    /// News all the variables that exist in the symbol table except for intrinsic variables and
    /// variables specified in the exclude parameter.
    pub fn new_all_but(&mut self, exclude: &[&VarU]) -> Result<(), CreationError> {
        let vars_to_new: Vec<_> = self
            .table
            .iter()
            .map(|(var, _value)| var)
            .filter(|x| !(x.is_intrinsic() || exclude.contains(x)))
            //NOTE I need to clone to avoid double borrowing
            .cloned()
            .collect();
        let vars_to_new: Vec<_> = vars_to_new.iter().collect();
        self.stack.push(NewFrame::with_capacity(vars_to_new.len()));
        self.new_var(&vars_to_new)
    }

    /// Checks if this variables exists anywhere in the `NewFrame` Stack.
    fn attached(&self, var: &VarU) -> bool {
        self.stack
            .iter()
            .any(|new_frame| new_frame.iter().any(|(new_ed_var, _)| var == new_ed_var))
    }
}

#[cfg(test)]
pub mod tests;

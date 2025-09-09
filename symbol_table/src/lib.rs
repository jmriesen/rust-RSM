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
#![feature(btree_cursors)]
#![feature(slice_split_once)]
#![feature(array_windows)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
//I disagree with how this lint handles expects.
//It requires documentation on panics that should be unreachable.
#![allow(clippy::missing_panics_doc)]

mod hash;
pub mod key;
mod m_var;
mod value_tree;
mod variable_name;

pub use hash::CreationError;
use key::{Path, Segment};
pub use m_var::MVar;
use value::Value;
use value_tree::ValueTree;
pub use value_tree::{DataResult, Direction};
pub use variable_name::VariableName;

impl hash::Key for VariableName {
    fn error() -> Self {
        VariableName::new(b"$ECODE").expect("the error key is a valid VarU")
    }
}

/// Stores the information required to restore new-ed variables to there previous state.
/// This Vec should be treated as a Stack, i.e. FILO.
type NewFrame = Vec<(VariableName, ValueTree)>;

/// The a `SymbolTable` stores
/// 1) What variables are currently in scope.
/// 2) What the value of those variables are.
/// 3) How to restore/shadow variables when the current scope changes.
#[derive(Default, Debug)]
pub struct SymbolTable {
    table: hash::HashTable<VariableName, ValueTree>,
    stack: Vec<NewFrame>,
}

impl SymbolTable {
    #[must_use]
    /// Create a new empty `SymbolTable`
    pub fn new() -> Self {
        Self::default()
    }

    ///Gets a value that was stored in the symbol table.
    #[must_use]
    pub fn get(&self, var: &MVar<Path>) -> Option<&Value> {
        self.table.get(&var.name)?.value(&var.key)
    }

    /// Inserts a value into the `SymbolTale` replacing any existing value.
    pub fn set(&mut self, var: &MVar<Path>, value: &Value) -> Result<(), CreationError> {
        self.table
            .create_entry(var.name.clone())?
            .set_value(&var.key, value);
        Ok(())
    }

    /// Removing var including all of its sub-keys.
    pub fn kill(&mut self, var: &MVar<Path>) {
        if let Some(data) = self.table.get_mut(&var.name) {
            data.kill(&var.key);
            if !(data.not_empty() || self.attached(&var.name)) {
                self.table.remove(&var.name);
            }
        }
    }

    /// Kill all variables except `keep` and intrinsic variables.
    //NOTE not yet mutation tested
    pub fn keep(&mut self, keep: &[VariableName]) {
        self.table
            .remove_if(|x| !(keep.contains(x) || x.is_intrinsic()));
    }

    /// The Data M intrinsic function.
    /// Checks if the specified key
    /// 1) Has a value.
    /// 2) Has any sub-keys.
    #[must_use]
    pub fn data(&self, var: &MVar<Path>) -> DataResult {
        self.table.get(&var.name).map_or(
            DataResult {
                has_value: false,
                has_descendants: false,
            },
            |x| x.data(&var.key),
        )
    }

    /// Returns the next the variable regardless of key-level.
    #[must_use]
    pub fn query<K: key::PathType>(
        &self,
        var: &MVar<K>,
        direction: Direction,
    ) -> Option<MVar<Path>> {
        self.table
            .get(&var.name)
            .and_then(|data| data.query(var.key.borrow(), direction))
            .map(|key| var.copy_new_key(key))
    }

    /// Returns the next `sub_key` that is in `MVar` and at the same `sub_key` depth as the provided `MVar`.
    #[must_use]
    pub fn order<Key: key::PathType>(
        &self,
        var: &MVar<Key>,
        direction: Direction,
    ) -> Option<Segment<'_>> {
        self.table
            .get(&var.name)
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
                if data.not_empty() || self.attached(&var) {
                    let slot = self
                        .table
                        .get_mut(&var)
                        .expect("The slot should already exists");
                    *slot = data;
                } else {
                    self.table.remove(&var);
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
    pub fn new_var(&mut self, vars: &[&VariableName]) -> Result<(), CreationError> {
        // Will panic if there is no current new_frame
        let current_frame = self
            .stack
            .last_mut()
            .expect("There must be a NewFrame in the stack before you can call new");
        for &var in vars {
            let slot = self.table.create_entry(var.clone())?;
            current_frame.push((var.clone(), std::mem::take(slot)));
        }
        Ok(())
    }

    /// News all the variables that exist in the symbol table except for intrinsic variables and
    /// variables specified in the exclude parameter.
    pub fn new_all_but(&mut self, exclude: &[&VariableName]) -> Result<(), CreationError> {
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
    fn attached(&self, var: &VariableName) -> bool {
        self.stack
            .iter()
            .any(|new_frame| new_frame.iter().any(|(new_ed_var, _)| var == new_ed_var))
    }
}

#[cfg(test)]
pub mod tests;

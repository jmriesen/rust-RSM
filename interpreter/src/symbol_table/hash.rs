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

const NUM_SLOTS: usize = 3073;
//TODO make fields private
/// Wrapper around the std std::collections::HashMap with a few additional properties.
/// Max number of entries is 'NUM_SLOTS'*
/// If the map is full we can still insert a special Error Key.
/// Internally the keys are stored in a array, and the indexes into that array are handed out using
/// a stack.
pub struct HashTable<K, V>
where
    K: Eq + std::hash::Hash,
    V: Default,
{
    /// Maps keys to their internal index
    pub map: std::collections::HashMap<K, usize>,
    ///Stack storing which indexes are available
    pub open_slots: Vec<usize>,
    ///The actual data store
    pub slots: [Option<V>; NUM_SLOTS],
}

impl<K, V> HashTable<K, V>
where
    K: Eq + std::hash::Hash,
    V: Default,
{
    pub fn new() -> Self {
        let mut open_slots = Vec::with_capacity(NUM_SLOTS);
        for i in (0..NUM_SLOTS).rev() {
            open_slots.push(i)
        }
        Self {
            open_slots,
            map: Default::default(),
            slots: array::from_fn(|_| None),
        }
    }

    pub fn create(&mut self, key: K) -> Result<Index, CreationError> {
        match self.map.entry(key) {
            hash_map::Entry::Occupied(entry) => Ok(Index(*entry.get() as i16)),
            hash_map::Entry::Vacant(entry) => {
                if let Some(new_slot_index) = self.open_slots.pop() {
                    entry.insert(new_slot_index);
                    self.slots[new_slot_index] = Some(V::default());
                    Ok(Index(new_slot_index as i16))
                } else {
                    //TODO Deal with $ECODE
                    Err(CreationError)
                }
            }
        }
    }

    pub fn locate(&self, key: &K) -> Option<Index> {
        self.map.get(key).map(|x| Index(*x as i16))
    }

    pub fn free(&mut self, key: &K) {
        if let Some(index) = self.map.remove(&key) {
            self.slots[index] = None;
            self.open_slots.push(index);
        }
    }
}
use super::{Tab, Table};
use ffi::VAR_U;
/// The symbol table stores its values using a hash table.
/// All the hash table specific things live in this module.
use std::{
    array::{self, from_fn},
    collections::hash_map,
    default,
    ptr::null_mut,
};

use super::c_code::{ST_HASH, ST_MAX, SYMTAB};
const TAB_RAW_SIZE: usize = ST_MAX as usize + 1;
const HASH_RAW_SIZE: usize = ST_HASH as usize + 1;
///Some API calls give out the internal index where data has been stored
///This type represents a index that has come from a table and is therefore valid.
///I would rather just return references to the data, but that
///is not how the c code is structured so this will likely remain until
///everything is in rust.
///TODO remove type and just return references to the data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Index(i16);

impl Index {
    fn raw(raw: i16) -> Option<Self> {
        if raw == -1 {
            None
        } else {
            Some(Index(raw))
        }
    }
    fn to_raw(internal: Option<Self>) -> i16 {
        if let Some(val) = internal {
            val.0
        } else {
            -1
        }
    }
}

impl std::ops::Index<Index> for Table {
    type Output = SYMTAB;

    fn index(&self, index: Index) -> &Self::Output {
        self.slots[index.0 as usize].as_ref().unwrap()
    }
}

impl std::ops::IndexMut<Index> for Table {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        self.slots[index.0 as usize].as_mut().unwrap()
    }
}

/// The only error condition is if we run out of room in the table.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CreationError;

impl CreationError {
    fn error_code() -> i16 {
        -256
    }
}

#[cfg(test)]
mod tests {

    use std::ptr::null_mut;

    use pretty_assertions::assert_eq;
    use rand::{distributions::Alphanumeric, Rng};

    use crate::{shared_seg::lock_tab::tests::assert_eq, symbol_table::c_code::VarU};

    use super::{CreationError, Index, Table, ST_MAX};
    use rstest::*;

    //Some syntactic sugar around try_into/unwrap
    //to make the tests a bit cleaner.
    use super::super::tests::var_u;

    #[test]
    fn create() {
        let mut table = Table::new();
        for i in 0..ST_MAX as i16 {
            let var = var_u(&format!("var{i}"));
            let index = table.create(var.clone());
            //NOTE having sequential indexes probably improves cash locality
            assert_eq!(index, Ok(Index(i)));
            assert_eq!(table.locate(&var), Some(Index(i)));

            //Verify data has been reset
            //TODO create a better test for this.
            //usage and data are both zeroed during initialization.
            let node = &table[index.expect("Someness allready checked")];
            assert_eq!(VarU(node.varnam), var);
            assert_eq!({ node.usage }, 0);
            assert_eq!({ node.data }, null_mut());
        }

        let last_straw = var_u("lastStraw");
        let index = table.create(last_straw);
        assert_eq!(index, Err(CreationError));

        //There is a special node reserved for ECODE in the case that everything else has
        //been filed.
        let index = table.create(var_u("$ECODE"));
        assert_eq!(index, Ok(Index(ST_MAX as i16)));
    }

    #[test]
    fn error_code() {
        assert_eq!(CreationError::error_code(), -256);
    }

    #[test]
    fn create_free_create() {
        let mut table = Table::new();
        let var0 = var_u("var0");
        let var1 = var_u("var1");
        let var2 = var_u("var2");
        let var3 = var_u("var3");

        let _index0 = table.create(var0).unwrap();
        let index1 = table.create(var1.clone()).unwrap();
        let index2 = table.create(var2.clone()).unwrap();
        let _index3 = table.create(var3).unwrap();

        let var1_1 = var_u("var1.1");
        let var2_1 = var_u("var2.1");

        //notes are reused in a FILO manor
        table.free(&var1);
        table.free(&var2);
        assert_eq!(table.create(var2_1), Ok(index2));
        assert_eq!(table.create(var1_1), Ok(index1));
    }

    #[test]
    fn create_duplicates() {
        let mut table = Table::new();
        let var = var_u("varname");

        let first = table.create(var.clone());
        let second = table.create(var);
        assert_eq!(first, second);
    }

    #[test]
    fn locate_nonexistent_var() {
        let table = Table::new();
        assert_eq!(table.locate(&var_u("foo")), None);
    }

    #[test]
    fn free_resets_to_default() {
        let mut table = Table::new();
        let var = var_u("varname");
        let index = table.create(var.clone()).unwrap();
        table.free(&var);

        assert!(table.slots[index.0 as usize].is_none())
    }
}

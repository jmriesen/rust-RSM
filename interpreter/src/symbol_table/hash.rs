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
use std::{
    array,
    collections::{hash_map, HashMap},
    fmt::{self, Debug},
};

///Required trait bounds for Keys in the hashtable
pub trait Key: Eq + std::hash::Hash + Sized {
    //The special Error key.
    //Even if the map is full 'HashTable' will allow you to push this error.
    fn error() -> Self;
}

/// The only error condition is if we run out of room in the table.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CreationError;

impl CreationError {
    fn error_code() -> i16 {
        -256
    }
}

const NUMBER_OF_NORMAL_SLOTS: usize = 3072;
///The +1 is so that we always have room for the error key.
const NUMBER_OF_TOTAL_SLOTS: usize = NUMBER_OF_NORMAL_SLOTS + 1;
const ERROR_SLOT_INDEX: usize = NUMBER_OF_NORMAL_SLOTS;

/// Wrapper around the std `std::collections::HashMap` with a few additional properties.
/// Max number of entries is '`NUM_SLOTS`'*
/// If the map is full we can still insert a special Error Key.
/// Internally the keys are stored in a array, and the indexes into that array are handed out using
/// a stack.
#[allow(clippy::module_name_repetitions)]
pub struct HashTable<K, V>
where
    K: Key,
    V: Default,
{
    /// Maps keys to their internal index
    map: std::collections::HashMap<K, usize>,
    ///Stack storing which indexes are available
    open_slots: Vec<usize>,
    ///The actual data store
    slots: [Option<V>; NUMBER_OF_TOTAL_SLOTS],
}

impl<K, V> HashTable<K, V>
where
    K: Key,
    V: Default,
{
    pub fn new() -> Self {
        let mut open_slots = Vec::with_capacity(NUMBER_OF_NORMAL_SLOTS);
        for i in (0..NUMBER_OF_NORMAL_SLOTS).rev() {
            open_slots.push(i);
        }
        Self {
            open_slots,
            map: HashMap::default(),
            slots: array::from_fn(|_| None),
        }
    }

    pub fn create(&mut self, key: K) -> Result<&mut V, CreationError> {
        match self.map.entry(key) {
            hash_map::Entry::Occupied(entry) => Ok(self.slots[*entry.get()].as_mut().unwrap()),
            hash_map::Entry::Vacant(entry) => {
                let index = self
                    .open_slots
                    .pop()
                    //If slots are all filled check if we should use the error slot
                    .or_else(|| (entry.key() == &K::error()).then_some(ERROR_SLOT_INDEX));

                if let Some(new_slot_index) = index {
                    entry.insert(new_slot_index);
                    self.slots[new_slot_index] = Some(V::default());
                    Ok(self.slots[new_slot_index].as_mut().unwrap())
                } else {
                    Err(CreationError)
                }
            }
        }
    }

    pub fn locate(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|x| self.slots[*x].as_ref().unwrap())
    }

    pub fn locate_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get(key).map(|x| self.slots[*x].as_mut().unwrap())
    }

    pub fn free(&mut self, key: &K) {
        if let Some(index) = self.map.remove(key) {
            self.slots[index] = None;
            if index != ERROR_SLOT_INDEX {
                self.open_slots.push(index);
            }
        }
    }

    pub fn remove_if(&mut self, predict: impl Fn(&K) -> bool) {
        let mut to_be_removed: Vec<_> = self
            .map
            .extract_if(|key, _| predict(key))
            .map(|(_, index)| index)
            .collect();

        //NOTE the only reason I am sorting is so that the order of keys inserted back into open
        //slots is deterministic
        to_be_removed.sort_by(|a, b| a.cmp(b).reverse());
        self.open_slots.extend_from_slice(&to_be_removed);
        for index in &to_be_removed {
            self.slots[*index] = None;
        }
    }

    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map
            .iter()
            .map(|(k, v)| (k, self.slots[*v].as_ref().unwrap()))
    }
}

impl<K, V> Default for HashTable<K, V>
where
    K: Key,
    V: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(test, mutants::skip)]
impl<K, V> Debug for HashTable<K, V>
where
    K: Key + Debug,
    V: Default + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_map();
        builder.entries(self.iter());
        builder.finish()
    }
}

impl<K, V> PartialEq for HashTable<K, V>
where
    K: Key,
    V: Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for (key, value) in self.iter() {
            if Some(value) != other.locate(key) {
                return false;
            }
        }
        for (key, value) in other.iter() {
            if Some(value) != self.locate(key) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::{super::VarU, CreationError, ERROR_SLOT_INDEX, NUMBER_OF_NORMAL_SLOTS};
    use crate::symbol_table::{var_data::VarData, var_u::helpers::var_u};
    use pretty_assertions::assert_eq;
    use std::ptr::from_ref;

    //For testing I want to know that the references are the same.
    //These helper methods just convert things into pointers so I don't
    //have to worry about lifetimes.
    type Map = super::HashTable<VarU, VarData>;
    impl Map {
        fn locate_ptr(&mut self, key: &VarU) -> Option<*const VarData> {
            let pointer = self.locate(key).map(from_ref);
            assert_eq!(pointer, self.locate_mut(key).map(|x| from_ref(x)));
            pointer
        }
        fn create_ptr(&mut self, key: VarU) -> Result<*const VarData, CreationError> {
            self.create(key).map(|x| from_ref(x))
        }
        fn index_ptr(&self, index: usize) -> Option<*const VarData> {
            self.slots[index].as_ref().map(from_ref)
        }
    }

    #[test]
    fn create() {
        let mut table = Map::new();
        for i in 0..NUMBER_OF_NORMAL_SLOTS {
            let var = var_u(&format!("var{i}"));
            let index = table.create_ptr(var.clone());
            //NOTE having sequential indexes probably improves cash locality
            let expected = table.index_ptr(i).unwrap();
            assert_eq!(index, Ok(expected));
            assert_eq!(table.locate_ptr(&var), Some(expected));
        }

        let last_straw = var_u("lastStraw");
        let index = table.create(last_straw);
        assert_eq!(index, Err(CreationError));

        //There is a special node reserved for ECODE in the case that everything else has
        //been filed.
        let index = table.create_ptr(var_u("$ECODE"));
        assert_eq!(index, Ok(table.index_ptr(ERROR_SLOT_INDEX).unwrap()));
    }

    #[test]
    fn error_code() {
        assert_eq!(CreationError::error_code(), -256);
    }

    #[test]
    fn create_free_create() {
        let mut table = Map::new();
        let var0 = var_u("var0");
        let var1 = var_u("var1");
        let var2 = var_u("var2");
        let var3 = var_u("var3");

        let _index0 = table.create_ptr(var0).unwrap();
        let index1 = table.create_ptr(var1.clone()).unwrap();
        let index2 = table.create_ptr(var2.clone()).unwrap();
        let _index3 = table.create_ptr(var3).unwrap();

        let var1_1 = var_u("var1.1");
        let var2_1 = var_u("var2.1");

        //notes are reused in a FILO manor
        table.free(&var1);
        table.free(&var2);
        assert_eq!(table.create_ptr(var2_1), Ok(index2));
        assert_eq!(table.create_ptr(var1_1), Ok(index1));
    }

    #[test]
    fn create_duplicates() {
        let mut table = Map::new();
        let var = var_u("varname");

        let first = table.create_ptr(var.clone());
        let second = table.create_ptr(var);
        assert_eq!(first, second);
    }

    #[test]
    fn locate_nonexistent_var() {
        let table = Map::new();
        assert_eq!(table.locate(&var_u("foo")), None);
    }

    #[test]
    fn equality() {
        use rand::seq::SliceRandom;
        let mut keys = [var_u("a"), var_u("b"), var_u("c"), var_u("d"), var_u("e")];
        let mut first = Map::new();
        for key in keys.clone() {
            let _ = first.create(key);
        }

        keys.shuffle(&mut rand::thread_rng());
        let mut second = Map::new();
        for key in keys.clone() {
            let _ = second.create(key);
        }
        assert_eq!(first, second);

        let _ = second.create(var_u("f"));
        assert_ne!(first, second);
    }
}

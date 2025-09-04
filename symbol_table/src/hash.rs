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
    collections::{HashMap, hash_map},
    fmt::{self, Debug},
};

///Required trait bounds for Keys in the `HashTable`
pub trait Key: Eq + std::hash::Hash + Sized {
    //The special Error key.
    //Even if the map is full 'HashTable' will allow you to insert with this key.
    fn error() -> Self;
}

/// The HashTable is full so we could not insert a new entry.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CreationError;

/// A pre-allocated `HashTable` with a fixed capacity.
/// # Error handling
/// One slot is reserved for holding an error value and will always be available even when the rest
/// of the table is full.
/// Note the additional slot is used iff the all other slots are already full and when the error
/// value is inserted.
/// # Optimization
/// The slots are reused in a FILO manor.
/// This behavior was present in the original C code and has been ported for completeness.
/// I have not benchmarked this potential optimization against a standard has map.
#[allow(clippy::module_name_repetitions)]
pub struct HashTable<K, V, const SLOTS: usize = 3073>
where
    K: Key,
    V: Default,
{
    /// Maps keys to their internal index
    map: std::collections::HashMap<K, usize>,
    ///Stack storing which indexes are available
    open_slots: Vec<usize>,
    /// The actual data store
    slots: [Option<V>; SLOTS],
}

impl<K, V, const SLOTS: usize> HashTable<K, V, SLOTS>
where
    K: Key,
    V: Default,
{
    const NUMBER_OF_NORMAL_SLOTS: usize = SLOTS - 1;
    const ERROR_SLOT_INDEX: usize = SLOTS - 1;
    /// Create a new empty `HashTable`
    pub fn new() -> Self {
        let mut open_slots = Vec::with_capacity(Self::NUMBER_OF_NORMAL_SLOTS);
        for i in (0..Self::NUMBER_OF_NORMAL_SLOTS).rev() {
            open_slots.push(i);
        }
        Self {
            open_slots,
            map: HashMap::default(),
            slots: array::from_fn(|_| None),
        }
    }

    ///Attempts to create a new entry in the symbol table and returns a mutable reference to the
    ///entry.
    ///If an entry already exists for that key this just returns a reference to the existing entry.
    pub fn create_entry(&mut self, key: K) -> Result<&mut V, CreationError> {
        match self.map.entry(key) {
            hash_map::Entry::Occupied(entry) => Ok(self.slots[*entry.get()].as_mut().unwrap()),
            hash_map::Entry::Vacant(entry) => {
                let index = self
                    .open_slots
                    .pop()
                    //If slots are all filled check if we should use the error slot
                    .or_else(|| (entry.key() == &K::error()).then_some(Self::ERROR_SLOT_INDEX));

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

    ///Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|x| self.slots[*x].as_ref().unwrap())
    }

    ///Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get(key).map(|x| self.slots[*x].as_mut().unwrap())
    }

    /// Remove entry from the table.
    pub fn remove(&mut self, key: &K) {
        if let Some(index) = self.map.remove(key) {
            self.slots[index] = None;
            if index != Self::ERROR_SLOT_INDEX {
                self.open_slots.push(index);
            }
        }
    }

    /// For each element in the table:
    /// Apply the predicate function to that key, if it returns true remove the key from the table
    pub fn remove_if(&mut self, predicate: impl Fn(&K) -> bool) {
        let to_be_removed: Vec<_> = self
            .map
            .extract_if(|key, _| predicate(key))
            .map(|(_, index)| index)
            .collect();

        self.open_slots.extend_from_slice(&to_be_removed);
        for index in &to_be_removed {
            self.slots[*index] = None;
        }
    }

    /// Creates an iterator that returns key value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let mut iter: Vec<_> = self.map.iter().collect();
        iter.sort_by_key(|(_, index)| **index);
        iter.into_iter()
            .map(|(k, v)| (k, self.slots[*v].as_ref().unwrap()))
    }
}

impl<K, V, const SLOTS: usize> Default for HashTable<K, V, SLOTS>
where
    K: Key,
    V: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(test, mutants::skip)]
impl<K, V, const SLOTS: usize> Debug for HashTable<K, V, SLOTS>
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

impl<K, V, const SLOTS: usize> PartialEq for HashTable<K, V, SLOTS>
where
    K: Key,
    V: Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for (key, value) in self.iter() {
            if Some(value) != other.get(key) {
                return false;
            }
        }
        for (key, value) in other.iter() {
            if Some(value) != self.get(key) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::Key;
    use pretty_assertions::assert_eq;

    #[derive(std::hash::Hash, Eq, PartialEq, Clone, Copy, Debug)]
    enum ExampleKey {
        Normal(usize),
        Error,
    }
    use ExampleKey::*;
    impl Key for ExampleKey {
        fn error() -> Self {
            Self::Error
        }
    }
    type Map = super::HashTable<ExampleKey, isize, 5>;

    #[test]
    fn can_add_error_even_when_full() {
        let mut table = Map::new();
        for i in 0..Map::NUMBER_OF_NORMAL_SLOTS {
            assert!(table.create_entry(Normal(i)).is_ok());
        }
        assert!(
            table
                .create_entry(Normal(Map::NUMBER_OF_NORMAL_SLOTS))
                .is_err()
        );
        // We can always insert an error slot since we specificity reserve extra space for
        // it.
        assert!(table.create_entry(Error).is_ok());
    }

    #[test]
    fn adding_error_when_not_full_takes_up_a_slot() {
        //This test documents existing behavior, but I would be open to changing this.
        let mut table = Map::new();
        assert!(table.create_entry(Error).is_ok());
        for i in 0..Map::NUMBER_OF_NORMAL_SLOTS - 1 {
            assert!(table.create_entry(Normal(i)).is_ok());
        }
        assert!(
            table
                .create_entry(Normal(Map::NUMBER_OF_NORMAL_SLOTS))
                .is_err()
        );
    }

    #[test]
    fn equality_does_not_care_about_insertion_order() {
        use rand::seq::SliceRandom;
        let mut keys = [Normal(1), Normal(2), Normal(3)];
        let mut create_table = || {
            keys.shuffle(&mut rand::thread_rng());
            let mut map = Map::new();
            for key in keys.clone() {
                map.create_entry(key).unwrap();
            }
            map
        };
        let first = create_table();
        let mut second = create_table();

        assert_eq!(first, second);

        second.create_entry(Normal(4)).unwrap();
        assert_ne!(first, second);
    }
    #[test]
    fn reuse_existing_slot() {
        let mut table = Map::new();
        let key = Normal(0);
        let value = table.create_entry(key).unwrap();
        assert_eq!(value, &0);
        *value = 6;
        assert_eq!(*table.create_entry(key).unwrap(), 6);
    }

    #[test]
    fn return_none_if_key_is_not_in_the_map() {
        let table = Map::new();
        assert_eq!(table.get(&Normal(4)), None);
    }

    #[test]
    fn values_can_be_removed_from_the_map() {
        let mut table = Map::new();
        assert_eq!(table.get(&Normal(0)), None);
        let _ = table.create_entry(Normal(0)).unwrap();
        assert_eq!(table.get(&Normal(0)), Some(&0));
        table.remove(&Normal(0));
        assert_eq!(table.get(&Normal(0)), None);
    }
    #[test]
    fn values_can_be_modified_after_insertion() {
        let mut table = Map::new();
        assert_eq!(table.get(&Normal(0)), None);
        let _ = table.create_entry(Normal(0)).unwrap();
        assert_eq!(table.get(&Normal(0)), Some(&0));
        *table.get_mut(&Normal(0)).unwrap() = 5;
        assert_eq!(table.get(&Normal(0)), Some(&5));
    }

    #[test]
    fn slots_reused_in_a_FILO_order() {
        //The original C code reuses slots in a FILO manor.
        //I assume this is probably a performance optimization, however I have not
        //benchmarked it.
        let mut table = Map::new();

        *table.create_entry(Normal(1)).unwrap() = 1;
        *table.create_entry(Normal(2)).unwrap() = 2;
        *table.create_entry(Normal(3)).unwrap() = 3;
        *table.create_entry(Normal(4)).unwrap() = 4;

        assert_eq!(&table.slots, &[Some(1), Some(2), Some(3), Some(4), None]);
        //notes are reused in a FILO manor
        table.remove(&Normal(4));
        table.remove(&Normal(2));
        table.remove(&Normal(3));
        assert_eq!(&table.slots, &[Some(1), None, None, None, None]);

        *table.create_entry(Normal(5)).unwrap() = 5;
        *table.create_entry(Normal(6)).unwrap() = 6;
        *table.create_entry(Normal(7)).unwrap() = 7;
        assert_eq!(&table.slots, &[Some(1), Some(6), Some(5), Some(7), None]);
    }

    #[test]
    fn one_slot_is_reserved() {
        //This test hear just to satisfy mutation testing.
        //As of time of writing I cannot apply the mutation's skip attribute to const
        //items
        assert_eq!(Map::NUMBER_OF_NORMAL_SLOTS, 4)
    }
}

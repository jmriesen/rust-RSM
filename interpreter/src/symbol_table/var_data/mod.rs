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
use std::{collections::BTreeMap, ops::Bound};

use crate::{key::Key, value::Value};

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, PartialEq)]
pub struct DataResult {
    pub has_value: bool,
    pub has_descendants: bool,
}

impl From<DataResult> for Value {
    fn from(value: DataResult) -> Self {
        match value {
            DataResult {
                has_value: false,
                has_descendants: false,
            } => &b"0"[..],
            DataResult {
                has_value: true,
                has_descendants: false,
            } => &b"1"[..],
            DataResult {
                has_value: false,
                has_descendants: true,
            } => &b"10"[..],
            DataResult {
                has_value: true,
                has_descendants: true,
            } => &b"11"[..],
        }
        .try_into()
        .expect("all data values should convert without issue")
    }
}

///Data associated for a specific variable
#[derive(Debug, PartialEq, Eq, Default)]
pub struct VarData {
    sub_values: BTreeMap<Key, Value>,
    //TODO consider removing I am currently not using attach
    attach: ::std::os::raw::c_short,
    value: Option<Value>,
}

impl VarData {
    pub fn value(&self, key: &Key) -> Option<&Value> {
        if key.is_empty() {
            self.value.as_ref()
        } else {
            //NOTE There is probably room for optimization here.
            //It is fairly common for the M code to access the values sequentially using $O
            //The C code speed up $O by string a reference to the last used node.
            //I am not doing this right now as it would require making a self referential type
            //and I am not focusing on performance right now. (just correctness)
            //NOTE you could also probably accomplish the last key thing using using a sorted vec
            self.sub_values.get(key)
        }
    }

    pub fn set_value(&mut self, key: &Key, data: &Value) {
        if key.is_empty() {
            self.value = Some(data.clone());
        } else {
            let _ = self.sub_values.insert(key.clone(), data.clone());
        }
    }

    pub fn kill(&mut self, key: &Key) {
        if key.is_empty() {
            //Clear values
            self.sub_values = BTreeMap::default();
            self.value = None;
        } else {
            //NOTE Removing a range of keys seems like something the std BTree map should support,
            //However it looks like there is still some design swirl going on, and the design has
            //not been touched in a while
            //https://github.com/rust-lang/rust/issues/81074
            //So I just chose to use the cursor api for now.
            let mut cursor = self.sub_values.lower_bound_mut(Bound::Included(key));
            while let Some((current_key, _)) = cursor.peek_next()
                && current_key.is_sub_key_of(key)
            {
                cursor.remove_next();
            }
        }
    }

    pub fn data(&self, key: &Key) -> DataResult {
        if key.is_empty() {
            DataResult {
                has_value: self.value.is_some(),
                has_descendants: !self.sub_values.is_empty(),
            }
        } else {
            DataResult {
                has_value: self.sub_values.contains_key(key),
                has_descendants: self
                    .sub_values
                    .lower_bound(Bound::Excluded(key))
                    .next()
                    .is_some_and(|(x, _)| x.is_sub_key_of(key)),
            }
        }
    }

    pub fn query(&self, key: &Key, direction: Direction) -> Option<&Key> {
        match direction {
            Direction::Forward => self.sub_values.lower_bound(Bound::Excluded(key)).next(),
            Direction::Backward => self
                .sub_values
                .upper_bound(Bound::Excluded(&key.wrap_null_tail()))
                .prev(),
        }
        .map(|x| x.0)
    }

    pub fn order(&self, key: &Key, direction: Direction) -> Option<crate::key::Segment> {
        let sub_len = key.iter().count();
        match direction {
            Direction::Forward => self
                .sub_values
                .lower_bound(Bound::Excluded(&key.upper_subscript_bound()))
                .next(),
            Direction::Backward => self
                .sub_values
                .upper_bound(Bound::Excluded(&key.wrap_null_tail()))
                .prev(),
        }
        .map(|x| x.0)
        .and_then(|key| key.iter().nth(sub_len - 1))
    }

    //todo
    //Dump

    //checks self contains any data, if not it can be freed
    //TODO I don't really understand how attached is supposed to work so am skipping mutation testing
    //on this for the moment
    #[cfg_attr(test, mutants::skip)]
    pub fn contains_data(&self) -> bool {
        !(self.sub_values.is_empty() && self.attach <= 1 && self.value.is_none())
    }
}

#[cfg(test)]
mod tests;

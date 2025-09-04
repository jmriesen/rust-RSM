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
use std::{borrow::Borrow, collections::BTreeMap, ops::Bound};

use serde::{Deserialize, Serialize};

use crate::key::{self, Key, KeyBound};
use value::{self, Value};

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
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
    sub_values: BTreeMap<KeyBound, Value>,
    value: Option<Value>,
}

impl VarData {
    pub fn value(&self, key: &Key) -> Option<&Value> {
        let key: &KeyBound = key.borrow();
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
        let key: &KeyBound = key.borrow();
        if key.is_empty() {
            self.value = Some(data.clone());
        } else {
            let _ = self.sub_values.insert(key.clone(), data.clone());
        }
    }

    pub fn kill(&mut self, key: &Key) {
        let key: &KeyBound = key.borrow();
        if key.is_empty() {
            //Clear values
            self.sub_values = BTreeMap::default();
            self.value = None;
        } else {
            //NOTE Removing a range of keys seems like something the std BTree map should support,
            //However it looks like there is still some design swirl going on, and the design has
            //not been touched in a while.
            //https://github.com/rust-lang/rust/issues/81074
            //So I just chose to use the cursor API for now.
            let mut cursor = self.sub_values.lower_bound_mut(Bound::Included(key));
            while let Some((current_key, _)) = cursor.peek_next()
                && current_key.is_sub_key_of(key)
            {
                cursor.remove_next();
            }
        }
    }

    pub fn data(&self, key: &Key) -> DataResult {
        let key: &KeyBound = key.borrow();
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

    pub fn query(&self, key: &KeyBound, direction: Direction) -> Option<Key> {
        static EMPTY: Value = Value::empty();
        match direction {
            Direction::Forward => self.sub_values.lower_bound(Bound::Excluded(key)).next(),
            Direction::Backward => {
                self.sub_values
                    .upper_bound(Bound::Excluded(&key.wrap_if_null_tail()))
                    .prev()
                    //If going backwards we also have to check the root
                    .or((!key.is_empty() && self.value.is_some())
                        .then_some((&key::EMPTY_BOUND, &EMPTY)))
            }
        }
        .map(|x| {
            x.0.clone()
                .try_into()
                .expect("sub_values should only store NonNullableKeys")
        })
    }

    pub fn order(&self, key: &KeyBound, direction: Direction) -> Option<crate::key::SubKey<'_>> {
        match direction {
            Direction::Forward => self
                .sub_values
                .lower_bound(Bound::Excluded(&key.upper_subscript_bound()))
                .next(),
            Direction::Backward => self
                .sub_values
                .upper_bound(Bound::Excluded(&key.wrap_if_null_tail()))
                .prev(),
        }
        .map(|x| x.0)
        .and_then(|x| key.extract_sibling_sub_key(x))
    }

    pub fn has_data(&self) -> bool {
        self.data(&Key::empty())
            != DataResult {
                has_value: false,
                has_descendants: false,
            }
    }
}

#[cfg(test)]
mod tests;

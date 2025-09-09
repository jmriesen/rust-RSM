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
use crate::key::{self, Path, PathBound};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::BTreeMap, ops::Bound};
use value::{self, Value};

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
/// The direction in which to iterate through a variable's keys.
pub enum Direction {
    Forward,
    Backward,
}

/// The result of the $Data intrinsic function.
///
/// When converted into a value the first bit represents if the node has descendants.
/// The second represents if the node has a value.
/// ```
/// use value::Value;
/// use symbol_table::DataResult;
/// let value = |x| Value::try_from(x).unwrap();
///       assert_eq!(
///           value("0"),
///           DataResult {
///               has_value: false,
///               has_descendants: false,
///           }
///           .into(),
///       );
///       assert_eq!(
///           value("1"),
///           DataResult {
///               has_value: true,
///               has_descendants: false,
///           }
///           .into(),
///       );
///       assert_eq!(
///           value("10"),
///           DataResult {
///               has_value: false,
///               has_descendants: true,
///           }
///           .into(),
///       );
///       assert_eq!(
///           value("11"),
///           DataResult {
///               has_value: true,
///               has_descendants: true,
///           }
///           .into(),
///       );
/// ```
#[derive(Debug, PartialEq)]
pub struct DataResult {
    /// Is the value at the node non-null
    pub has_value: bool,
    /// Does the node have children.
    pub has_descendants: bool,
}

impl From<DataResult> for Value {
    fn from(value: DataResult) -> Self {
        let bytes: &[u8] = match value {
            DataResult {
                has_value: false,
                has_descendants: false,
            } => b"0",
            DataResult {
                has_value: true,
                has_descendants: false,
            } => b"1",
            DataResult {
                has_value: false,
                has_descendants: true,
            } => b"10",
            DataResult {
                has_value: true,
                has_descendants: true,
            } => b"11",
        };
        bytes
            .try_into()
            .expect("all data values should convert without issue")
    }
}

/// A tree of nullable values.
/// Tree paths are specified by sequences of keys.
/// Each layer of the tree is sorted by the key.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ValueTree {
    // Value of the tree root.
    value: Option<Value>,
    // Values of all the other nodes.
    // They are internally stored in a flattened in a sorted list structure.
    sub_values: BTreeMap<PathBound, Value>,
}

impl ValueTree {
    /// Get a value of a node.
    pub fn value(&self, key: &Path) -> Option<&Value> {
        let key: &PathBound = key.borrow();
        if key.is_empty() {
            self.value.as_ref()
        } else {
            //NOTE There is probably room for optimization here.
            //It is fairly common for the M code to access the values sequentially using $O
            //The C code speed up $O by string a reference to the last used node.
            //I am not doing this right now as it would require making a self referential type
            //and I am not focusing on performance right now. (just correctness)
            //NOTE you could also probably accomplish the last key thing using a sorted vec
            self.sub_values.get(key)
        }
    }

    /// Sets the value of a given node.
    pub fn set_value(&mut self, key: &Path, data: &Value) {
        let key: &PathBound = key.borrow();
        if key.is_empty() {
            self.value = Some(data.clone());
        } else {
            let _ = self.sub_values.insert(key.clone(), data.clone());
        }
    }

    /// Remove a node from the tree.
    /// This also removes all of that notes children.
    pub fn kill(&mut self, key: &Path) {
        let key: &PathBound = key.borrow();
        if key.is_empty() {
            self.value = None;
            self.sub_values = BTreeMap::default();
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

    /// Checks if a given node has a value, and if it has descendants.
    ///
    /// Corresponds to the M intrinsic data function.
    pub fn data(&self, key: &Path) -> DataResult {
        let key: &PathBound = key.borrow();
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

    /// Returns the path to the next node in the tree.
    ///
    /// Corresponds to the M intrinsic query function.
    pub fn query(&self, key: &PathBound, direction: Direction) -> Option<Path> {
        static EMPTY: Value = Value::empty();
        static EMPTY_BOUND: PathBound = PathBound::empty();
        match direction {
            Direction::Forward => self.sub_values.lower_bound(Bound::Excluded(key)).next(),
            Direction::Backward => {
                self.sub_values
                    .upper_bound(Bound::Excluded(&key.wrap_if_null_tail()))
                    .prev()
                    //If going backwards we also have to check the root
                    .or((!key.is_empty() && self.value.is_some()).then_some((&EMPTY_BOUND, &EMPTY)))
            }
        }
        .map(|x| {
            x.0.clone()
                .try_into()
                .expect("sub_values should only store NonNullableKeys")
        })
    }

    /// Find the next node in the same level as the given node and return the last segment of its
    /// path.
    pub fn order(&self, key: &PathBound, direction: Direction) -> Option<crate::key::Segment<'_>> {
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

    /// Check if this tree contains any values.
    pub fn not_empty(&self) -> bool {
        self.data(&Path::empty())
            != DataResult {
                has_value: false,
                has_descendants: false,
            }
    }
}

#[cfg(test)]
mod tests;

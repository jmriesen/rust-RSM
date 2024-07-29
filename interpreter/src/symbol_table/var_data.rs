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
use std::collections::BTreeMap;

use crate::{key::Key, value::Value};

///Data associated for a specific variable
#[derive(Debug, PartialEq, Eq)]
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
            let mut cursor = self
                .sub_values
                .lower_bound_mut(std::ops::Bound::Included(key));
            while let Some((current_key, _)) = cursor.peek_next()
                && current_key.is_sub_key_of(key)
            {
                cursor.remove_next();
            }
        }
    }

    pub fn data(&self, key: &Key) -> (bool, bool) {
        if key.is_empty() {
            (!self.sub_values.is_empty(), self.value.is_some())
        } else {
            let mut cursor = self.sub_values.lower_bound(std::ops::Bound::Included(key));
            if let Some(node) = cursor.next() {
                if node.0 == key {
                    (
                        cursor
                            .next()
                            .filter(|(key, _)| key.is_sub_key_of(key))
                            .is_some(),
                        true,
                    )
                } else {
                    (node.0.is_sub_key_of(key), false)
                }
            } else {
                (false, false)
            }
        }
    }

    //todo Data
    //Order
    //Query
    //Dump

    //checks self contains any data, if not it can be freed
    //TODO I don't really understand how attached is supposed to work so am skipping mutation testing
    //on this for the moment
    #[cfg_attr(test, mutants::skip)]
    pub fn contains_data(&self) -> bool {
        !(self.sub_values.is_empty() && self.attach <= 1 && self.value.is_none())
    }
}

impl Default for VarData {
    fn default() -> Self {
        Self {
            sub_values: Default::default(),
            attach: 0,
            value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    mod data {

        use crate::{
            symbol_table::{m_var::helpers::var_m, Table},
            value::Value,
        };

        #[test]
        fn root_data() {
            let mut table = Table::new();
            let var = var_m("var", &[]);
            assert!(!table.data(&var).1);
            let data: Value = "data".try_into().unwrap();
            let _ = table.set(&var, &data);
            assert!(table.data(&var).1);
        }
        #[test]
        fn root_descendants() {
            let mut table = Table::new();
            let root = var_m("var", &[]);
            let sub = var_m("var", &["sub"]);
            assert!(!table.data(&root).0);
            let data: Value = "data".try_into().unwrap();
            let _ = table.set(&sub, &data);
            assert!(table.data(&root).0);
        }
        #[test]
        fn sub_key_data() {
            let mut table = Table::new();
            let var = var_m("var", &["sub"]);
            assert!(!table.data(&var).1);
            let data: Value = "data".try_into().unwrap();
            let _ = table.set(&var, &data);
            assert!(table.data(&var).1);
        }
        #[test]
        fn sub_key_descendants() {
            let mut table = Table::new();
            let sub = var_m("var", &[]);
            let sub_sub = var_m("var", &["sub"]);
            assert!(!table.data(&sub).0);
            let data: Value = "data".try_into().unwrap();
            let _ = table.set(&sub_sub, &data);
            assert!(table.data(&sub).0);
        }
    }
}

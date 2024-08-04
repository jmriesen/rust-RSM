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

pub enum Direction {
    Forward,
    Backward,
}

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
            let mut cursor = self.sub_values.lower_bound_mut(Bound::Included(key));
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
            (
                self.sub_values
                    .lower_bound(Bound::Excluded(key))
                    .next()
                    .map(|(key, _)| key.is_sub_key_of(key))
                    .unwrap_or(false),
                self.sub_values.contains_key(key),
            )
        }
    }

    pub fn query(&self, key: &Key, direction: Direction) -> Option<&Key> {
        match direction {
            Direction::Forward => self.sub_values.lower_bound(Bound::Excluded(key)).next(),
            Direction::Backward => self
                .sub_values
                .upper_bound(Bound::Excluded(&key.wrap_null_key()))
                .prev(),
        }
        .map(|x| x.0)
    }

    pub fn order(&self, key: &Key, direction: Direction) -> Value {
        dbg!(&self.sub_values);
        dbg!(key);
        let sub_len = key.iter().count();
        match direction {
            Direction::Forward => self
                .sub_values
                .lower_bound(Bound::Excluded(&key.upper_subscript_bound()))
                .next(),
            Direction::Backward => self
                .sub_values
                .upper_bound(Bound::Excluded(&key.wrap_null_key()))
                .prev(),
        }
        .map(|x| x.0)
        .and_then(|key| dbg!(&key).iter().skip(sub_len - 1).next())
        .map(|x| x.into())
        .unwrap_or_default()
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
    use crate::{
        symbol_table::{m_var::helpers::var_m, Table},
        value::Value,
    };
    mod data {
        use super::*;

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

    mod query {

        use super::*;
        use crate::symbol_table::var_data::Direction;

        #[test]
        fn forward_and_backward() {
            let keys: [&[&str]; 4] = [&["-1"], &["0"], &["0", "1"], &["a"]];
            let mut m_vars: Vec<_> = keys.map(|x| var_m("foo", x)).to_vec();

            let mut table = Table::new();
            for var in &m_vars {
                table.set(&var, &Value::try_from("Value").unwrap()).unwrap();
            }

            m_vars.insert(0, var_m("foo", &[""]));
            m_vars.push(var_m("foo", &[""]));

            for i in 0..m_vars.len() - 2 {
                assert_eq!(
                    table.query(&m_vars[i], Direction::Forward),
                    format!("{}", m_vars[i + 1])
                );
            }
            for i in (2..m_vars.len()).rev() {
                assert_eq!(
                    table.query(&m_vars[i], Direction::Backward),
                    format!("{}", m_vars[i - 1])
                );
            }
        }

        #[test]
        fn value_with_no_subscripts() {
            let mut table = super::Table::new();
            let _ = table.set(&var_m("foo", &[]), &Value::try_from("Value").unwrap());
            assert_eq!(table.query(&var_m("foo", &[""]), Direction::Forward), "");
            assert_eq!(table.query(&var_m("foo", &["bar"]), Direction::Forward), "");
            assert_eq!(table.query(&var_m("foo", &[""]), Direction::Backward), "");
            assert_eq!(
                table.query(&var_m("foo", &["bar"]), Direction::Backward),
                ""
            );
        }

        ///A trailing null is treated as the fist key while moving forward and the last key when moving
        ///backwards.
        #[test]
        fn moving_backwards_null_subscript() {
            let keys: [&[&str]; 5] = [&["-1"], &["0"], &["0", "-1"], &["0", "1"], &["1"]];
            let m_vars = keys.map(|x| var_m("foo", x));

            let mut table = Table::new();
            for var in &m_vars {
                table.set(&var, &Value::try_from("Value").unwrap()).unwrap()
            }
            let null_last_key = var_m("foo", &["0", ""]);
            assert_eq!(
                table.query(&null_last_key, Direction::Forward),
                format!("{}", m_vars[2])
            );
            assert_eq!(
                table.query(&null_last_key, Direction::Backward),
                format!("{}", m_vars[3])
            );
        }

        ///If in the middle of the key null is treated normally (null< everything else)
        #[test]
        fn null_in_the_middle() {
            let keys: [&[&str]; 4] = [&["-1"], &["0", "1", "-1"], &["0", "1", "1"], &["1"]];
            let m_vars = keys.map(|x| var_m("foo", x));

            let mut table = Table::new();
            for var in &m_vars {
                table.set(&var, &Value::try_from("Value").unwrap()).unwrap()
            }

            let null_in_middle = var_m("foo", &["0", "", "0"]);
            assert_eq!(
                table.query(&null_in_middle, Direction::Forward),
                format!("{}", m_vars[1])
            );
            assert_eq!(
                table.query(&null_in_middle, Direction::Backward),
                format!("{}", m_vars[0])
            );
        }
    }

    mod order {

        use crate::symbol_table::var_data::Direction;

        use super::*;

        #[test]
        fn forward_and_backward() {
            let mut table = Table::new();
            let data = Value::try_from("data").unwrap();
            table.set(&var_m("foo", &["0"]), &data).unwrap();
            table.set(&var_m("foo", &["1", "a"]), &data).unwrap();
            table.set(&var_m("foo", &["1", "b"]), &data).unwrap();
            table.set(&var_m("foo", &["2"]), &data).unwrap();

            //Top level forward
            assert_eq!(
                table.order(&var_m("foo", &[""]), Direction::Forward),
                "0".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["0"]), Direction::Forward),
                "1".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["1"]), Direction::Forward),
                "2".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["2"]), Direction::Forward),
                "".try_into().unwrap(),
            );

            //Top level Backwords
            assert_eq!(
                table.order(&var_m("foo", &[""]), Direction::Backward),
                "2".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["2"]), Direction::Backward),
                "1".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["1"]), Direction::Backward),
                "0".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["0"]), Direction::Backward),
                "".try_into().unwrap(),
            );

            //sub layer Forward
            assert_eq!(
                table.order(&var_m("foo", &["1", ""]), Direction::Forward),
                "a".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["1", "a"]), Direction::Forward),
                "b".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["1", "b"]), Direction::Forward),
                "".try_into().unwrap(),
            );

            //Top level Backwords
            assert_eq!(
                table.order(&var_m("foo", &["1", ""]), Direction::Backward),
                "b".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["1", "b"]), Direction::Backward),
                "a".try_into().unwrap(),
            );
            assert_eq!(
                table.order(&var_m("foo", &["1", "a"]), Direction::Backward),
                "".try_into().unwrap(),
            );
        }

        #[test]
        fn value_with_no_subscripts() {
            let mut table = ffi::symbol_table::Table::new();
            let foo = var_m("foo", &[]).into_cmvar();
            let bar = var_m("bar", &["subscript"]).into_cmvar();
            let _ = table.set(&foo, &Value::try_from("Value").unwrap().into_cstring());
            let _ = table.set(&bar, &Value::try_from("Value").unwrap().into_cstring());
            assert_eq!("", String::from_utf8(table.order(&foo, false)).unwrap());
            assert_eq!("", String::from_utf8(table.order(&bar, false)).unwrap());
            assert_eq!("", String::from_utf8(table.order(&foo, true)).unwrap());
            assert_eq!("", String::from_utf8(table.order(&bar, true)).unwrap());
        }
    }
}

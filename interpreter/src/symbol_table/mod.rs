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
#![allow(dead_code)]

#[allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
mod c_code {
    //Generated code violates a lot of formatting stuff conventions.
    //Pointless to warn about all of them
    #![allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]

    use ffi::u_char;

    use std::{collections::BTreeMap, hash::Hash, sync::Mutex};

    //New type wrapper so I can implement methods on VAR_U
    //TODO decouple from ffi
    #[derive(Clone, Debug)]
    pub struct VarU(pub ffi::VAR_U);

    use crate::{key::Key, value::Value};
    pub static lock: Mutex<()> = Mutex::new(());

    impl std::fmt::Debug for MVAR {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //TODO implement a more complete implementation.
            //Currently this is just enough to start fuzz testing.
            let name = String::from_utf8(self.name.0.as_array().into()).unwrap();

            let mut builder = f.debug_struct("MVar");
            builder
                .field("name", &name)
                .field("key", &self.key)
                .finish()
        }
    }

    //TODO make fields private
    #[derive(Clone)]
    pub struct MVAR {
        pub name: VarU,
        pub volset: u_char,
        pub uci: u_char,
        pub key: Key,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ST_DATA {
        pub sub_values: BTreeMap<Key, Value>,
        //TODO consider removing I am currently not using attach
        pub attach: ::std::os::raw::c_short,
        pub value: Option<Value>,
    }

    impl Default for ST_DATA {
        fn default() -> Self {
            Self {
                sub_values: Default::default(),
                attach: 0,
                value: None,
            }
        }
    }

    impl Hash for VarU {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            unsafe { self.0.var_cu }.hash(state)
        }
    }

    impl Eq for VarU {}
    impl PartialEq for VarU {
        fn eq(&self, other: &Self) -> bool {
            unsafe { self.0.var_cu == other.0.var_cu }
        }
    }

    impl super::hash::Key for VarU {
        fn error() -> Self {
            Self("$ECODE".try_into().expect("the error key is a valid VarU"))
        }
    }
    pub type table_struct = super::hash::HashTable<VarU, ST_DATA>;

    pub type Table = table_struct;
}

use std::collections::BTreeMap;

pub use c_code::{Table, MVAR};
//TODO remove and replace with derive once type move over to Rust
mod hash;
mod manual;

use c_code::{VarU, ST_DATA};
use ffi::{PARTAB, UCI_IS_LOCALVAR};

use crate::{key::Key, value::Value};

impl ST_DATA {
    fn value(&self, key: &Key) -> Option<Value> {
        //TODO remove clones
        if key.is_empty() {
            self.value.clone()
        } else {
            //NOTE There is probably room for optimization here.
            //It is fairly common for the M code to access the values sequentially using $O
            //The C code speed up $O by string a reference to the last used node.
            //I am not doing this right now as it would require making a self referential type
            //and I am not focusing on performance right now. (just correctness)
            //NOTE you could also probably accomplish the last key thing using using a sorted vec
            self.sub_values.get(key).cloned()
        }
    }

    fn set_value(&mut self, key: &Key, data: &Value) {
        if key.is_empty() {
            self.value = Some(data.clone());
        } else {
            let _ = self.sub_values.insert(key.clone(), data.clone());
        }
    }

    fn kill(&mut self, key: &Key) {
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

    //checks self contains any data, if not it can be freed
    //TODO I don't really understand how attached is supposed to work so am skipping mutation testing
    //on this for the moment
    #[cfg_attr(test, mutants::skip)]
    fn contains_data(&self) -> bool {
        !(self.sub_values.is_empty() && self.attach <= 1 && self.value.is_none())
    }
}

impl Table {
    #[must_use]
    pub fn get(&self, var: &MVAR) -> Option<Value> {
        self.locate(&var.name)
            .and_then(|index| self[index].value(&var.key))
    }

    pub fn set(&mut self, var: &MVAR, value: &Value) -> Result<(), ()> {
        let index = self.create(var.name.clone()).map_err(|_| ())?;
        self[index].set_value(&var.key, value);
        Ok(())
    }

    pub fn kill(&mut self, var: &MVAR) {
        if let Some(index) = self.locate(&var.name) {
            self[index].kill(&var.key);
            if !self[index].contains_data() {
                self.free(&var.name);
            }
        }
    }

    //NOTE not yet mutation tested
    fn keep(&mut self, vars: &[VarU], tab: &mut PARTAB) {
        //NOTE I am not sure how src_var is used, but this was done in the C code.
        tab.src_var = ffi::MVAR {
            uci: UCI_IS_LOCALVAR as u8,
            slen: 0,
            volset: 0,
            ..tab.src_var
        };
        let mut to_be_removed: Vec<_> = self
            .map
            .extract_if(|x, _| !vars.contains(x) && unsafe { x.0.var_cu[0] } != b'$')
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
}
#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use arbitrary::Arbitrary;

    use crate::{key::Key, value::Value};

    use super::c_code::{VarU, MVAR};
    use ffi::VAR_U;
    #[must_use]
    pub fn var_u(var: &str) -> VarU {
        VarU(var.try_into().unwrap())
    }

    #[must_use]
    pub fn var_m(name: &str, values: &[&str]) -> MVAR {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = Key::new(&values).unwrap();

        MVAR {
            name: var_u(name),
            volset: Default::default(),
            uci: Default::default(),
            key,
        }
    }

    impl<'a> Arbitrary<'a> for MVAR {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let name: [u8; 32] = u.arbitrary()?;
            if name.is_ascii() && name.contains(&0) {
                Ok(MVAR {
                    name: VarU(VAR_U { var_cu: name }),
                    volset: 0,
                    uci: 0,
                    //TODO implement arbitrary for key.
                    key: Key::empty(),
                })
            } else {
                Err(arbitrary::Error::IncorrectFormat)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {

    pub use super::helpers::{var_m, var_u};
    use ffi::UCI_IS_LOCALVAR;
    use pretty_assertions::assert_eq;

    use super::c_code::Table;
    #[test]
    fn get_unset_variable() {
        let table = Table::new();
        let m_var = var_m("foo", &[]);
        assert_eq!(table.get(&m_var), None);
    }
    #[test]
    fn get_unset_key() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut data = "Data".try_into().unwrap();
        table.set(&mut m_var, &mut data).unwrap();

        let mut m_var = var_m("foo", &["bar"]);
        assert_eq!(table.get(&mut m_var), None);
    }

    #[test]
    fn set_root_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut data = "Data".try_into().unwrap();

        table.set(&mut m_var, &mut data).unwrap();
        assert_eq!(table.get(&mut m_var), Some(data));
    }

    #[test]
    fn set_index_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &["keys"]);
        let mut data = "Data".try_into().unwrap();
        table.set(&mut m_var, &mut data).unwrap();
        assert_eq!(Some(data), table.get(&mut m_var));
    }

    #[test]
    fn set_root_then_index() {
        let mut root = var_m("foo", &[]);
        let mut root_data = "root Data".try_into().unwrap();
        let mut with_key = var_m("foo", &["keys"]);
        let mut key_data = "key Data".try_into().unwrap();
        {
            let mut table = Table::new();

            table.set(&mut root, &mut root_data).unwrap();
            table.set(&mut with_key, &mut key_data).unwrap();
            assert_eq!(Some(root_data.clone()), table.get(&mut root));
            assert_eq!(Some(key_data.clone()), table.get(&mut with_key));
        }
        {
            let mut table = Table::new();

            table.set(&mut with_key, &mut key_data).unwrap();
            table.set(&mut root, &mut root_data).unwrap();
            assert_eq!(Some(root_data.clone()), table.get(&mut root));
            assert_eq!(Some(key_data.clone()), table.get(&mut with_key));
        }
    }

    #[test]
    fn set_null_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut data = "".try_into().unwrap();

        table.set(&mut m_var, &mut data).unwrap();
        assert_eq!(Some(data), table.get(&mut m_var));
    }

    #[test]
    fn set_works_while_with_prefixs() {
        let mut table = Table::new();
        let mut prefix = var_m("foo", &["prefix"]);
        let mut full = var_m("foo", &["prefixAndMore"]);
        let mut prefix_data = "prefix".try_into().unwrap();
        let mut full_data = "full".try_into().unwrap();

        table.set(&mut prefix, &mut prefix_data).unwrap();
        table.set(&mut full, &mut full_data).unwrap();
        assert_eq!(Some(prefix_data), table.get(&mut prefix));
        assert_eq!(Some(full_data), table.get(&mut full));
    }

    #[test]
    fn set_overrides_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut initial_value = "inital".try_into().unwrap();
        let mut end_value = "end".try_into().unwrap();

        table.set(&mut m_var, &mut initial_value).unwrap();
        assert_eq!(Some(initial_value), table.get(&mut m_var));

        table.set(&mut m_var, &mut end_value).unwrap();
        assert_eq!(Some(end_value), table.get(&mut m_var));
    }

    #[test]
    fn do_a_bunch_of_sets() {
        let test_data = [
            (vec![], ""),
            (vec!["SomeKey0"], "someKey0"),
            (vec!["SomeKey1"], "someKey1"),
            (vec!["SomeKey2"], "someKey2"),
            (vec!["SomeKey3"], "someKey3"),
            (vec!["SomeKey4"], "someKey4"),
            (vec!["lots", "of", "Keys", "even", "more"], "lots of keys"),
        ];
        let mut test_data = test_data
            .map(|(keys, value)| (var_m("foo", &keys), Box::new(value.try_into().unwrap())));

        let mut table = Table::new();
        for (var, value) in &mut test_data {
            table.set(var, value).unwrap();
        }

        for (mut var, value) in test_data {
            assert_eq!(Some(*value), table.get(&mut var));
        }
    }

    #[test]
    fn kill_uninitialized_var() {
        let mut table = Table::new();
        //These should be noops.
        table.kill(&mut var_m("foo", &[]));
        table.kill(&mut var_m("foo", &["arg"]));
    }

    #[test]
    fn kill_initialized_root() {
        let mut table = Table::new();
        let mut var = var_m("foo", &[]);
        let var_i = var_m("foo", &["i"]);
        let var_ii = var_m("foo", &["i", "ii"]);
        let data = "data".try_into().unwrap();
        table.set(&var, &data).unwrap();
        table.set(&var_i, &data).unwrap();
        table.set(&var_ii, &data).unwrap();
        table.kill(&mut var);
        assert_eq!(table.get(&var), None);
        assert_eq!(table.get(&var_i), None);
        assert_eq!(table.get(&var_i), None);

        //hash table should have freed the entire.
        assert_eq!(table.locate(&var_u("foo")), None);
    }

    #[test]
    fn kill_initialized_index() {
        let mut table = Table::new();
        let var = var_m("foo", &[]);
        let mut var_i = var_m("foo", &["i"]);
        let var_ii = var_m("foo", &["i", "ii"]);
        let data = "data".try_into().unwrap();
        table.set(&var, &data).unwrap();
        table.set(&var_i, &data).unwrap();
        table.set(&var_ii, &data).unwrap();
        table.kill(&mut var_i);
        assert_eq!(table.get(&var), Some(data));
        assert_eq!(table.get(&var_i), None);
        assert_eq!(table.get(&var_i), None);
    }

    #[test]
    fn kill_removes_only_specified_index() {
        let mut table = Table::new();
        let data = "data".try_into().unwrap();
        let a = var_m("foo", &["a"]);
        let b = var_m("foo", &["b"]);
        let c = var_m("foo", &["c"]);

        table.set(&a, &data).unwrap();
        table.set(&b, &data).unwrap();
        table.set(&c, &data).unwrap();
        table.kill(&b);
        assert_eq!(table.get(&a), Some(data.clone()));
        assert_eq!(table.get(&b), None);
        assert_eq!(table.get(&c), Some(data));
    }

    #[test]
    fn keep_vars() {
        let mut table = Table::new();
        let data = "data".try_into().unwrap();
        let dolor = var_m("$dolor", &[]);
        let normal = var_m("normal", &[]);
        let retain_a = var_m("retain_a", &[]);
        let retain_b = var_m("retain_b", &[]);
        table.set(&dolor, &data).unwrap();
        table.set(&normal, &data).unwrap();
        table.set(&retain_a, &data).unwrap();
        table.set(&retain_b, &data).unwrap();
        let mut part_tab = Default::default();

        table.keep(
            &[retain_a.name.clone(), retain_b.name.clone()],
            &mut part_tab,
        );
        assert_eq!(part_tab.src_var.uci, UCI_IS_LOCALVAR as u8);
        assert_eq!(part_tab.src_var.slen, 0);
        assert_eq!(part_tab.src_var.volset, 0);

        assert_eq!(table.get(&normal), None);
        for var in &[dolor, retain_a, retain_b] {
            assert_eq!(table.get(var), Some(data.clone()));
        }
    }
}

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

mod var_data;
use ffi::u_char;
use var_data::VarData;

use std::hash::Hash;

//New type wrapper so I can implement methods on VAR_U
//TODO decouple from ffi
#[derive(Clone, Debug)]
pub struct VarU(pub ffi::VAR_U);

use crate::{key::Key, value::Value};

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
    name: VarU,
    volset: u_char,
    uci: u_char,
    key: Key,
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

impl hash::Key for VarU {
    fn error() -> Self {
        Self("$ECODE".try_into().expect("the error key is a valid VarU"))
    }
}

pub type Table = hash::HashTable<VarU, VarData>;

mod hash;

use ffi::{PARTAB, UCI_IS_LOCALVAR};

impl Table {
    #[must_use]
    pub fn get(&self, var: &MVAR) -> Option<Value> {
        self.locate(&var.name)?.value(&var.key)
    }

    pub fn set(&mut self, var: &MVAR, value: &Value) -> Result<(), ()> {
        self.create(var.name.clone())
            .map_err(|_| ())?
            .set_value(&var.key, value);
        Ok(())
    }

    pub fn kill(&mut self, var: &MVAR) {
        if let Some(data) = self.locate_mut(&var.name) {
            data.kill(&var.key);
            if !data.contains_data() {
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
        //Keep anything from the passed in slice and all $ vars
        self.remove_if(|x| !(vars.contains(x) || unsafe { x.0.var_cu[0] } == b'$'))
    }
}
#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use super::{VarU, MVAR};
    use crate::{key::Key, value::Value};
    use arbitrary::Arbitrary;
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
    use super::Table;
    use ffi::UCI_IS_LOCALVAR;
    use pretty_assertions::assert_eq;

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

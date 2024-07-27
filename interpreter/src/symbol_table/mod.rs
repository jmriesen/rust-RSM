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

    pub use ffi::VAR_U;
    use ffi::{u_char, u_short};
    use std::{collections::BTreeMap, ffi::c_short, sync::Mutex};

    use crate::{key::Key, value::Value};
    pub static lock: Mutex<()> = Mutex::new(());

    impl std::fmt::Debug for MVAR {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //TODO implement a more complete implementation.
            //Currently this is just enough to start fuzz testing.
            let name = String::from_utf8(self.name.as_array().into()).unwrap();

            let key = String::from_utf8(crate::key::Key::from_raw(&self.key).string_key()).unwrap();
            let mut builder = f.debug_struct("MVar");
            builder.field("name", &name).field("key", &key).finish()
        }
    }

    #[repr(C, packed)]
    #[derive(Copy, Clone)]
    pub struct MVAR {
        pub name: VAR_U,
        pub volset: u_char,
        pub uci: u_char,
        pub slen: u_char,
        pub key: [u_char; 256usize],
    }
    pub const ST_HASH: u32 = 1023;
    pub const ST_FREE: u32 = 1023;
    pub const ST_MAX: u32 = 3072;

    #[derive(Debug)]
    pub struct ST_DATA {
        pub sub_values: BTreeMap<Key, Value>,
        pub attach: ::std::os::raw::c_short,
        pub dbc: u_short,
        pub data: [u_char; 65535usize],
    }

    pub struct table_struct {
        pub st_hash_temp: [i16; 1024],
        pub sym_tab: [SYMTAB; 3073],
    }
    pub type Table = table_struct;
    pub struct SYMTAB {
        pub fwd_link: c_short,
        pub usage: c_short,
        pub data: *mut ST_DATA,
        pub varnam: VAR_U,
    }
    pub type symtab_struct = SYMTAB;
}

pub use c_code::{Table, MVAR};
//TODO remove and replace with derive once type move over to Rust
mod hash;
mod manual;

use std::ptr::null_mut;

use c_code::ST_DATA;
use ffi::{PARTAB, UCI_IS_LOCALVAR, VAR_U, VAR_UNDEFINED};

use crate::{key::Key, value::Value};

type Tab = c_code::symtab_struct;

impl MVAR {
    fn key(&self) -> &[u8] {
        &self.key[..self.slen as usize]
    }
}

impl ST_DATA {
    fn root_value(&self) -> Option<Value> {
        if self.dbc == VAR_UNDEFINED as u16 {
            None
        } else {
            let buff = &self.data[..self.dbc as usize];
            Some(
                Value::try_from(buff)
                    .expect("TODO this should be removed when the ST_Data strcut is redefined"),
            )
        }
    }

    fn set_root(&mut self, data: &Value) {
        let content = data.content();
        self.dbc = content.len() as u16;
        self.data[..content.len()].copy_from_slice(content);
    }

    fn value(&self, key: &[u8]) -> Option<Value> {
        if key.is_empty() {
            self.root_value()
        } else {
            //NOTE There is probably room for optimization here.
            //It is fairly common for the M code to access the values sequentially using $O
            //The C code speed up $O by string a reference to the last used node.
            //I am not doing this right now as it would require making a self referential type
            //and I am not focusing on performance right now. (just correctness)
            //NOTE you could also probably accomplish the last key thing using using a sorted vec
            self.sub_values.get(&Key::from_raw(key)).cloned()
        }
    }

    fn set_value(&mut self, key: &[u8], data: &Value) {
        if key.is_empty() {
            self.set_root(data);
        } else {
            let _ = self.sub_values.insert(Key::from_raw(key), data.clone());
        }
    }

    fn kill(&mut self, key: &[u8]) {
        let key = Key::from_raw(key);
        if key.is_empty() {
            self.sub_values.retain(|_, _| false);
            //Clear values
            self.dbc = VAR_UNDEFINED as u16;
        } else {
            //NOTE Removing a range of keys seems like something the std BTree map should support,
            //However it looks like there is still some design swirl going on, and the design has
            //not been touched in a while
            //https://github.com/rust-lang/rust/issues/81074
            //So I just chose to use the cursor api for now.
            let mut cursor = self
                .sub_values
                .lower_bound_mut(std::ops::Bound::Included(&key));
            while let Some((current_key, _)) = cursor.peek_next()
                && current_key.is_sub_key_of(&key)
            {
                cursor.remove_next();
            }
        }
    }

    //checks self contains any data, if not it can be fried
    //TODO I don't really understand how attached is supposed to work so am skipping mutation testing
    //on this for the moment
    #[cfg_attr(test, mutants::skip)]
    fn contains_data(&self) -> bool {
        !(self.sub_values.is_empty() && self.attach <= 1 && self.dbc == VAR_UNDEFINED as u16)
    }
}

impl Table {
    #[must_use]
    pub fn get(&self, var: &MVAR) -> Option<Value> {
        self.locate(var.name)
            .and_then(|index| unsafe { self[index].data.as_ref() })
            .and_then(|data| data.value(var.key()))
    }

    pub fn set(&mut self, var: &MVAR, value: &Value) -> Result<(), ()> {
        let index = self.create(var.name).map_err(|_| ())?;

        if unsafe { self[index].data.as_mut() }.is_none() {
            self[index].data = Box::into_raw(Box::new(ST_DATA {
                sub_values: Default::default(),
                attach: 1,
                dbc: 0,
                data: [0; 65535],
            }));
        }
        let data =
            unsafe { self[index].data.as_mut() }.expect("If it was none we should have created it");
        data.set_value(var.key(), value);
        Ok(())
    }

    pub fn kill(&mut self, var: &MVAR) {
        use std::ptr::from_mut;
        if let Some(index) = self.locate(var.name) {
            if let Some(data) = unsafe { self[index].data.as_mut() } {
                data.kill(var.key());

                //Drop Data block if no longer used
                if !data.contains_data() {
                    self[index].data = null_mut();
                    let _ = unsafe { Box::from_raw(from_mut(data)) };
                }
            }
            //clean up hash table entry.
            if self[index].data.is_null() && self[index].usage == 0 {
                self.free(var.name);
            }
        }
    }

    //Yes this code is horribly inefficient.
    //However eventually this will be replaced by a std collection method so it is not really worth
    //optimizing right now.
    //NOTE not yet mutation tested
    fn keep(&mut self, vars: &[VAR_U], tab: &mut PARTAB) {
        //NOTE I am not sure how src_var is used, but this was done in the C code.
        tab.src_var = ffi::MVAR {
            uci: UCI_IS_LOCALVAR as u8,
            slen: 0,
            volset: 0,
            ..tab.src_var
        };

        let to_kill: Vec<_> = self
            .sym_tab
            .iter_mut()
            .filter(|x| !x.data.is_null())
            .map(|x| x.varnam)
            //always keep $ vars
            .filter(|x| unsafe { x.var_cu[0] } != b'$')
            .filter(|x| unsafe { x.var_cu[0] } != b'\0')
            .filter(|x| !vars.contains(x))
            .collect();
        for var in to_kill {
            self.kill(&MVAR {
                name: var,
                volset: 0,
                uci: 0,
                slen: 0,
                key: [0; 256],
            });
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.keep(&[], &mut Default::default());
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use arbitrary::Arbitrary;

    use crate::{key::Key, value::Value};

    use super::c_code::{MVAR, VAR_U};
    #[must_use]
    pub fn var_u(var: &str) -> VAR_U {
        var.try_into().unwrap()
    }

    #[must_use]
    pub fn var_m(name: &str, values: &[&str]) -> MVAR {
        let values = values
            .into_iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = Key::new(&values).unwrap();

        let mut key_buff = [0; 256];
        let len = key.len();
        key_buff[..key.len()].copy_from_slice(key.raw_keys());

        MVAR {
            name: var_u(name),
            volset: Default::default(),
            uci: Default::default(),
            slen: len as u8,
            key: key_buff,
        }
    }

    impl<'a> Arbitrary<'a> for MVAR {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let name: [u8; 32] = u.arbitrary()?;
            if name.is_ascii() && name.contains(&0) {
                Ok(MVAR {
                    name: VAR_U { var_cu: name },
                    volset: 0,
                    uci: 0,
                    slen: 0,
                    key: [0; 256],
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
        assert_eq!(table.locate(var_u("foo")), None);
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

        table.keep(&[retain_a.name, retain_b.name], &mut part_tab);
        assert_eq!(part_tab.src_var.uci, UCI_IS_LOCALVAR as u8);
        assert_eq!(part_tab.src_var.slen, 0);
        assert_eq!(part_tab.src_var.volset, 0);

        assert_eq!(table.get(&normal), None);
        for var in &[dolor, retain_a, retain_b] {
            assert_eq!(table.get(var), Some(data.clone()));
        }
    }
}

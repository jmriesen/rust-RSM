//TODO remove once this module is actually being used.
#![allow(dead_code)]
use std::{array::from_fn, mem::transmute, ptr::null_mut};

#[allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
mod c_code {
    pub use ffi::CSTRING;
    pub use ffi::VAR_U;
    use std::sync::Mutex;
    pub static lock: Mutex<()> = Mutex::new(());
    include!(concat!(env!("OUT_DIR"), "/symbol_table_c.rs"));
}

//TODO remove and replace with derive once type move over to Rust
mod hash;
mod manual;

use c_code::{Table, MVAR, ST_DATA, ST_DEPEND, VAR_UNDEFINED};

use crate::key::CArrayString;
type Tab = c_code::symtab_struct;

impl MVAR {
    fn key(&self) -> &[u8] {
        &self.key[..self.slen as usize]
    }
}

//Both the Key and the value are stored in the bytes array.
//Also the C code uses this a dynamically sized type (rust thinks this is a sized type).
impl ST_DEPEND {
    fn key(&self) -> &[u8] {
        &self.bytes[..self.keylen as usize]
    }

    fn value(&self) -> &[u8] {
        let len_start = self.keylen.next_multiple_of(2) as usize;
        let len_end = len_start + size_of::<u16>();
        let len = u16::from_ne_bytes(self.bytes[len_start..len_end].try_into().unwrap());
        &self.bytes[len_end..len_end + len as usize]
    }
}

struct DependIter<'a> {
    current: Option<&'a ST_DEPEND>,
}

impl<'a> Iterator for DependIter<'a> {
    type Item = &'a ST_DEPEND;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current;
        if let Some(current) = self.current {
            self.current = unsafe { current.deplnk.as_ref() };
        }
        temp
    }
}

impl<'a> DependIter<'a> {
    fn new(data: &'a ST_DATA) -> Self {
        Self {
            current: unsafe { data.deplnk.as_ref() },
        }
    }
    //Find the dependent block that contains the key.
    fn find_key(&mut self, key: &[u8]) -> Option<&'a ST_DEPEND> {
        //Keys are stored in sorted order so don't need to check all of them
        self.skip_while(|x| x.key() < key)
            .next()
            .filter(|x| x.key() == key)
    }
}

impl ST_DATA {
    fn root_value(&self) -> Option<&[u8]> {
        if self.dbc == VAR_UNDEFINED as u16 {
            None
        } else {
            Some(&self.data[..self.dbc as usize])
        }
    }

    fn value(&self, key: &[u8]) -> Option<CArrayString> {
        if key == &[] {
            self.root_value()
        } else {
            //TODO This can be optimized using using the last key value
            //I am not doing that optimization at the moment since I want this to be a &self function
            //I may try and add it with inner mutability in the future
            DependIter::new(self).find_key(key).map(|node| node.value())
        }
        .map(|x| x.try_into().unwrap())
    }
}

impl Table {
    fn get(&mut self, var: &MVAR) -> Option<CArrayString> {
        self.locate(var.name)
            .map(|index| unsafe { self[index].data.as_ref() })
            .flatten()
            .map(|data| data.value(var.key()))
            .flatten()
    }
}

#[cfg(test)]
pub mod tests {

    use pretty_assertions::assert_eq;

    use crate::key::{CArrayString, KeyList};

    use super::c_code::{TMP_Set, Table, MVAR, VAR_U};
    use std::{i32, ptr::from_mut};
    pub fn var_u(var: &str) -> VAR_U {
        var.try_into().unwrap()
    }

    //TODO clean this up at some point
    pub fn var_m(name: &str, keys: &[&str]) -> MVAR {
        let mut key_buff = KeyList::new();
        for key in keys {
            key_buff.push(&((*key).into())).unwrap();
        }

        let mut key = [0; 256];
        let keys = key_buff.into_raw();
        key[..keys.len()].copy_from_slice(&keys[..]);

        MVAR {
            name: var_u(name),
            volset: Default::default(),
            uci: Default::default(),
            slen: keys.len() as u8,
            key,
        }
    }

    ///Temporary shim layer for calling the C code.
    impl Table {
        fn c_set(&mut self, var: &mut MVAR, data: &mut CArrayString) {
            unsafe { TMP_Set(from_mut(var), from_mut(data.as_mut()), from_mut(self)) };
        }
        fn c_get(&mut self, var: &mut MVAR) -> Result<CArrayString, i32> {
            self.get(var).ok_or_else(|| -6)
        }
    }

    #[test]
    fn get_unset_variable() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        assert_eq!(Err(-6), table.c_get(&mut m_var));
    }
    #[test]
    fn get_unset_key() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut data: CArrayString = "Data".into();
        table.c_set(&mut m_var, &mut data);

        let mut m_var = var_m("foo", &["bar"]);
        assert_eq!(Err(-6), table.c_get(&mut m_var));
    }

    #[test]
    fn set_root_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut data: CArrayString = "Data".into();

        table.c_set(&mut m_var, &mut data);
        assert_eq!(Ok(data), table.c_get(&mut m_var));
    }

    #[test]
    fn set_index_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &["keys"]);
        let mut data: CArrayString = "Data".into();
        table.c_set(&mut m_var, &mut data);
        assert_eq!(Ok(data), table.c_get(&mut m_var));
    }

    #[test]
    fn set_root_then_index() {
        let mut root = var_m("foo", &[]);
        let mut root_data: CArrayString = "root Data".into();
        let mut with_key = var_m("foo", &["keys"]);
        let mut key_data: CArrayString = "key Data".into();
        {
            let mut table = Table::new();

            table.c_set(&mut root, &mut root_data);
            table.c_set(&mut with_key, &mut key_data);
            assert_eq!(Ok(root_data.clone()), table.c_get(&mut root));
            assert_eq!(Ok(key_data.clone()), table.c_get(&mut with_key));
        }
        {
            let mut table = Table::new();

            table.c_set(&mut with_key, &mut key_data);
            table.c_set(&mut root, &mut root_data);
            assert_eq!(Ok(root_data.clone()), table.c_get(&mut root));
            assert_eq!(Ok(key_data.clone()), table.c_get(&mut with_key));
        }
    }

    #[test]
    fn set_null_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut data: CArrayString = "".into();

        table.c_set(&mut m_var, &mut data);
        assert_eq!(Ok(data), table.c_get(&mut m_var));
    }

    #[test]
    fn set_overrides_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        let mut initial_value: CArrayString = "inital".into();
        let mut end_value: CArrayString = "end".into();

        table.c_set(&mut m_var, &mut initial_value);
        assert_eq!(Ok(initial_value), table.c_get(&mut m_var));

        table.c_set(&mut m_var, &mut end_value);
        assert_eq!(Ok(end_value), table.c_get(&mut m_var));
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
        let mut test_data =
            test_data.map(|(keys, value)| (var_m("foo", &keys), Box::new(value.into())));

        let mut table = Table::new();
        for (var, value) in test_data.iter_mut() {
            table.c_set(var, value);
        }

        for (mut var, value) in test_data.into_iter() {
            assert_eq!(Ok(*value), table.c_get(&mut var));
        }
    }
}

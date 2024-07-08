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

impl Table {
    //TODO consider removing allocations.
    fn get(&mut self, var: &MVAR) -> Result<CArrayString, i32> {
        if let Some(index) = self.locate(var.name) {
            let data = unsafe { self[index].data.as_ref() };
            if let Some(data) = data {
                if var.slen == 0 {
                    let data = unsafe { *self[index].data };
                    if data.dbc == VAR_UNDEFINED as u16 {
                        Err(-6)
                    } else {
                        Ok(CArrayString::new(ffi::CSTRING {
                            len: data.dbc,
                            buf: data.data,
                        }))
                    }
                } else {
                    //TODO This can be optimized using using the last key value
                    //TODO this can be optomized since we know the keys are in sorted order.
                    KeyIter::new(data)
                        .find(|x| var.key[..var.slen as usize] == x.bytes[..x.keylen as usize])
                        .map(|data| {
                            let data_len_start = data.keylen.next_multiple_of(2) as usize;
                            let data_len_end = data_len_start + size_of::<u16>();
                            let len = u16::from_ne_bytes(
                                data.bytes[data_len_start..data_len_end].try_into().unwrap(),
                            );
                            let mut buf = [0; 65535];
                            buf[..]
                                .clone_from_slice(&data.bytes[data_len_end..data_len_end + 65535]);
                            CArrayString::new(ffi::CSTRING { len, buf })
                        })
                        .ok_or_else(|| -6)
                }
            } else {
                Err(-6)
            }
        } else {
            Err(-6)
        }
    }
}

struct KeyIter<'a> {
    current: Option<&'a ST_DEPEND>,
}

impl<'a> Iterator for KeyIter<'a> {
    type Item = &'a ST_DEPEND;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current;
        if let Some(current) = self.current {
            self.current = unsafe { current.deplnk.as_ref() };
        }
        temp
    }
}
impl<'a> KeyIter<'a> {
    fn new(data: &'a ST_DATA) -> Self {
        Self {
            current: unsafe { data.deplnk.as_ref() },
        }
    }
}

#[cfg(test)]
pub mod tests {

    use ffi::CSTRING;
    use pretty_assertions::assert_eq;

    use crate::{
        key::{CArrayString, KeyList},
        shared_seg::lock_tab::tests::assert_eq,
        symbol_table::c_code,
    };

    use super::c_code::{TMP_Get, TMP_Set, Table, MVAR, VAR_U};
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
            name: var_u("foo"),
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
            /*
                        let mut c_string = CSTRING {
                            len: 0,
                            buf: [0; 65535],
                        };
                        let len = unsafe { TMP_Get(from_mut(var), c_string.buf.as_mut_ptr(), from_mut(self)) };
                        if len >= 0 {
                            c_string.len = len as u16;
                            Ok(CArrayString::new(c_string))
                        } else {
                            Err(len)
                        }
            */
            self.get(var)
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

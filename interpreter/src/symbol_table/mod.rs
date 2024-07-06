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

use c_code::Table;
type Tab = c_code::symtab_struct;

#[cfg(test)]
pub mod tests {

    use ffi::CSTRING;

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
        }
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
    fn get_unset_variable() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &[]);
        assert_eq!(Err(-6), table.c_get(&mut m_var));
    }

    #[test]
    fn set_index_value() {
        let mut table = Table::new();
        let mut m_var = var_m("foo", &["keys"]);
        let mut data: CArrayString = "Data".into();
        table.c_set(&mut m_var, &mut data);
        assert_eq!(Ok(data), table.c_get(&mut m_var));
    }
}

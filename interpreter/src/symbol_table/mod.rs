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

    use crate::{
        key::{CArrayString, KeyList},
        symbol_table::c_code,
    };

    use super::c_code::{TMP_Get, TMP_Set, Table, MVAR, VAR_U};
    use std::ptr::from_mut;
    pub fn var_u(var: &str) -> VAR_U {
        var.try_into().unwrap()
    }

    #[test]
    fn set_root_value() {
        let mut table = Table::new();
        let mut m_var = MVAR {
            name: var_u("foo"),
            volset: Default::default(),
            uci: Default::default(),
            slen: 0,
            key: [0; 256],
        };
        let mut data: CArrayString = "Data".into();
        unsafe {
            TMP_Set(
                from_mut(&mut m_var),
                from_mut(&mut data.as_mut()),
                from_mut(&mut table),
            )
        };

        let mut buff = [0; 300];
        let len = unsafe {
            TMP_Get(
                from_mut(&mut m_var),
                buff.as_mut_ptr(),
                from_mut(&mut table),
            )
        };
        assert_eq!(data.content(), &buff[..len as usize]);
    }

    #[test]
    fn get_unset_variable() {
        let mut table = Table::new();
        let mut m_var = MVAR {
            name: var_u("foo"),
            volset: Default::default(),
            uci: Default::default(),
            slen: 0,
            key: [0; 256],
        };
        let mut buff = [0; 300];
        let len = unsafe {
            TMP_Get(
                from_mut(&mut m_var),
                buff.as_mut_ptr(),
                from_mut(&mut table),
            )
        };
        assert_eq!(len, -6);
    }

    #[test]
    fn set_index_value() {
        let mut table = Table::new();

        let mut keys = KeyList::new();
        keys.push(&("key".into()));
        let mut key = [0; 256];
        let keys = keys.into_raw();
        key[..keys.len()].copy_from_slice(&keys[..]);

        let mut m_var = MVAR {
            name: var_u("foo"),
            volset: Default::default(),
            uci: Default::default(),
            slen: keys.len() as u8,
            key,
        };
        let mut data: CArrayString = "Data".into();

        unsafe {
            TMP_Set(
                from_mut(&mut m_var),
                from_mut(&mut data.as_mut()),
                from_mut(&mut table),
            )
        };

        let mut buff = [0; 300];
        let len = unsafe {
            TMP_Get(
                from_mut(&mut m_var),
                buff.as_mut_ptr(),
                from_mut(&mut table),
            )
        };

        assert_eq!(data.content(), &buff[..len as usize]);
    }
}

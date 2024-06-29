use std::{array::from_fn, fmt::Debug, mem::transmute, ptr::null_mut};

use derive_more::{AsMut, AsRef};

#[allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
mod c_code {
    use ffi::CSTRING;
    use ffi::VAR_U;
    use std::sync::Mutex;
    pub static lock: Mutex<()> = Mutex::new(());
    include!(concat!(env!("OUT_DIR"), "/symbol_table_c.rs"));
}
use c_code::{symtab_struct, ST_HASH, ST_MAX};
use ffi::VAR_U;
use ref_cast::RefCast;

const TAB_RAW_SIZE: usize = ST_MAX as usize + 1;
const HASH_RAW_SIZE: usize = ST_HASH as usize + 1;

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    hash: [i16; HASH_RAW_SIZE],
    tabs: [Tab; TAB_RAW_SIZE],
}

//It looks like 3.c files touch symtab.
//So until all three of therm are converted I will need to match the C structure.
//But that still leaves the question do I match my internals, or just map the structures.
//This is not in the shared memory segment so I am inclined to do my own data structure using rust std
//I also think it might be a bit more true to normal development since rarely do you want to rewrite
//your entire code base.

impl Table {
    pub fn new() -> Self {
        let mut hash = [-1; HASH_RAW_SIZE];
        *hash.last_mut().unwrap() = 0;
        let var_name: VAR_U = "".try_into().expect("string literals should not fail");

        let mut tabs = from_fn(|i| {
            Tab(symtab_struct {
                fwd_link: 1 + i as i16,
                usage: 0,
                data: null_mut(),
                varnam: var_name.clone(),
            })
        });
        // -1 == end of the list;
        tabs.last_mut().unwrap().0.fwd_link = -1;

        Self { hash, tabs }
    }

    //Question how isolated are these from the rest of C code
    fn from_c() -> Self {
        use ffi::{st_hash, symtab};
        let mut hash = [0; HASH_RAW_SIZE];
        hash.copy_from_slice(unsafe {
            std::slice::from_raw_parts(st_hash.as_ptr(), HASH_RAW_SIZE)
        });
        let tabs: Vec<_> = unsafe { std::slice::from_raw_parts(symtab.as_ptr(), TAB_RAW_SIZE) }
            .iter()
            .map(|x| {
                Tab(symtab_struct {
                    fwd_link: x.fwd_link,
                    usage: x.usage,
                    //TODO Yes this is pointer copying.
                    //Fix this at some point
                    data: unsafe { transmute(x.data) },
                    varnam: x.varnam,
                })
            })
            .collect();

        Self {
            hash,
            tabs: tabs.try_into().unwrap(),
        }
    }
}

#[derive(RefCast, AsMut, AsRef)]
#[repr(transparent)]
pub struct Tab(symtab_struct);

impl Eq for Tab {}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.0.fwd_link == other.0.fwd_link
        && self.0.usage == other.0.usage
        //Note data is a pointer 
        //We well need to switch to deep copies at some point.
        && self.0.data == other.0.data
        && self.0.varnam == other.0.varnam
    }
}

impl Debug for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("forward_link", &{ self.0.fwd_link })
            .field("usage", &{ self.0.usage })
            .field("data", &{ self.0.data })
            .field("variable name", &self.0.varnam)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        array::from_fn,
        ptr::{from_mut, null_mut},
    };

    use pretty_assertions::assert_eq;

    use crate::{
        key::CArrayString,
        symbol_table::c_code::{lock, ST_Init, SYMTAB},
    };

    use super::{
        c_code::{ST_Set, MVAR},
        Table,
    };

    #[test]
    fn init() {
        let _guard = lock.lock().unwrap();
        /*
                let mut table = super::c_code::Table {
                    st_hash_temp: [0; 1024],
                    sym_tab: from_fn(|_| SYMTAB {
                        fwd_link: 0,
                        usage: 0,
                        data: null_mut(),
                        varnam: "".try_into().unwrap(),
                    }),
                };
        */
        unsafe { ffi::ST_Init() }
        let c = Table::from_c();
        assert_eq!(c, Table::new());
    }

    /*
    * set and get actually involve a lot so I am not going to mess with them right away.
    #[test]
    fn set_get() {
        let _guard = lock.lock().unwrap();
        let data = CArrayString::from("Data");
        let mut var = MVAR {
            name: "varname".try_into().unwrap(),
            volset: 0,
            uci: 0,
            slen: 0,
            key: [0; 256],
        };
        unsafe { ST_Init() };
        unsafe { ST_Set(from_mut(&mut var), from_mut((&mut data.clone()).as_mut())) };
        let c = Table::from_c();
        assert_eq!(c, Table::new());
    }
    */
}

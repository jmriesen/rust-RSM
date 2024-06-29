use std::{array::from_fn, fmt::Debug, mem::transmute, ptr::null_mut};

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

//TODO remove and replace with derive once type move over to Rust
mod manual;
use c_code::{ST_HASH, ST_MAX};
use ffi::VAR_U;

const TAB_RAW_SIZE: usize = ST_MAX as usize + 1;
const HASH_RAW_SIZE: usize = ST_HASH as usize + 1;

use c_code::Table;
type Tab = c_code::symtab_struct;

impl Table {
    pub fn new() -> Self {
        let mut hash = [-1; HASH_RAW_SIZE];
        *hash.last_mut().unwrap() = 0;
        let var_name: VAR_U = "".try_into().expect("string literals should not fail");

        let mut tabs = from_fn(|i| Tab {
            fwd_link: 1 + i as i16,
            usage: 0,
            data: null_mut(),
            varnam: var_name.clone(),
        });
        // -1 == end of the list;
        tabs.last_mut().unwrap().fwd_link = -1;

        Self {
            st_hash_temp: hash,
            sym_tab: tabs,
        }
    }

    //Question how isolated are these from the rest of C code
    #[cfg(test)]
    fn clone_c_table() -> Self {
        use ffi::{st_hash, symtab};
        let mut hash = [0; HASH_RAW_SIZE];
        hash.copy_from_slice(unsafe {
            std::slice::from_raw_parts(st_hash.as_ptr(), HASH_RAW_SIZE)
        });
        let tabs: Vec<_> = unsafe { std::slice::from_raw_parts(symtab.as_ptr(), TAB_RAW_SIZE) }
            .iter()
            .map(|x| {
                Tab {
                    fwd_link: x.fwd_link,
                    usage: x.usage,
                    //TODO Yes this is pointer copying.
                    //Fix this at some point
                    data: unsafe { transmute(x.data) },
                    varnam: x.varnam,
                }
            })
            .collect();

        Self {
            st_hash_temp: hash,
            sym_tab: tabs.try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::symbol_table::c_code::lock;

    use super::Table;
    use rstest::*;

    #[test]
    fn init() {
        use pretty_assertions::assert_eq;
        let _guard = lock.lock().unwrap();
        //This may not be referring to the right table
        unsafe { ffi::ST_Init() }
        let c = Table::clone_c_table();
        assert_eq!(c, Table::new());
    }

    #[rstest]
    #[case("", 0)]
    #[case("Some string", 704)]
    #[case("Another string", 35)]
    #[case("      ", 769)]
    #[case("aaaa", 476)]
    fn hash(#[case] input: &str, #[case] expected: i16) {
        let var = dbg!(input).try_into().unwrap();
        assert_eq!(unsafe { super::c_code::TMP_Hash(var) }, expected)
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

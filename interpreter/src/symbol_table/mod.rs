use std::{array::from_fn, fmt::Debug, mem::transmute, ptr::null_mut};

use derive_more::{AsMut, AsRef};

mod c_code {
    include!(concat!(env!("OUT_DIR"), "/symbol_table_c.rs"));
}
use c_code::{st_hash_temp, sym_tab, symtab_struct, ST_FREE, ST_HASH, ST_MAX};
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
        let mut tabs = from_fn(|i| {
            Tab(symtab_struct {
                fwd_link: 1 + i as i16,
                usage: 0,
                data: null_mut(),
                varnam: convert_var_u_back("".try_into().expect("string literals should not fail")),
            })
        });
        // -1 == end of the list;
        tabs.last_mut().unwrap().0.fwd_link = -1;

        Self { hash, tabs }
    }

    //Question how isolated are these from the rest of C code
    fn from_c() -> Self {
        let mut hash = [0; HASH_RAW_SIZE];
        hash.copy_from_slice(unsafe {
            std::slice::from_raw_parts(st_hash_temp.as_ptr(), HASH_RAW_SIZE)
        });
        let tabs: Vec<_> = unsafe { std::slice::from_raw_parts(sym_tab.as_ptr(), TAB_RAW_SIZE) }
            .iter()
            .map(|x| {
                Tab(symtab_struct {
                    fwd_link: x.fwd_link,
                    usage: x.usage,
                    //TODO Yes this is pointer copying.
                    //Fix this at some point
                    data: x.data,
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

fn convert_var_u(input: c_code::VAR_U) -> ffi::VAR_U {
    //TODO remove this
    //This is safe since the header files are 2 copies of the same file.
    //Once the system table files are fixed this should be removed
    unsafe { transmute::<_, _>(input) }
}

fn convert_var_u_back(input: ffi::VAR_U) -> c_code::VAR_U {
    //TODO remove this
    //This is safe since the header files are 2 copies of the same file.
    //Once the system table files are fixed this should be removed
    unsafe { transmute::<_, _>(input) }
}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.0.fwd_link == other.0.fwd_link
        && self.0.usage == other.0.usage
        //Note data is a pointer 
        //We well need to switch to deep copies at some point.
        && self.0.data == other.0.data
        && convert_var_u(self.0.varnam) == convert_var_u(other.0.varnam)
    }
}

impl Debug for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("forward_link", &{ self.0.fwd_link })
            .field("usage", &{ self.0.usage })
            .field("data", &{ self.0.data })
            .field("variable name", &convert_var_u(self.0.varnam))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use ffi::ST_Init;
    use pretty_assertions::assert_eq;

    use super::Table;

    #[test]
    fn init() {
        unsafe { ST_Init() }
        let c = Table::from_c();
        assert_eq!(c, Table::new());
    }
}

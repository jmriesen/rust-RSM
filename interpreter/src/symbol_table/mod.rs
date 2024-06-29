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

#[no_mangle]
pub extern "C" fn rust_hash(var: c_code::var_u) -> i16 {
    hash(var)
}

fn hash(var: VAR_U) -> i16 {
    let primes = [
        3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137,
    ];
    (var.as_array()
        .into_iter()
        .cloned()
        .take_while(|x| *x != 0)
        .enumerate()
        //Note using i32 to mimic C's int
        .map(|(i, x)| x as i32 * primes[i])
        .sum::<i32>()
        //matching casting behavior of C
        % ST_HASH as i32) as i16
}

#[cfg(test)]
mod tests {

    use std::ptr::from_mut;

    use crate::{
        shared_seg::lock_tab::tests::assert_eq,
        symbol_table::c_code::{lock, TMP_Locate, MVAR},
    };
    use arbitrary::Arbitrary;
    use ffi::ST_MAX;
    use pretty_assertions::assert_eq;
    use rand::{distributions::Alphanumeric, Rng};

    use super::{
        c_code::{TMP_Create, TMP_Free},
        Table,
    };
    use rstest::*;

    #[test]
    fn init() {
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
        assert_eq!(super::hash(var), expected)
    }

    #[test]
    fn create() {
        let mut table = Table::new();
        let table = from_mut(&mut table);
        for i in 0..ST_MAX as i16 {
            let var = format!("var{i}").try_into().unwrap();
            let index = unsafe { TMP_Create(var, table) };
            //NOTE having sequential indexes probably improves cash locality
            assert_eq!(index, i);
            assert_eq!(unsafe { TMP_Locate(var, table) }, i);
        }

        let last_straw = format!("lastStraw").try_into().unwrap();
        let index = unsafe { TMP_Create(last_straw, table) };
        assert_eq!(index, -256);
    }
    #[test]
    fn create_duplicate_hash() {
        use ffi::VAR_U;
        let mut table = Table::new();
        let table = from_mut(&mut table);
        let vars = ["TMNriCuk1j", "kYyWF1E499", "ZdTKA4eNgW"];
        let vars: [VAR_U; 3] = vars.map(|x| x.try_into().unwrap());
        for var in vars {
            //These should all hash to the same value
            assert_eq!(super::hash(var), 10);
            unsafe { TMP_Create(var, table) };
        }
        //Verify that links still work properly after deletion
        unsafe { TMP_Free(vars[1], table) };
        assert!(unsafe { TMP_Locate(vars[2], table) } >= 0);

        /*
                let mut count = 0;
                loop {
                    let var_str: String = rand::thread_rng()
                        .sample_iter(Alphanumeric)
                        .take(10)
                        .map(char::from)
                        .collect();
                    let var = var_str.as_str().try_into().unwrap();
                    if super::hash(var) == 10 {
                        dbg!(var_str);
                        count += 1;
                        if count == 3 {
                            break;
                        }
                    }
                }
                panic!();
        */
    }

    #[test]
    fn create_kill_create() {
        let mut table = Table::new();
        let table = from_mut(&mut table);
        let var0 = format!("var0").try_into().unwrap();
        let var1 = format!("var1").try_into().unwrap();
        let var2 = format!("var2").try_into().unwrap();
        let var3 = format!("var3").try_into().unwrap();

        let index0 = unsafe { TMP_Create(var0, table) };
        let index1 = unsafe { TMP_Create(var1, table) };
        let index2 = unsafe { TMP_Create(var2, table) };
        let index3 = unsafe { TMP_Create(var3, table) };

        let var1_1 = format!("var1.1").try_into().unwrap();
        let var2_1 = format!("var2.1").try_into().unwrap();

        //killing a var opens up its slot again.
        //It looks like the kill/create operate like the pop/push of a stack.
        //FILO
        unsafe { TMP_Free(var1, table) };
        unsafe { TMP_Free(var2, table) };

        assert_eq!(unsafe { TMP_Create(var2_1, table) }, index2);
        assert_eq!(unsafe { TMP_Create(var1_1, table) }, index1);
    }

    #[test]
    fn create_duplicates() {
        let mut table = Table::new();
        let var = "varname".try_into().unwrap();

        let index = unsafe { TMP_Create(var, from_mut(&mut table)) };
        let index_2 = unsafe { TMP_Create(var, from_mut(&mut table)) };
        assert_eq!(index, index_2);
    }
    #[test]
    fn locate_nonexistent_var() {
        let mut table = Table::new();
        assert_eq!(
            unsafe { TMP_Locate("foo".try_into().unwrap(), from_mut(&mut table)) },
            -1
        )
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

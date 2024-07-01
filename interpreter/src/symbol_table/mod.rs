use std::{array::from_fn, mem::transmute, ptr::null_mut};

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
use c_code::{ST_FREE, ST_HASH, ST_MAX, SYMTAB};
use ffi::VAR_U;

const TAB_RAW_SIZE: usize = ST_MAX as usize + 1;
const HASH_RAW_SIZE: usize = ST_HASH as usize + 1;

use c_code::Table;
type Tab = c_code::symtab_struct;

///Some API calls give out the internal index where data has been stored
///This type represents a index that has come from a table and is therefore valid.
///I would rather just return references to the data, but that
///is not how the c code is structured so this will likely remain until
///everything is in rust.
///TODO remove type and just return references to the data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Index(i16);

impl Index {
    fn raw(raw: i16) -> Option<Self> {
        if raw == -1 {
            None
        } else {
            Some(Index(raw))
        }
    }
    fn to_raw(internal: Option<Self>) -> i16 {
        if let Some(val) = internal {
            val.0
        } else {
            -1
        }
    }
}

impl std::ops::Index<Index> for Table {
    type Output = SYMTAB;

    fn index(&self, index: Index) -> &Self::Output {
        &self.sym_tab[index.0 as usize]
    }
}

impl std::ops::IndexMut<Index> for Table {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.sym_tab[index.0 as usize]
    }
}

/// The only error condition is if we run out of room in the table.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CreationError;

impl CreationError {
    fn error_code(&self) -> i16 {
        -256
    }
}

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

    //The tables have a max capacity and will fail if to many variables are added.
    fn create(&mut self, var: VAR_U) -> Result<Index, CreationError> {
        if let Some(index) = self.locate(var) {
            Ok(index)
        } else {
            let index = self.st_hash_temp[ST_FREE as usize];
            if index == -1 || (index == ST_MAX as i16 && unsafe { var.var_q != 76159689901348 }) {
                return Err(CreationError);
            } else {
                let index = Index::raw(index).expect("null check has alrady happend");
                let hash = hash(var);
                self.st_hash_temp[ST_FREE as usize] = self[index].fwd_link;
                self[index].fwd_link = self.st_hash_temp[hash as usize];
                self.st_hash_temp[hash as usize] = Index::to_raw(Some(index));

                self[index] = SYMTAB {
                    usage: 0,
                    varnam: var,
                    data: null_mut(),
                    ..self[index]
                };
                Ok(index)
            }
        }
    }

    fn locate(&self, var: VAR_U) -> Option<Index> {
        self.locate_helper(var).map(|(_, x)| x)
    }

    fn locate_helper(&self, var: VAR_U) -> Option<(Option<Index>, Index)> {
        LineIterator {
            table: self,
            next: Index::raw(self.st_hash_temp[hash(var) as usize]),
            previous: None,
        }
        .find(|(_, i)| self[*i].varnam == var)
    }

    fn free(&mut self, var: VAR_U) {
        match self.locate_helper(var) {
            None => { /*Value never found do nothing */ }
            Some((previous, current)) => {
                //Update links
                if let Some(previous) = previous {
                    self[previous].fwd_link = self[current].fwd_link;
                } else {
                    self.st_hash_temp[hash(var) as usize] = self[current].fwd_link
                }
                //I don't know if I like ST_FREE being used as a special index.
                self[current].fwd_link = self.st_hash_temp[ST_FREE as usize];
                self.st_hash_temp[ST_FREE as usize] = Index::to_raw(Some(current));

                //Clear old data.
                self[current] = SYMTAB {
                    data: null_mut(),
                    varnam: "".try_into().unwrap(),
                    ..self[current]
                };
            }
        }
    }
}

#[derive(Clone, Copy)]
struct LineIterator<'a> {
    table: &'a Table,
    next: Option<Index>,
    previous: Option<Index>,
}

//Iterates over the nodes fwd_links.
//Returns (next-1,next) since having the next-1 node make un-linking easier.
impl Iterator for LineIterator<'_> {
    type Item = (Option<Index>, Index);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next {
            let val = (self.previous, cur);
            self.previous = self.next;
            self.next = Index::raw(self.table[cur].fwd_link);
            Some(val)
        } else {
            None
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

    use std::ptr::{from_mut, null_mut};

    use pretty_assertions::assert_eq;
    use rand::{distributions::Alphanumeric, Rng};

    use super::{
        c_code::{lock, TMP_Create},
        CreationError, Index, Table, ST_MAX,
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
        for i in 0..ST_MAX as i16 {
            let var = format!("var{i}").try_into().unwrap();
            let index = table.create(var);
            //NOTE having sequential indexes probably improves cash locality
            assert_eq!(index, Ok(Index(i)));
            assert_eq!(table.locate(var), Some(Index(i)));

            //Verify data has been reset
            //TODO create a better test for this.
            //usage and data are both zeroed during initialization.
            let node = &table[index.expect("Someness allready checked")];
            assert_eq!(node.varnam, var);
            assert_eq!({ node.usage }, 0);
            assert_eq!({ node.data }, null_mut());
        }

        let last_straw = format!("lastStraw").try_into().unwrap();
        let index = table.create(last_straw);
        assert_eq!(index, Err(CreationError));

        //There is a special node reserved for ECODE in the case that everything else has
        //been filed.
        let index = table.create("$ECODE".try_into().unwrap());
        assert_eq!(index, Ok(Index(ST_MAX as i16)));
    }

    #[test]
    fn error_code() {
        let err = CreationError;
        assert_eq!(err.error_code(), -256);
    }

    #[test]
    fn create_duplicate_hash() {
        use ffi::VAR_U;
        let mut table = Table::new();
        let table_ptr = from_mut(&mut table);
        let vars = ["TMNriCuk1j", "kYyWF1E499", "ZdTKA4eNgW"];
        let vars: [VAR_U; 3] = vars.map(|x| x.try_into().unwrap());
        for var in vars {
            //These should all hash to the same value
            assert_eq!(super::hash(var), 10);
            let _ = table.create(var);
        }
        //Verify we can still access the remaining values
        table.free(vars[1]);
        assert_ne!(table.locate(vars[0]), None);
        assert_ne!(table.locate(vars[2]), None);
    }

    //helper function used to find the hash conflicts that are used in the tests
    //
    //This tries a bunch of random strings and checks if they hash to the provided value.
    #[allow(dead_code)]
    #[cfg(test)]
    fn find_hash_coalitions(hash: i16) -> impl std::iter::Iterator<Item = String> {
        std::iter::repeat_with(|| {
            rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        })
        .filter(move |var| {
            let var = var.as_str().try_into().unwrap();
            super::hash(var) == hash
        })
    }

    #[test]
    fn create_free_create() {
        let mut table = Table::new();
        let var0 = format!("var0").try_into().unwrap();
        let var1 = format!("var1").try_into().unwrap();
        let var2 = format!("var2").try_into().unwrap();
        let var3 = format!("var3").try_into().unwrap();

        let _index0 = table.create(var0).unwrap();
        let index1 = table.create(var1).unwrap();
        let index2 = table.create(var2).unwrap();
        let _index3 = table.create(var3).unwrap();

        let var1_1 = format!("var1.1").try_into().unwrap();
        let var2_1 = format!("var2.1").try_into().unwrap();

        //notes are reused in a FILO manor
        table.free(var1);
        table.free(var2);
        assert_eq!(table.create(var2_1), Ok(index2));
        assert_eq!(table.create(var1_1), Ok(index1));
    }

    #[test]
    fn create_duplicates() {
        let mut table = Table::new();
        let var = "varname".try_into().unwrap();

        let first = table.create(var);
        let second = table.create(var);
        assert_eq!(first, second);
    }

    #[test]
    fn locate_nonexistent_var() {
        let table = Table::new();
        assert_eq!(table.locate("foo".try_into().unwrap()), None);
    }

    #[test]
    fn free_resets_to_default() {
        let mut table = Table::new();
        let var = "varname".try_into().unwrap();
        let index = table.create(var).unwrap();
        table.free(var);

        let node = &table[index];
        assert_eq!(node.varnam, "".try_into().unwrap());
        assert_eq!({ node.data }, null_mut());
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

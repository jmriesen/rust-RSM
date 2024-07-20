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
    pub use ffi::{CSTRING, VAR_U};
    use std::sync::Mutex;
    pub static lock: Mutex<()> = Mutex::new(());
    include!(concat!(env!("OUT_DIR"), "/symbol_table_c.rs"));
}

//TODO remove and replace with derive once type move over to Rust
mod hash;
mod manual;

use std::ptr::null_mut;

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
        //data is stored as [(key:array),(possible padding)(value:CSTRING),(extra space)]
        //TODO move a way from this data layout
        //I don't like that we are relying on the specific data layout/padding details.
        let len_start = self.keylen.next_multiple_of(2) as usize;
        let len_end = len_start + size_of::<u16>();
        let len = u16::from_ne_bytes(self.bytes[len_start..len_end].try_into().unwrap());
        &self.bytes[len_end..len_end + len as usize]
    }

    fn set_value(&mut self, value: &[u8]) {
        let len_start = self.keylen.next_multiple_of(2) as usize;
        let len_end = len_start + size_of::<u16>();
        self.bytes[len_start..len_end].copy_from_slice(&(value.len() as u16).to_ne_bytes());
        self.bytes[len_end..len_end + value.len()].copy_from_slice(value);
    }

    fn new(key: &[u8]) -> Self {
        let mut bytes = [0; 65794];
        bytes[..key.len()].copy_from_slice(key);
        Self {
            deplnk: null_mut(),
            keylen: key.len() as u8,
            bytes,
        }
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
        self.find(|x| x.key() >= key).filter(|x| x.key() == key)
    }
}

struct Cursor<'a> {
    //Stores the reference to the current node of the linked list.
    current: &'a mut *mut ST_DEPEND,
}

impl<'a> Cursor<'a> {
    fn advance(&mut self) {
        let ptr = *self.current;
        if let Some(current_node) = unsafe { ptr.as_mut() } {
            self.current = &mut current_node.deplnk;
        } else {
            //Do nothing (stay at the end of the list)
        }
    }

    //consider replacing with some sort of get
    fn key(&self) -> Option<&[u8]> {
        let current = unsafe { (*self.current).as_ref() }?;
        Some(current.key())
    }

    //consider replacing with some sort of get_mut
    fn set_value(&mut self, data: &CArrayString) -> Result<(), &'static str> {
        let current = unsafe { (*self.current).as_mut() }.ok_or("Set non existent node")?;
        current.set_value(data.content());
        Ok(())
    }

    //inserts at the current position.
    //So if we have ABC current is at B and we insert N
    //the resulting structure is ANBC
    fn insert(&mut self, key: &[u8]) {
        let next = *self.current;
        let mut new_node = Box::new(ST_DEPEND::new(key));
        new_node.deplnk = next;
        *self.current = Box::into_raw(new_node);
    }

    //removes the current node
    fn remove(&mut self) {
        if *self.current != null_mut() {
            let next = unsafe { (*self.current).as_ref() }
                .expect("null check was already preformed")
                .deplnk;
            //un-link the node
            let tmp = *self.current;
            *self.current = next;
            //convert node back into a box so it can be dropped.
            let _ = unsafe { Box::from_raw(tmp) };
        } else {
            //do nothing if at the end of the list.
        }
    }

    fn at_list_end(&self) -> bool {
        self.key().is_none()
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

    fn set_root(&mut self, data: &CArrayString) {
        let content = data.content();
        self.dbc = content.len() as u16;
        self.data[..content.len()].copy_from_slice(content);
    }

    fn value(&self, key: &[u8]) -> Option<CArrayString> {
        if key.is_empty() {
            self.root_value()
        } else {
            //TODO This can be optimized using using the last key value
            //I am not doing that optimization at the moment since I want this to be a &self function
            //I may try and add it with inner mutability in the future
            DependIter::new(self).find_key(key).map(ST_DEPEND::value)
        }
        .map(|x| x.try_into().unwrap())
    }

    fn set_value(&mut self, key: &[u8], data: &CArrayString) {
        if key.is_empty() {
            self.set_root(data);
        } else {
            let mut cursor = Cursor {
                current: &mut self.deplnk,
            };
            while let Some(x) = cursor.key()
                && x < key
            {
                cursor.advance();
            }
            if Some(key) != cursor.key() {
                cursor.insert(key);
            }
            cursor
                .set_value(data)
                .expect("either found key or just inserted key");
        }
    }

    fn kill(&mut self, key: &[u8]) {
        let mut cursor = Cursor {
            current: &mut self.deplnk,
        };
        if key.is_empty() {
            while !cursor.at_list_end() {
                cursor.remove()
            }

            //Clear values
            self.last_key = null_mut();
            self.dbc = VAR_UNDEFINED as u16;
        } else {
            //advance before the index
            while let Some(x) = cursor.key()
                && x < key
            {
                cursor.advance();
            }
            //Remove all entries prefixed with the killed key
            while let Some(x) = cursor.key()
                && x[..key.len()] == *key
            {
                //NOTE if we ever start using last key wee need up update it hear.
                cursor.remove()
            }
        }
    }

    //checks self contains any data, if not it can be fried
    //TODO I don't really understand how attached is supposed to work so am skipping mutation testing
    //on this for the moment
    #[cfg_attr(test, mutants::skip)]
    fn contains_data(&self) -> bool {
        !(self.deplnk == null_mut() && self.attach <= 1 && self.dbc == VAR_UNDEFINED as u16)
    }
}

impl Table {
    fn get(&self, var: &MVAR) -> Option<CArrayString> {
        self.locate(var.name)
            .and_then(|index| unsafe { self[index].data.as_ref() })
            .and_then(|data| data.value(var.key()))
    }

    fn set(&mut self, var: &MVAR, value: &CArrayString) -> Result<(), ()> {
        let index = self.create(var.name).map_err(|_| ())?;

        if unsafe { self[index].data.as_mut() }.is_none() {
            self[index].data = Box::into_raw(Box::new(ST_DATA {
                deplnk: null_mut(),
                last_key: null_mut(),
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

    fn kill(&mut self, var: &MVAR) {
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
            if self[index].data == null_mut() && self[index].usage == 0 {
                self.free(var.name);
            }
        }
    }
}

#[cfg(test)]
pub mod tests {

    use pretty_assertions::assert_eq;

    use crate::key::{CArrayString, List};

    use super::c_code::{Table, MVAR, VAR_U};
    pub fn var_u(var: &str) -> VAR_U {
        var.try_into().unwrap()
    }

    //TODO clean this up at some point
    pub fn var_m(name: &str, keys: &[&str]) -> MVAR {
        let mut key_buff = List::new();
        for key in keys {
            key_buff.push(&((*key).into())).unwrap();
        }

        let mut key = [0; 256];
        let len = key_buff.len();
        key[..key_buff.len()].copy_from_slice(key_buff.raw_keys());

        MVAR {
            name: var_u(name),
            volset: Default::default(),
            uci: Default::default(),
            slen: len as u8,
            key,
        }
    }

    ///Temporary shim layer for calling the C code.
    impl Table {
        fn c_set(&mut self, var: &mut MVAR, data: &mut CArrayString) {
            self.set(var, data).unwrap();
            //unsafe { TMP_Set(from_mut(var), from_mut(data.as_mut()), from_mut(self)) };
        }
        fn c_get(&mut self, var: &mut MVAR) -> Result<CArrayString, i32> {
            self.get(var).ok_or(-6)
        }
        fn c_kill(&mut self, var: &mut MVAR) {
            //unsafe { TMP_Kill(from_mut(var), from_mut(self)) };
            self.kill(var);
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
        for (var, value) in &mut test_data {
            table.c_set(var, value);
        }

        for (mut var, value) in test_data {
            assert_eq!(Ok(*value), table.c_get(&mut var));
        }
    }

    #[test]
    fn kill_uninitialized_var() {
        let mut table = Table::new();
        //These should be noops.
        table.c_kill(&mut var_m("foo", &[]));
        table.c_kill(&mut var_m("foo", &["arg"]));
    }

    #[test]
    fn kill_initialized_root() {
        let mut table = Table::new();
        let mut var = var_m("foo", &[]);
        let var_i = var_m("foo", &["i"]);
        let var_ii = var_m("foo", &["i", "ii"]);
        let data: CArrayString = "data".into();
        table.set(&var, &data).unwrap();
        table.set(&var_i, &data).unwrap();
        table.set(&var_ii, &data).unwrap();
        table.c_kill(&mut var);
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
        let data: CArrayString = "data".into();
        table.set(&var, &data).unwrap();
        table.set(&var_i, &data).unwrap();
        table.set(&var_ii, &data).unwrap();
        table.c_kill(&mut var_i);
        assert_eq!(table.get(&var), Some(data));
        assert_eq!(table.get(&var_i), None);
        assert_eq!(table.get(&var_i), None);
    }

    #[test]
    fn kill_removes_only_specified_index() {
        let mut table = Table::new();
        let data: CArrayString = "data".into();
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
}

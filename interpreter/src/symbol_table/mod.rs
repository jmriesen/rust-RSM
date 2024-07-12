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
    pub use ffi::CSTRING;
    pub use ffi::VAR_U;
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

//Iterator over REVERENCES TO THE NEXT POINTERS (Not an iterator over ST_DEPEND)
struct DependIterMut<'a> {
    current: Option<&'a mut *mut ST_DEPEND>,
}

impl<'a> Iterator for DependIterMut<'a> {
    type Item = &'a mut *mut ST_DEPEND;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            let ptr_to_next_node = *current;
            let next_node = unsafe { ptr_to_next_node.as_mut() };
            self.current = next_node.map(|x| &mut x.deplnk);
            Some(current)
        } else {
            None
        }
    }
}

impl<'a> DependIterMut<'a> {
    fn new(data: &'a mut ST_DATA) -> Self {
        Self {
            current: Some(&mut data.deplnk),
        }
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
        //Get the key value stored in the pointed to ST_DEPEND
        let key_value = |x: &*mut ST_DEPEND| -> Option<&[u8]> {
            let next = *x;
            unsafe { next.as_ref() }.map(ST_DEPEND::key)
        };

        if key.is_empty() {
            self.set_root(data);
        } else {
            // advance until just before where the key's entire should be.
            let ref_to_next_ptr = DependIterMut::new(self)
                .find(|x| {
                    //if there is is a next key compare.
                    if let Some(next) = key_value(x) {
                        next >= key
                    } else {
                        //don't go past the end of the list
                        true
                    }
                })
                .expect(
                    "There will always be a next node since we only skip if the next key exists",
                );
            //Add a new node if it is needed
            if key_value(ref_to_next_ptr) != Some(key) {
                let mut new_node = Box::new(ST_DEPEND::new(key));
                new_node.deplnk = *ref_to_next_ptr;
                *ref_to_next_ptr = Box::into_raw(new_node);
            }

            let current = *ref_to_next_ptr;
            let current =
                unsafe { current.as_mut() }.expect("A next node exists or we just inserted one");
            current.set_value(data.content());
        }
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
}

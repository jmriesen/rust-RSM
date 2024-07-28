use std::collections::BTreeMap;

use crate::{key::Key, value::Value};

///Data associated for a specific variable
#[derive(Debug, PartialEq, Eq)]
pub struct VarData {
    sub_values: BTreeMap<Key, Value>,
    //TODO consider removing I am currently not using attach
    attach: ::std::os::raw::c_short,
    value: Option<Value>,
}

impl VarData {
    pub fn value(&self, key: &Key) -> Option<Value> {
        //TODO remove clones
        if key.is_empty() {
            self.value.clone()
        } else {
            //NOTE There is probably room for optimization here.
            //It is fairly common for the M code to access the values sequentially using $O
            //The C code speed up $O by string a reference to the last used node.
            //I am not doing this right now as it would require making a self referential type
            //and I am not focusing on performance right now. (just correctness)
            //NOTE you could also probably accomplish the last key thing using using a sorted vec
            self.sub_values.get(key).cloned()
        }
    }

    pub fn set_value(&mut self, key: &Key, data: &Value) {
        if key.is_empty() {
            self.value = Some(data.clone());
        } else {
            let _ = self.sub_values.insert(key.clone(), data.clone());
        }
    }

    pub fn kill(&mut self, key: &Key) {
        if key.is_empty() {
            //Clear values
            self.sub_values = BTreeMap::default();
            self.value = None;
        } else {
            //NOTE Removing a range of keys seems like something the std BTree map should support,
            //However it looks like there is still some design swirl going on, and the design has
            //not been touched in a while
            //https://github.com/rust-lang/rust/issues/81074
            //So I just chose to use the cursor api for now.
            let mut cursor = self
                .sub_values
                .lower_bound_mut(std::ops::Bound::Included(key));
            while let Some((current_key, _)) = cursor.peek_next()
                && current_key.is_sub_key_of(key)
            {
                cursor.remove_next();
            }
        }
    }

    //checks self contains any data, if not it can be freed
    //TODO I don't really understand how attached is supposed to work so am skipping mutation testing
    //on this for the moment
    #[cfg_attr(test, mutants::skip)]
    pub fn contains_data(&self) -> bool {
        !(self.sub_values.is_empty() && self.attach <= 1 && self.value.is_none())
    }
}

impl Default for VarData {
    fn default() -> Self {
        Self {
            sub_values: Default::default(),
            attach: 0,
            value: None,
        }
    }
}

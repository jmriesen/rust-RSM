use super::{Tab, Table};
use std::fmt::Debug;
impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("Table");
        builder.field("sym_tab", &self.sym_tab);
        builder.field("hash", &self.st_hash_temp);
        builder.finish()
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.sym_tab == other.sym_tab && self.st_hash_temp == other.st_hash_temp
    }
}

impl Eq for Table {}

impl Eq for Tab {}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.fwd_link == other.fwd_link
        && self.usage == other.usage
        //Note data is a pointer 
        //We well need to switch to deep copies at some point.
        && self.data == other.data
        && self.varnam == other.varnam
    }
}

impl Debug for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("forward_link", &{ self.fwd_link })
            .field("usage", &{ self.usage })
            .field("data", &{ self.data })
            .field("variable name", &self.varnam)
            .finish()
    }
}

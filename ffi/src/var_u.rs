use std::array::from_fn;

use symbol_table::{key::KeyType, MVar, VarU};
pub trait IntoCvarU {
    fn into_c(self) -> crate::bindings::VAR_U;
}

impl IntoCvarU for VarU {
    fn into_c(self) -> crate::bindings::VAR_U {
        let mut content = self.contents().iter().cloned();
        crate::bindings::VAR_U {
            var_cu: from_fn(|_| content.next().unwrap_or(0)),
        }
    }
}

pub trait IntoCmVar {
    fn into_c(self) -> crate::bindings::MVAR;
}

impl<Key: KeyType> IntoCmVar for MVar<Key> {
    #[must_use]
    fn into_c(self) -> crate::bindings::MVAR {
        let (slen, key) = self.key.borrow().clone().into_ckey();
        crate::bindings::MVAR {
            name: self.name.into_c(),
            volset: self.volset,
            uci: self.uci,
            slen,
            key,
        }
    }
}

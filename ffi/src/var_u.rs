use crate::IntoC;
use std::array::from_fn;
use symbol_table::{key::KeyType, MVar, VarU};

impl IntoC for VarU {
    type CType = crate::bindings::VAR_U;
    fn into_c(self) -> Self::CType {
        let mut content = self.contents().iter().cloned();
        Self::CType {
            var_cu: from_fn(|_| content.next().unwrap_or(0)),
        }
    }
}

impl<Key: KeyType> IntoC for MVar<Key> {
    type CType = crate::bindings::MVAR;
    #[must_use]
    fn into_c(self) -> Self::CType {
        let (slen, key) = self.key.borrow().clone().into_ckey();
        Self::CType {
            name: self.name.into_c(),
            volset: self.volset,
            uci: self.uci,
            slen,
            key,
        }
    }
}

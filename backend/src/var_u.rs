use std::fmt::Debug;

use symbol_table::{MAX_VAR_NAME_SIZE, VariableName};

use crate::{
    BiteCode, Compile,
    runtime::{Decode, program_counter::AssemblyDecoder},
};
const VAR_LEN: i32 = 32;

static _VAR_IDENT_NAME_LENGTH: () = {
    //TODO The VariableName and Idents should probably be the same thing or one as a NewType wrapper
    //around the other.
    //Keeping separate for now to avoid a big refactor while I am still feeling out where the ident
    //type will be used.
    //Also would have to resolve where Ident is defined, in the ir in symbol table in a shared new
    //crate?
    assert!(VAR_LEN as usize == MAX_VAR_NAME_SIZE);
};
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ident([u8; 32]);

#[cfg_attr(test, mutants::skip)]
impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner: VariableName = self.clone().into();
        f.debug_tuple("Ident").field(&inner).finish()
    }
}

impl TryFrom<&str> for Ident {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() > VAR_LEN as usize {
            Err(concat!(
                "Ident name is too long max length is :",
                stringify!(VAR_LEN)
            ))
        } else {
            let mut raw = [0; _];
            raw[..bytes.len()].copy_from_slice(bytes);
            Ok(Self(raw))
        }
    }
}

impl Compile for Ident {
    type Context = ();

    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        bite_code.extend(self.0);
    }
}

impl Decode for Ident {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self(decoder.consume_n()))
    }
}
impl Into<VariableName> for Ident {
    fn into(self) -> VariableName {
        //TODO Check if this end makes since I am not sure if Ident is allowed to have null
        //bytes.
        let end = self.0.iter().position(|x| x == &0).unwrap_or(self.0.len());
        VariableName::new(&self.0[..end])
            .expect("Can never fail due to const assertions that max sizes are the same.")
    }
}

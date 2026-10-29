use std::iter::repeat;

use symbol_table::{Ident, MAX_VAR_NAME_SIZE};

use crate::{
    ByteCode, Compile,
    runtime::{Decode, program_counter::AssemblyDecoder},
};

impl Compile for Ident {
    type Context = ();

    fn compile(&self, bite_code: &mut ByteCode, _: &Self::Context) {
        bite_code.extend(
            self.contents()
                .iter()
                .cloned()
                .chain(repeat(0))
                .take(MAX_VAR_NAME_SIZE),
        );
    }
}

impl Decode for Ident {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let raw: [u8; MAX_VAR_NAME_SIZE] = decoder.consume_n();
        let end = raw
            .iter()
            .position(|x| *x == 0)
            .unwrap_or(MAX_VAR_NAME_SIZE);
        Some(
            Self::new(&(raw[..end]))
                .expect("Should never fail because we only decode what we have encoded."),
        )
    }
}

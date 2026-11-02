use crate::{
    Compile,
    byte_code::{ByteCode, EncoderExt},
    macros::OpCodesForeign,
    runtime::{Decode, program_counter::AssemblyDecoder},
    variable::VarContext,
};
use ir::commands::{New, NewKind};
use serde::{Deserialize, Serialize};

OpCodesForeign!(NewKind{
    Inclusive => 164,
    Exclusive => 165,
});

impl Compile for New {
    type Context = ();
    fn compile(&self, byte_code: &mut ByteCode, _: &()) {
        self.vars.compile(byte_code, &VarContext::Build);
        NewStackAsm {
            kind: self.kind.clone(),
            number_of_variables: self.vars.len() as u8,
        }
        .encode(byte_code);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStackAsm {
    pub kind: NewKind,
    pub number_of_variables: u8,
}

impl Decode for NewStackAsm {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self {
            kind: NewKind::decode(decoder)?,
            number_of_variables: decoder.consume_n::<1>()[0],
        })
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        self.kind.encode(byte_code);
        byte_code.push_raw(self.number_of_variables);
    }
}

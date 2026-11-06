use ir::commands::kill::{Kill, KillType};
use serde::{Deserialize, Serialize};

use crate::{
    Compile,
    macros::OpCodesForeign,
    runtime::{LegacyEncoding, program_counter::AssemblyDecoder},
    variable::VarContext,
};

OpCodesForeign! {
    KillType{
        Inclusive => 166,
        Exclusive => 167,
    }
}

impl Compile for Kill {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::ByteCode, _context: &Self::Context) {
        self.variables.compile(bite_code, &VarContext::Build);
        KillInstruction {
            r#type: self.r#type,
            number_of_variables: self.variables.len() as u8,
        }
        .compile(bite_code, &());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KillInstruction {
    pub r#type: KillType,
    pub number_of_variables: u8,
}
impl Compile for KillInstruction {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::ByteCode, _context: &Self::Context) {
        self.encode(bite_code);
    }
}
impl LegacyEncoding for KillInstruction {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self {
            r#type: KillType::decode(decoder)?,
            number_of_variables: decoder.consume_n::<1>()[0],
        })
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        use crate::byte_code::EncoderExt;
        self.r#type.encode(byte_code);
        byte_code.push_raw(self.number_of_variables);
    }
}

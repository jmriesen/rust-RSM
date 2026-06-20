use ir::commands::kill::{Kill, KillType};

use crate::{
    Compile,
    runtime::{Decode, Encode, OpCodesForeign, byte_code::AssemballyDecoder},
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

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        self.variables.compile(bite_code, &VarContext::Build);
        KillInstruction {
            r#type: self.r#type,
            number_of_variables: self.variables.len() as u8,
        }
        .compile(bite_code, &());
    }
}

#[derive(Debug)]
pub struct KillInstruction {
    pub r#type: KillType,
    pub number_of_variables: u8,
}
impl Compile for KillInstruction {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        bite_code.push(self.r#type.encode());
        bite_code.push(self.number_of_variables);
    }
}
impl Decode for KillInstruction {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        Some(Self {
            r#type: KillType::decode(decoder)?,
            number_of_variables: decoder.consume_n::<1>()[0],
        })
    }
}

use crate::{
    Compile,
    bite_code::BiteCode,
    macros::OpCodesForeign,
    runtime::{Decode, Encode, program_counter::AssemblyDecoder},
    variable::VarContext,
};
use ir::commands::{New, NewKind};

OpCodesForeign!(NewKind{
    Inclusive => 164,
    Exclusive => 165,
});

impl Compile for New {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        self.vars.compile(bite_code, &VarContext::Build);
        bite_code.push(self.kind.encode());
        bite_code.push(self.vars.len() as u8);
    }
}

#[derive(Debug)]
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
}

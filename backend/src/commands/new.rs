use crate::{
    Compile,
    bite_code::BiteCode,
    macros::OpCodes,
    runtime::{Decode, program_counter::AssemblyDecoder},
    variable::VarContext,
};
use ir::commands::New;

OpCodes! {NewCodes{
    Inclusive = 164,
    Exclusive = 165,
}}
impl Compile for New {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        let (vars, typ) = match self {
            New::Exclusive(vars) => (vars, NewCodes::Exclusive),
            New::Inclusive(vars) => (vars, NewCodes::Inclusive),
        };
        vars.compile(bite_code, &VarContext::Build);
        bite_code.push(typ as u8);
        bite_code.push(vars.len() as u8);
    }
}

#[derive(Debug)]
pub struct NewStackAsm {
    pub r#type: NewCodes,
    pub number_of_variables: u8,
}

impl Decode for NewStackAsm {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self {
            r#type: NewCodes::decode(decoder)?,
            number_of_variables: decoder.consume_n::<1>()[0],
        })
    }
}

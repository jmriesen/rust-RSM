use crate::{
    macros::OpCodes,
    runtime::{
        Decode,
        program_counter::{self, AssemblyDecoder},
    },
};
OpCodes! {
    JumpCodes {
        Conditional = 5,
        Unconditional = 172,
}}
#[derive(Debug)]
pub struct Jump {
    pub r#type: JumpCodes,
    pub target: program_counter::Location,
}
impl Decode for Jump {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self {
            r#type: JumpCodes::decode(decoder)?,
            target: Decode::decode(decoder)?,
        })
    }
}

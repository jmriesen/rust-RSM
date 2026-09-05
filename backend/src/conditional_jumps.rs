use crate::runtime::{
    Decode, OpCodes,
    program_counter::{self, AssemballyDecoder},
};
OpCodes! {JumpCodes {
        Conditional = 5,
        Unconditional = 172,
}}
#[derive(Debug)]
pub struct Jump {
    pub r#type: JumpCodes,
    pub target: program_counter::Location,
}
impl Decode for Jump {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        Some(Self {
            r#type: JumpCodes::decode(decoder)?,
            target: Decode::decode(decoder)?,
        })
    }
}

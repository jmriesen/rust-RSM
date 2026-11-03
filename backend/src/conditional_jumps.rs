use serde::{Deserialize, Serialize};

use crate::{
    byte_code::Location,
    macros::OpCodes,
    runtime::{Decode, program_counter::AssemblyDecoder},
};
OpCodes! {
    JumpCodes {
        Conditional = 5,
        Unconditional = 172,
}}
#[derive(Debug, Serialize, Deserialize)]
pub struct Jump {
    pub r#type: JumpCodes,
    pub target: Location,
}
impl Decode for Jump {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self {
            r#type: JumpCodes::decode(decoder)?,
            target: Decode::decode(decoder)?,
        })
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        self.r#type.encode(byte_code);
        self.target.encode(byte_code);
    }
}

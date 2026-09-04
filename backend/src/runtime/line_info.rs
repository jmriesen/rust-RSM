use crate::{
    Compile,
    runtime::{Decode, OpCode, program_counter::AssemballyDecoder},
};

OpCode! {EndLine=0}
OpCode! {LineNum=170}

#[derive(Debug)]
pub struct StartLine {
    pub line_numb: u16,
    pub level: u16,
}

impl Decode for StartLine {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        LineNum::decode(decoder)?;
        Some(StartLine {
            line_numb: u16::from_le_bytes(decoder.consume_n()),
            level: u16::from_le_bytes(decoder.consume_n()),
        })
    }
}

impl Compile for StartLine {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        bite_code.push(LineNum.encode());
        bite_code.extend(self.line_numb.to_le_bytes());
        bite_code.extend(self.level.to_le_bytes());
    }
}

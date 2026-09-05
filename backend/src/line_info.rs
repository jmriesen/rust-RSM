use ir::Line;

use crate::{
    BiteCode, Compile,
    runtime::{Decode, OpCode, program_counter::AssemballyDecoder},
};

OpCode! {EndLine=0}
OpCode! {LineNum=170}

impl Compile for Line {
    type Context = u16;

    fn compile(&self, bite_code: &mut BiteCode, line_numb: &Self::Context) {
        StartLine {
            line_numb: *line_numb,
            level: self.level,
        }
        .compile(bite_code, &());
        //NOTE: I have decided to change the bite code layout compared to the original C code.
        //In the ordinal C the line level/check is part of the stack machine instructions.
        //In this version it will be unconditional included as part of the line encoding.
        self.commands.compile(bite_code, &());
        bite_code.push(EndLine.encode());
    }
}

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

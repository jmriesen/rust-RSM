use ir::commands::r#for::{For, ForKind};
use serde::{Deserialize, Serialize};

use crate::{
    Compile,
    byte_code::{ByteCode, JumpOffset},
    expression::ExpressionContext,
    macros::{OpCode, OpCodes},
    runtime::{
        EndCommand, LegacyEncoding, NoOpCode, StackAssembally, program_counter::AssemblyDecoder,
    },
    variable::VarContext,
};

OpCodes! {
ForParameterType {
    Value = 174,
    Unbounded = 175,
    Bounded = 176,
}}

OpCodes! {
ForStart {
    ArgumentLess= 173,
    // NOTE: Same as VarContext::For IMPORTANT
    Arguments = 177,
}}
OpCode! {ForEnd =178}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForMetaData {
    pub loop_body: JumpOffset,
    pub r#break: JumpOffset,
    pub kind: ForStart,
}

impl Compile for For {
    type Context = ();
    /*
     * NOTE I have changed the format from the original C code's to make it easier to encode/decode
     */
    fn compile(&self, bite_code: &mut ByteCode, _: &()) {
        //Loop Body
        let mut loop_body_byte_code = ByteCode::new();
        self.commands.compile(&mut loop_body_byte_code, &());
        //Inserting an extra `EndCommand` command (probably not needed)
        loop_body_byte_code.push_asm(StackAssembally::EndCommand(EndCommand));
        //Insert end off loop logic
        loop_body_byte_code.push_asm(StackAssembally::ForEnd(ForEnd));

        // Arguments
        let mut args_byte_code = ByteCode::new();
        match &self.kind {
            ForKind::VarLoop {
                variable,
                arguments,
            } => {
                variable.compile(bite_code, &VarContext::Build);
                //-----
                for range in arguments {
                    range
                        .start
                        .compile(&mut args_byte_code, &ExpressionContext::Eval);

                    if let Some((inc, end)) = &range.increment_end {
                        inc.compile(&mut args_byte_code, &ExpressionContext::Eval);

                        if let Some(end) = end {
                            end.compile(&mut args_byte_code, &ExpressionContext::Eval);
                        }
                    }

                    args_byte_code.push_asm(StackAssembally::ForParameterType(
                        match range.increment_end {
                            None => ForParameterType::Value,
                            Some((_, None)) => ForParameterType::Unbounded,
                            Some((_, Some(_))) => ForParameterType::Bounded,
                        },
                    ));
                }
            }
            ForKind::Infinite => {}
        }

        ForMetaData {
            loop_body: JumpOffset(args_byte_code.len() as i16),
            r#break: JumpOffset(args_byte_code.len() as i16 + loop_body_byte_code.len() as i16),
            kind: match self.kind {
                ForKind::Infinite => ForStart::ArgumentLess,
                ForKind::VarLoop { .. } => ForStart::Arguments,
            },
        }
        .encode(bite_code);
        bite_code.append(args_byte_code);
        bite_code.append(loop_body_byte_code);

        bite_code.push_asm(StackAssembally::NoOpCode(NoOpCode));
    }
}

impl LegacyEncoding for ForMetaData {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let kind = ForStart::decode(decoder)?;
        let loop_body =
            LegacyEncoding::decode(decoder).expect("already verified we are in for set");
        let r#break = LegacyEncoding::decode(decoder).expect("already verified we are in for set");

        Some(ForMetaData {
            loop_body,
            r#break,
            kind,
        })
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        self.kind.encode(byte_code);
        self.loop_body.encode(byte_code);
        self.r#break.encode(byte_code);
    }
}

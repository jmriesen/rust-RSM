use ir::commands::r#for::{For, ForKind};
use serde::{Deserialize, Serialize};
use symbol_table::Ident;

use crate::{
    Compile,
    byte_code::{ByteCode, EncoderExt, Location},
    expression::ExpressionContext,
    macros::{OpCode, OpCodes},
    runtime::{Decode, EndCommand, NoOpCode, StackAssembally, program_counter::AssemblyDecoder},
    variable::{BuildVarInstructions, VarContext},
};

OpCodes! {
ForRangeType {
    One = 174,
    Two = 175,
    Three = 176,
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
    pub loop_body: Location,
    pub r#break: Location,
    pub kind: ForStart,
}

impl Compile for For {
    type Context = ();
    /*
     * | Meta Data                                                                        |  Args Program Counter                | Body PC | Meta Data |
     * |               | ForStart::Argument_less|                | body_jump | break_jump |                                      | Body    | For End   |
     * | Variable args | ForStart::Arguments    | Variable name  | body_jump | break_jump |  (Range | For Range type argument )* | Body    | For End   |
     * NOTE: this odd splicing the opcode inside of the variable is standard for how variables are
     * encoded.
     * The variable arguments **must** come before the variable (they need to be evaluated and on the
     * stack) We could have a opcode that is just build this variable and hold onto it for a second,
     * but that is going down the same route as for_preamble (which I am trying to get rid of, due
     * to barrow checking/state being weird with incomplete objects.)
     * */
    fn compile(&self, bite_code: &mut ByteCode, _: &()) {
        let mut loop_body_byte_code = ByteCode::new();
        self.commands.compile(&mut loop_body_byte_code, &());
        //Inserting an extra `EndCommand` command (probably not needed)
        loop_body_byte_code.push_asm(StackAssembally::EndCommand(EndCommand));
        //Insert end off loop logic
        loop_body_byte_code.push_asm(StackAssembally::ForEnd(ForEnd));

        match &self.kind {
            ForKind::VarLoop {
                variable,
                arguments,
            } => {
                variable.compile(bite_code, &VarContext::Build);
                //-----
                let mut args_byte_code = ByteCode::new();
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

                    args_byte_code.push(match range.increment_end {
                        None => ForRangeType::One,
                        Some((_, None)) => ForRangeType::Two,
                        Some((_, Some(_))) => ForRangeType::Three,
                    } as u8);
                }
                //-----

                dbg!(&args_byte_code);
                ForMetaData {
                    loop_body: Location(args_byte_code.0.len() + 2), //+2 to compensate for position
                    //deference of loop body not being the last thing in the structure.
                    r#break: Location(args_byte_code.0.len() + loop_body_byte_code.0.len()),
                    kind: match self.kind {
                        ForKind::Infinite => ForStart::ArgumentLess,
                        ForKind::VarLoop { .. } => ForStart::Arguments,
                    },
                }
                .encode(bite_code);
                bite_code.extend(args_byte_code.0);
                bite_code.extend(loop_body_byte_code.0);

                //TODO:
                //VarContext is the same as `ForStart::Arguments`
                //make them the same in the type system.
                const _: () = const {
                    assert!(VarContext::For as u8 == ForStart::Arguments as u8);
                };
            }
            ForKind::Infinite => {
                ForMetaData {
                    loop_body: Location(0),
                    r#break: Location(loop_body_byte_code.0.len()),
                    kind: match self.kind {
                        ForKind::Infinite => ForStart::ArgumentLess,
                        ForKind::VarLoop { .. } => ForStart::Arguments,
                    },
                }
                .encode(bite_code);
                bite_code.extend(loop_body_byte_code.0);
            }
        }

        // Inserting loop body
        //////bite_code.write_jump(loop_body, bite_code.current_location());

        //bite_code.extend(loop_body_byte_code.0);

        //Inserting an extra `EndCommand` command (probably not needed)
        //bite_code.push_asm(StackAssembally::EndCommand(EndCommand));
        //Insert end off loop logic
        //bite_code.push_asm(StackAssembally::ForEnd(ForEnd));
        // Jump out of for loop
        //     bite_code.write_jump(r#break, bite_code.current_location());
        // Insure the break jump always lands on a no-op
        bite_code.push_asm(StackAssembally::NoOpCode(NoOpCode));
    }
}

impl Decode for ForMetaData {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let kind = ForStart::decode(decoder)?;
        let loop_body = Decode::decode(decoder).expect("already verified we are in for set");
        let r#break = Decode::decode(decoder).expect("already verified we are in for set");

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

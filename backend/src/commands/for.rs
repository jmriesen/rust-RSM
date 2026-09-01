use ir::commands::r#for::{For, ForKind};

use crate::{
    Compile,
    bite_code::BiteCode,
    expression::ExpressionContext,
    runtime::{Decode, EndCommand, NoOpCode, OpCode, OpCodes, program_counter::AssemballyDecoder},
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

#[derive(Debug)]
pub struct ForArgMetaData {
    pub loop_variable: BuildVarInstructions,
    /// tracks source code for next `Range` group.
    pub range_pc: crate::runtime::program_counter::Location,
}

#[derive(Debug)]
pub struct ForMetaData {
    pub loop_body: crate::runtime::program_counter::Location,
    pub r#break: crate::runtime::program_counter::Location,
    pub args: Option<ForArgMetaData>,
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
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match &self.kind {
            ForKind::VarLoop {
                variable,
                arguments: _,
            } => {
                //TODO:
                //VarContext is the same as `ForStart::Arguments`
                //make them the same in the type system.
                variable.compile(bite_code, &VarContext::For)
            }
            ForKind::Infinite => bite_code.push(ForStart::ArgumentLess as u8),
        }

        let loop_body = bite_code.reserve_jump();
        let r#break = bite_code.reserve_jump();

        if let ForKind::VarLoop {
            variable: _,
            arguments,
        } = &self.kind
        {
            for range in arguments {
                range.start.compile(bite_code, &ExpressionContext::Eval);

                if let Some((inc, end)) = &range.increment_end {
                    inc.compile(bite_code, &ExpressionContext::Eval);

                    if let Some(end) = end {
                        end.compile(bite_code, &ExpressionContext::Eval);
                    }
                }

                bite_code.push(match range.increment_end {
                    None => ForRangeType::One,
                    Some((_, None)) => ForRangeType::Two,
                    Some((_, Some(_))) => ForRangeType::Three,
                } as u8);
            }
        }

        // Inserting loop body
        bite_code.write_jump(loop_body, bite_code.current_location());
        self.commands.compile(bite_code, &());
        //Inserting an extra `EndCommand` command (probably not needed)
        bite_code.push(EndCommand.encode());
        //Insert end off loop logic
        bite_code.push(ForEnd.encode());
        // Jump out of for loop
        bite_code.write_jump(r#break, bite_code.current_location());
        // Insure the break jump always lands on a no-op
        bite_code.push(NoOpCode.encode());
    }
}

impl Decode for ForMetaData {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        let r#type = ForStart::decode(decoder)?;

        let variable = match r#type {
            ForStart::ArgumentLess => None,
            ForStart::Arguments => {
                let loop_variable = Decode::decode(decoder).unwrap();
                Some(loop_variable)
            }
        };
        let loop_body = Decode::decode(decoder).expect("already verified we are in for set");
        let r#break = Decode::decode(decoder).expect("already verified we are in for set");

        let args = variable.map(|loop_variable| ForArgMetaData {
            loop_variable,
            range_pc: decoder.current_location(),
        });

        Some(ForMetaData {
            loop_body,
            r#break,
            args,
        })
    }
}

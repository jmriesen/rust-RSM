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
    // Repurposing VarContext::For
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
     * | Meta Data                                       |  Args Program Counter            | Body PC | Meta Data |
     * | ForStart::Argument_less| body_jump | break_jump | _ | _                                       | Body    | For End   |
     * | ForStart::Arguments    | body_jump | break_jump | Variable | (Range | For Range type argument )* | Body    | For End   |
     *
     * */
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        bite_code.push(match &self.kind {
            ForKind::Infinite => ForStart::ArgumentLess,
            ForKind::VarLoop { .. } => ForStart::Arguments,
        } as u8);

        let loop_body = bite_code.reserve_jump();
        let r#break = bite_code.reserve_jump();

        if let ForKind::VarLoop {
            variable,
            arguments,
        } = &self.kind
        {
            variable.compile(bite_code, &VarContext::For);

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
        let loop_body = Decode::decode(decoder).expect("already verified we are in for set");
        let r#break = Decode::decode(decoder).expect("already verified we are in for set");

        let args = match r#type {
            ForStart::ArgumentLess => {
                /*No arguments to decode*/
                None
            }
            ForStart::Arguments => {
                //TODO: Remove old variable building context (unneeded due to new bite_code layout.)
                let [_] = decoder.consume_n();
                let loop_variable = Decode::decode(decoder).unwrap();

                Some(ForArgMetaData {
                    loop_variable,
                    range_pc: decoder.current_location(),
                })
            }
        };

        Some(ForMetaData {
            loop_body,
            r#break,
            args,
        })
    }
}

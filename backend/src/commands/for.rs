use ir::commands::r#for::{For, ForKind};

use crate::{
    Compile,
    bite_code::{BiteCode, JumpCodes, JumpLocation, Location},
    expression::ExpressionContext,
    runtime::{Decode, EndCommand, NoOpCode, OpCode, OpCodes, program_counter::AssemballyDecoder},
    variable::{BuildVarInstructions, VarContext},
};

OpCodes! {
ForStart {
    One = 174,
    Two = 175,
    Three = 176,
}}

OpCode! {ForEnd =178}

impl Compile for For {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        struct EndBehavior {
            break_jump: JumpLocation,
            unconditional_jump: Option<Location>,
        }
        // Insert start of loop logic
        let end_behavior = {
            match &self.kind {
                ForKind::Infinite => {
                    //  | Type | jump break | content | extra end command | Jump unconditional.
                    //  Ok I can see why you might want to make this an unconditional jump.
                    //  Doing the lookup is technicality not needed, but I think I would rather
                    //  simplicity then absolute efficiently..
                    //  so what would it look like to rewrite this.
                    bite_code.push(JumpCodes::ForUnconditional as u8);
                    EndBehavior {
                        break_jump: bite_code.reserve_jump(),
                        unconditional_jump: Some(bite_code.current_location()),
                    }
                }
                ForKind::VarLoop {
                    variable,
                    arguments,
                } => {
                    //Variable | Jump to content  | jump break| Args | Type | content | extra end command | For end.
                    //Variable | Args | Type | jump break| content | extra end command | For end.
                    variable.compile(bite_code, &VarContext::For);
                    // Jump pass the rest of the for commands arguments and strait to the loop body
                    let jump_to_content = bite_code.reserve_jump();
                    // Break out of the loop.
                    let break_jump = bite_code.reserve_jump();

                    for args in arguments {
                        args.start.compile(bite_code, &ExpressionContext::Eval);
                        if let Some((inc, end)) = &args.increment_end {
                            inc.compile(bite_code, &ExpressionContext::Eval);
                            if let Some(end) = end {
                                end.compile(bite_code, &ExpressionContext::Eval);
                            }
                        }

                        bite_code.push(match args.increment_end {
                            None => ForStart::One,
                            Some((_, None)) => ForStart::Two,
                            Some((_, Some(_))) => ForStart::Three,
                        } as u8);
                    }

                    bite_code.write_jump(jump_to_content, bite_code.current_location());
                    EndBehavior {
                        break_jump,
                        unconditional_jump: None,
                    }
                }
            }
        };
        // Inserting loop body
        self.commands.compile(bite_code, &());
        //Inserting an extra OPENDC command (probably not needed)
        bite_code.push(EndCommand.encode());

        //Insert end off loop logic
        {
            let EndBehavior {
                break_jump,
                unconditional_jump,
            } = end_behavior;
            //This is a bit of wired ness? why is this differnt
            if let Some(location) = unconditional_jump {
                //Jump back to start of for loop.
                let jump = bite_code.unconditional_jump();
                bite_code.write_jump(jump, location);
            } else {
                bite_code.push(ForEnd.encode());
            }
            // Jump out of for loop
            bite_code.write_jump(break_jump, bite_code.current_location());
            bite_code.push(NoOpCode.encode());
        };
    }
}

#[derive(Debug)]
pub struct ForSet {
    pub loop_variable: BuildVarInstructions,
    pub loop_body: crate::runtime::program_counter::Location,
    pub r#break: crate::runtime::program_counter::Location,
}
impl Decode for ForSet {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        const CODE: u8 = VarContext::For as u8;
        if let [CODE] = decoder.consume_n() {
            let loop_variable =
                BuildVarInstructions::decode(decoder).expect("already verified we are in for set");
            let loop_body = Decode::decode(decoder).expect("already verified we are in for set");
            let break_jump = Decode::decode(decoder).expect("already verified we are in for set");

            Some(Self {
                loop_variable,
                loop_body,
                r#break: break_jump,
            })
        } else {
            None
        }
    }
}

use ir::commands::r#for::{For, ForKind};

use crate::{
    Compile, NO_OP_CODE,
    bite_code::{BiteCode, JumpCodes, JumpLocation, Location},
    commands::COMAND_END,
    expression::ExpressionContext,
    variable::VarContext,
};

enum ForCodes {
    One = 174,
    Two = 175,
    Three = 176,
    End = 178,
}

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
                            None => ForCodes::One,
                            Some((_, None)) => ForCodes::Two,
                            Some((_, Some(_))) => ForCodes::Three,
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
        bite_code.push(COMAND_END);

        //Insert end off loop logic
        {
            let EndBehavior {
                break_jump,
                unconditional_jump,
            } = end_behavior;
            if let Some(location) = unconditional_jump {
                //Jump back to start of for loop.
                let jump = bite_code.unconditional_jump();
                bite_code.write_jump(jump, location);
            } else {
                bite_code.push(ForCodes::End as u8);
            }
            // Jump out of for loop
            bite_code.write_jump(break_jump, bite_code.current_location());
            bite_code.push(NO_OP_CODE);
        };
    }
}

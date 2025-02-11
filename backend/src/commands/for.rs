use ir::commands::r#for::{For, ForKind};

use crate::{
    Compile,
    bite_code::{BiteCode, JumpLocation, Location},
    expression::ExpressionContext,
    variable::VarContext,
};

struct EndBehavior {
    jump_to_exit: JumpLocation,
    unconditional_jump: Option<Location>,
}

fn compile_preface(for_kind: &ForKind, bite_code: &mut BiteCode) -> EndBehavior {
    match for_kind {
        ForKind::Infinite => {
            bite_code.push(ffi::CMFOR0);
            EndBehavior {
                jump_to_exit: bite_code.reserve_jump(),
                unconditional_jump: Some(bite_code.current_location()),
            }
        }
        ForKind::VarLoop {
            variable,
            arguments,
        } => {
            variable.compile(bite_code, &VarContext::For);
            let jump_to_content = bite_code.reserve_jump();
            let jump_to_exit = bite_code.reserve_jump();

            for args in arguments {
                args.start.compile(bite_code, &ExpressionContext::Eval);
                if let Some((inc, end)) = &args.increment_end {
                    inc.compile(bite_code, &ExpressionContext::Eval);
                    if let Some(end) = end {
                        end.compile(bite_code, &ExpressionContext::Eval);
                    }
                }

                bite_code.push(match args.increment_end {
                    None => ffi::CMFOR1,
                    Some((_, None)) => ffi::CMFOR2,
                    Some((_, Some(_))) => ffi::CMFOR3,
                });
            }

            bite_code.write_jump(jump_to_content, bite_code.current_location());
            EndBehavior {
                jump_to_exit,
                unconditional_jump: None,
            }
        }
    }
}

fn compile_end_behavior(end_behavior: EndBehavior, bite_code: &mut BiteCode) {
    if let Some(location) = end_behavior.unconditional_jump {
        //Jump back to start of for loop.
        let jump = bite_code.unconditional_jump();
        bite_code.write_jump(jump, location);
    } else {
        bite_code.push(ffi::CMFOREND);
    }
    // Jump out of for loop
    bite_code.write_jump(end_behavior.jump_to_exit, bite_code.current_location());
    bite_code.push(ffi::OPNOP);
}

impl Compile for For {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        let end_behavior = compile_preface(&self.kind, bite_code);
        self.commands.compile(bite_code, &());
        //Inserting an extra OPENDC command (probably not needed)
        bite_code.push(ffi::OPENDC);
        compile_end_behavior(end_behavior, bite_code);
    }
}

use crate::{
    bite_code::{BiteCode, JumpLocation, Location},
    expression::ExpressionContext,
    ir::{Compile, Expression, Variable},
    localvar::VarContext,
};

use super::Command;

pub struct Argument {
    start: Expression,
    increment_end: Option<(Expression, Option<Expression>)>,
}

pub enum ForKind {
    Infinite,
    VarLoop {
        variable: Variable,
        //TODO insure this vector is none empty
        arguments: Vec<Argument>,
    },
}
pub struct For {
    kind: ForKind,
    commands: Vec<Command>,
}

struct EndBehavior {
    jump_to_exit: JumpLocation,
    unconditional_jump: Option<Location>,
}

impl ForKind {
    fn new(sitter: &lang_model::For, source_code: &str) -> Self {
        if let Some(var) = sitter.variable() {
            assert!(sitter.args().len() > 0);
            Self::VarLoop {
                variable: Variable::new(&var, source_code),
                arguments: sitter
                    .args()
                    .iter()
                    .map(|x| {
                        let mut iter = x
                            .children()
                            .into_iter()
                            .map(|x| Expression::new(&x, source_code));
                        Argument {
                            start: iter.next().expect("We should always have a starting value"),
                            increment_end: iter.next().map(|x| (x, iter.next())),
                        }
                    })
                    .collect(),
            }
        } else {
            Self::Infinite
        }
    }

    fn compile_preface(&self, bite_code: &mut BiteCode) -> EndBehavior {
        match self {
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
                variable.compile(bite_code, VarContext::For);
                let jump_to_content = bite_code.reserve_jump();
                let jump_to_exit = bite_code.reserve_jump();

                for args in arguments {
                    args.start.compile(bite_code, ExpressionContext::Eval);
                    if let Some((inc, end)) = &args.increment_end {
                        inc.compile(bite_code, ExpressionContext::Eval);
                        if let Some(end) = end {
                            end.compile(bite_code, ExpressionContext::Eval);
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
}
impl For {
    pub fn new(
        sitter: &lang_model::For,
        source_code: &str,
        line_tail: &mut dyn Iterator<Item = lang_model::command>,
    ) -> Self {
        let kind = ForKind::new(sitter, source_code);
        let mut commands = vec![];
        while let Some(command) = line_tail.next() {
            commands.push(Command::new(&command, source_code, line_tail));
        }
        Self { kind, commands }
    }
}
impl Compile for For {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        let end_behavior = self.kind.compile_preface(bite_code);
        self.commands.compile(bite_code, &());
        //Inserting an extra OPENDC command (probably not needed)
        bite_code.push(ffi::OPENDC);
        ForKind::compile_end_behavior(end_behavior, bite_code);
    }
}

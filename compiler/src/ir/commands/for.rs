use crate::{
    bite_code::{BiteCode, JumpLocation, Location},
    expression::ExpressionContext,
    ir::{Expression, Variable},
    localvar::VarContext,
};

pub struct Argument {
    start: Expression,
    increment_end: Option<(Expression, Option<Expression>)>,
}

pub enum For {
    Infinite,
    VarLoop {
        variable: Variable,
        //TODO insure this vector is none empty
        arguments: Vec<Argument>,
    },
}

impl For {
    pub fn new(sitter: &lang_model::For, source_code: &str) -> Self {
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
    pub fn compile(&self, comp: &mut BiteCode) -> EndOfLine {
        match self {
            For::Infinite => {
                comp.push(ffi::CMFOR0);
                EndOfLine {
                    jump_to_exit: comp.reserve_jump(),
                    unconditional_jump: Some(comp.current_location()),
                }
            }
            For::VarLoop {
                variable,
                arguments,
            } => {
                variable.compile(comp, VarContext::For);
                let jump_to_content = comp.reserve_jump();
                let jump_to_exit = comp.reserve_jump();

                for args in arguments {
                    args.start.compile(comp, ExpressionContext::Eval);
                    if let Some((inc, end)) = &args.increment_end {
                        inc.compile(comp, ExpressionContext::Eval);
                        if let Some(end) = end {
                            end.compile(comp, ExpressionContext::Eval);
                        }
                    }

                    comp.push(match args.increment_end {
                        None => ffi::CMFOR1,
                        Some((_, None)) => ffi::CMFOR2,
                        Some((_, Some(_))) => ffi::CMFOR3,
                    });
                }

                comp.write_jump(jump_to_content, comp.current_location());
                EndOfLine {
                    jump_to_exit,
                    unconditional_jump: None,
                }
            }
        }
    }
}

pub struct EndOfLine {
    jump_to_exit: JumpLocation,
    unconditional_jump: Option<Location>,
}

impl EndOfLine {
    pub fn compile(self, comp: &mut BiteCode) {
        //End for command
        comp.push(ffi::OPENDC);
        if let Some(location) = self.unconditional_jump {
            //Jump back to start of for loop.
            let jump = comp.unconditional_jump();
            comp.write_jump(jump, location);
        } else {
            comp.push(ffi::CMFOREND);
        }
        // Jump out of for loop
        comp.write_jump(self.jump_to_exit, comp.current_location());
        comp.push(ffi::OPNOP);

        comp.push(ffi::OPENDC);
    }
}

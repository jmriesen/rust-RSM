use lang_model::BrakeCommand;

use crate::{
    bite_code::BiteCode,
    ir::{Compile, Expression},
};

use super::{Command, PostCondition};

pub enum Break {
    ArgumentLess,
    Arg(Vec<Expression>),
}

impl Break {
    pub fn new(sitter: &BrakeCommand, source_code: &str) -> Command {
        Command::Break(PostCondition {
            condition: sitter
                .post_condition()
                .map(|x| Expression::new(&x, source_code)),
            value: if sitter.args().is_empty() {
                Self::ArgumentLess
            } else {
                Self::Arg(
                    sitter
                        .args()
                        .iter()
                        .map(|x| Expression::new(&x, source_code))
                        .collect(),
                )
            },
        })
    }
}
impl Compile for Break {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Self::ArgumentLess => {
                bite_code.push(ffi::OPBRK0);
            }
            Self::Arg(args) => {
                for arg in args {
                    arg.compile(bite_code, crate::expression::ExpressionContext::Eval);
                    if !matches!(arg, Expression::InderectExpression { .. }) {
                        bite_code.push(ffi::OPBRKN);
                    }
                }
            }
        }
    }
}

use lang_model::DoCommand;

use crate::{
    bite_code::BiteCode,
    ir::{Compile, Expression, ExtrinsicFunction},
};

use super::{Command, PostCondition};

pub enum Do {
    ArgumentLess,
    FunctionCall(Vec<PostCondition<ExtrinsicFunction>>),
}

impl Do {
    pub fn new(sitter: &DoCommand, source_code: &str) -> Command {
        Command::Do(PostCondition {
            condition: sitter
                .post_condition()
                .map(|x| Expression::new(&x, source_code)),
            value: if sitter.args().is_empty() {
                Self::ArgumentLess
            } else {
                Self::FunctionCall(
                    sitter
                        .args()
                        .iter()
                        .map(|x| PostCondition {
                            condition: x.post_condition().map(|x| Expression::new(&x, source_code)),
                            value: ExtrinsicFunction::new(&x.function(), source_code),
                        })
                        .collect(),
                )
            },
        })
    }
}
impl Compile for Do {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Do::ArgumentLess => bite_code.push(ffi::CMDON),
            Do::FunctionCall(x) => x.compile(bite_code, &crate::ExtrinsicFunctionContext::Do),
        }
    }
}

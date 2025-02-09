use lang_model::DoCommand;

use crate::{
    bite_code::BiteCode,
    ir::{Expression, ExtrinsicFunction},
};

pub enum Do {
    ArgumentLess,
    FunctionCall {
        post_condition: Option<Expression>,
        function_call: ExtrinsicFunction,
    },
}

impl Do {
    pub fn new(sitter: &DoCommand, source_code: &str) -> Vec<Self> {
        if sitter.args().is_empty() {
            vec![Self::ArgumentLess]
        } else {
            sitter
                .args()
                .iter()
                .map(|x| Self::FunctionCall {
                    post_condition: x.post_condition().map(|x| Expression::new(&x, source_code)),
                    function_call: ExtrinsicFunction::new(&x.function(), source_code),
                })
                .collect()
        }
    }
    pub fn compile(&self, comp: &mut BiteCode) {
        match self {
            Do::ArgumentLess => comp.push(ffi::CMDON),
            Do::FunctionCall {
                post_condition,
                function_call,
            } => {
                if let Some(condition) = post_condition {
                    comp.conditional_jump(condition, |x| {
                        function_call.compile(x, crate::ExtrinsicFunctionContext::Do)
                    })
                } else {
                    function_call.compile(comp, crate::ExtrinsicFunctionContext::Do);
                }
            }
        }
    }
}

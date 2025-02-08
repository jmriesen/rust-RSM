use lang_model::DoCommand;

use crate::{
    bite_code::BiteCode,
    expression::ExpressionContext,
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
                let jump_from = post_condition.as_ref().map(|x| {
                    x.compile(comp, ExpressionContext::Eval);
                    comp.push(ffi::JMP0);
                    comp.reserve_jump()
                });
                function_call.compile(comp, crate::ExtrinsicFunctionContext::Do);
                if let Some(from) = jump_from {
                    comp.write_jump(from, comp.current_location())
                }
            }
        }
    }
}

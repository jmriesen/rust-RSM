use lang_model::BrakeCommand;

use crate::{bite_code::BiteCode, ir::Expression};

pub struct Break(Option<Expression>);

impl Break {
    pub fn new(sitter: &BrakeCommand, source_code: &str) -> Vec<Self> {
        let children = sitter.args();
        if children.is_empty() {
            vec![Self(None)]
        } else {
            children
                .iter()
                .map(|x| Expression::new(x, source_code))
                .map(|x| Self(Some(x)))
                .collect()
        }
    }
    pub fn compile(&self, comp: &mut BiteCode) {
        match self {
            Self(None) => {
                comp.push(ffi::OPBRK0);
            }
            Self(Some(exp)) => {
                exp.compile(comp, crate::expression::ExpressionContext::Eval);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    comp.push(ffi::OPBRKN);
                }
            }
        }
    }
}

use crate::{
    bite_code::BiteCode,
    expression::ExpressionContext,
    ir::{commands::PostCondition, Compile, Expression},
};

use super::Command;

pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}

impl Write {
    pub fn new(sitter: &lang_model::WriteCommand, source_code: &str) -> Command {
        Command::Write(PostCondition {
            condition: sitter
                .post_condition()
                .map(|x| Expression::new(&x, source_code)),

            value: sitter
                .args()
                .iter()
                .map(|x| {
                    use lang_model::WriteArgChildren as E;
                    match x.children() {
                        E::Bang(_) => Self::Bang,
                        E::Clear(_) => Self::Clear,
                        E::Tab(tab) => Self::Tab(Expression::new(&tab.children(), source_code)),
                        E::Expression(expression) => {
                            Self::Expression(Expression::new(&expression, source_code))
                        }
                    }
                })
                .collect(),
        })
    }
}
impl Compile for Write {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        match self {
            Self::Bang => comp.push(ffi::CMWRTNL),
            Self::Clear => comp.push(ffi::CMWRTFF),
            Self::Tab(tab) => {
                tab.compile(comp, ExpressionContext::Eval);
                comp.push(ffi::CMWRTAB);
            }
            Self::Expression(exp) => {
                exp.compile(comp, ExpressionContext::Write);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    comp.push(ffi::CMWRTEX);
                }
            }
        }
    }
}

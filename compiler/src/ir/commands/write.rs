use lang_model::WriteArg;

use crate::{bite_code::BiteCode, expression::ExpressionContext, ir::Expression};

pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}

impl Write {
    pub fn new(sitter: &WriteArg, source_code: &str) -> Self {
        use lang_model::WriteArgChildren as E;
        match sitter.children() {
            E::Bang(_) => Self::Bang,
            E::Clear(_) => Self::Clear,
            E::Tab(tab) => Self::Tab(Expression::new(&tab.children(), source_code)),
            E::Expression(expression) => {
                Self::Expression(Expression::new(&expression, source_code))
            }
        }
    }
    pub fn compile(&self, comp: &mut BiteCode) {
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

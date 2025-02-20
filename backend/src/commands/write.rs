use ir::{Expression, commands::Write};

use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext};

enum WriteCodes {
    Bang = 53,
    Clear = 54,
    Tab = 55,
    Expression = 56,
}
impl Compile for Write {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        match self {
            Self::Bang => comp.push(WriteCodes::Bang as u8),
            Self::Clear => comp.push(WriteCodes::Clear as u8),
            Self::Tab(tab) => {
                tab.compile(comp, &ExpressionContext::Eval);
                comp.push(WriteCodes::Tab as u8);
            }
            Self::Expression(exp) => {
                exp.compile(comp, &ExpressionContext::Write);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    comp.push(WriteCodes::Expression as u8);
                }
            }
        }
    }
}

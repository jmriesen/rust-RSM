use ir::{Expression, commands::Write};

use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext};

impl Compile for Write {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        match self {
            Self::Bang => comp.push(ffi::CMWRTNL),
            Self::Clear => comp.push(ffi::CMWRTFF),
            Self::Tab(tab) => {
                tab.compile(comp, &ExpressionContext::Eval);
                comp.push(ffi::CMWRTAB);
            }
            Self::Expression(exp) => {
                exp.compile(comp, &ExpressionContext::Write);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    comp.push(ffi::CMWRTEX);
                }
            }
        }
    }
}

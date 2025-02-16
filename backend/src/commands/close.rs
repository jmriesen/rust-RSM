use ir::{Expression, commands::close::Close};

use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext};

impl Compile for Close {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        self.0.compile(comp, &ExpressionContext::Close);
        if !matches!(self.0, Expression::InderectExpression { .. }) {
            comp.push(ffi::CMCLOSE);
        }
    }
}

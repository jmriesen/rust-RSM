use ir::{Expression, commands::close::Close};

use crate::{Compile, byte_code::ByteCode, expression::ExpressionContext};

const CLOSE_CODE: u8 = 59;

impl Compile for Close {
    type Context = ();
    fn compile(&self, comp: &mut ByteCode, _: &()) {
        self.0.compile(comp, &ExpressionContext::Close);
        if !matches!(self.0, Expression::InderectExpression { .. }) {
            comp.push(CLOSE_CODE);
        }
    }
}

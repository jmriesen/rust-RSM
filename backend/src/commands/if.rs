use ir::commands::r#if::If;

use crate::{
    Compile,
    expression::ExpressionContext,
    runtime::{Decode, OpCode},
};

OpCode! {IfOp = 7}
OpCode! {ElseOp = 9}

impl Compile for If {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        self.0.compile(bite_code, &ExpressionContext::Eval);
        bite_code.push(IfOp.encode());
    }
}

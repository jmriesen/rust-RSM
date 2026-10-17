use ir::commands::r#if::If;

use crate::{Compile, expression::ExpressionContext, macros::OpCode, runtime::Decode};

OpCode! {IfOp = 7}
OpCode! {IfTest = 6}
OpCode! {ElseOp = 9}

impl Compile for If {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        if self.0.is_empty() {
            bite_code.push(IfTest.encode());
        } else {
            for arg in &self.0 {
                arg.compile(bite_code, &ExpressionContext::Eval);
                bite_code.push(IfOp.encode());
            }
        }
    }
}

use ir::commands::r#if::If;

use crate::{Compile, expression::ExpressionContext, macros::OpCodes, runtime::Decode};

OpCodes! {
    IfElse {
        IfNoArgs= 6,
        If = 7,
        Else= 9,
    }
}

impl Compile for If {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        if self.0.is_empty() {
            bite_code.push(IfElse::IfNoArgs as u8);
        } else {
            for arg in &self.0 {
                arg.compile(bite_code, &ExpressionContext::Eval);
                bite_code.push(IfElse::If as u8);
            }
        }
    }
}

use ir::commands::Quit;

use crate::{
    Compile,
    bite_code::BiteCode,
    expression::ExpressionContext,
    runtime::{Decode, OpCodes},
};

OpCodes! {QuitCodes {
    WithoutArg = 157,
    WithArg = 158,
}}
impl Compile for Quit {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match &self.0 {
            Some(arg) => {
                arg.compile(bite_code, &ExpressionContext::Eval);
                bite_code.push(QuitCodes::WithArg as u8);
            }
            None => bite_code.push(QuitCodes::WithoutArg as u8),
        }
    }
}

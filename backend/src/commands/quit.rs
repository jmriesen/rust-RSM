use ir::commands::Quit;

use crate::{
    Compile, byte_code::ByteCode, expression::ExpressionContext, macros::OpCodes, runtime::Decode,
};

OpCodes! {QuitCodes {
    WithoutArg = 157,
    WithArg = 158,
}}
impl Compile for Quit {
    type Context = ();
    fn compile(&self, bite_code: &mut ByteCode, _: &()) {
        match &self.0 {
            Some(arg) => {
                arg.compile(bite_code, &ExpressionContext::Eval);
                bite_code.push(QuitCodes::WithArg as u8);
            }
            None => bite_code.push(QuitCodes::WithoutArg as u8),
        }
    }
}

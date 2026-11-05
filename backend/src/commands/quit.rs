use ir::commands::Quit;

use crate::{
    Compile,
    byte_code::ByteCode,
    expression::ExpressionContext,
    macros::OpCodes,
    runtime::{LegacyEncoding, StackAssembally},
};

OpCodes! {QuitCodes {
    WithoutArg = 157,
    WithArg = 158,
}}
impl Compile for Quit {
    type Context = ();
    fn compile(&self, byte_code: &mut ByteCode, _: &()) {
        match &self.0 {
            Some(arg) => {
                arg.compile(byte_code, &ExpressionContext::Eval);
                byte_code.push_asm(StackAssembally::QuitCodes(QuitCodes::WithArg));
            }
            None => byte_code.push_asm(StackAssembally::QuitCodes(QuitCodes::WithoutArg)),
        }
    }
}

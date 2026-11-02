use ir::{Expression, commands::r#break::Break};

use crate::{
    Compile,
    byte_code::ByteCode,
    expression::ExpressionContext,
    runtime::{StackAssembally, TEMP},
};
enum BreakCode {
    ArgumentLess = 70,
    Arg = 71,
}

impl Compile for Break {
    type Context = ();
    fn compile(&self, byte_code: &mut ByteCode, _: &()) {
        match self {
            Self::ArgumentLess => {
                byte_code.push_asm(StackAssembally::TEMP(TEMP(BreakCode::ArgumentLess as u8)));
            }
            Self::Arg(args) => {
                for arg in args {
                    arg.compile(byte_code, &ExpressionContext::Eval);
                    if !matches!(arg, Expression::InderectExpression { .. }) {
                        byte_code.push_asm(StackAssembally::TEMP(TEMP(BreakCode::Arg as u8)));
                    }
                }
            }
        }
    }
}

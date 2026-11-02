use ir::{Expression, commands::Write};

use crate::{
    Compile, Compile2,
    byte_code::ByteCode,
    expression::ExpressionContext,
    macros::OpCodes,
    runtime::{Decode, StackAssembally},
};

OpCodes! {WriteCodes {
    Bang = 53,
    Clear = 54,
    Tab = 55,
    Expression = 56,
}}
impl Compile for Write {
    type Context = ();
    fn compile(&self, comp: &mut ByteCode, _: &()) {
        match self {
            Self::Bang => comp.push(WriteCodes::Bang as u8),
            Self::Clear => comp.push(WriteCodes::Clear as u8),
            Self::Tab(tab) => {
                tab.compile(comp, &ExpressionContext::Eval);
                comp.push(WriteCodes::Tab as u8);
            }
            Self::Expression(exp) => {
                exp.compile(comp, &ExpressionContext::Write);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    comp.push(WriteCodes::Expression as u8);
                }
            }
        }
    }
}

impl Compile2 for Write {
    type Context = ();

    fn compile_new(&self, byte_code: &mut Vec<StackAssembally>, _: &Self::Context) {
        use StackAssembally as Asm;
        match self {
            Self::Bang => byte_code.push(Asm::WriteCodes(WriteCodes::Bang)),
            Self::Clear => byte_code.push(Asm::WriteCodes(WriteCodes::Clear)),
            Self::Tab(tab) => {
                tab.compile_new(byte_code, &ExpressionContext::Eval);
                byte_code.push(Asm::WriteCodes(WriteCodes::Tab))
            }
            Self::Expression(exp) => {
                exp.compile_new(byte_code, &ExpressionContext::Write);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    byte_code.push(Asm::WriteCodes(WriteCodes::Expression));
                }
            }
        }
    }
}

use ir::{Expression, commands::Write};

use crate::{
    Compile,
    byte_code::ByteCode,
    expression::ExpressionContext,
    macros::OpCodes,
    runtime::{LegacyEncoding, StackAssembally},
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
        use StackAssembally as Asm;
        match self {
            Self::Bang => comp.push_asm(Asm::WriteCodes(WriteCodes::Bang)),
            Self::Clear => comp.push_asm(Asm::WriteCodes(WriteCodes::Clear)),
            Self::Tab(tab) => {
                tab.compile(comp, &ExpressionContext::Eval);
                comp.push_asm(Asm::WriteCodes(WriteCodes::Tab))
            }
            Self::Expression(exp) => {
                exp.compile(comp, &ExpressionContext::Write);
                if !matches!(exp, Expression::InderectExpression { .. }) {
                    comp.push_asm(Asm::WriteCodes(WriteCodes::Expression));
                }
            }
        }
    }
}

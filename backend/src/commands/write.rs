use ir::{Expression, commands::Write};

use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext, operators::Decode};

#[derive(Debug)]
pub enum WriteCodes {
    Bang = 53,
    Clear = 54,
    Tab = 55,
    Expression = 56,
}
impl Decode for WriteCodes {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        match code {
            53 => Some(Self::Bang),
            54 => Some(Self::Clear),
            55 => Some(Self::Tab),
            56 => Some(Self::Expression),
            _ => None,
        }
        .map(|x| (x, tail))
    }
}

impl Compile for Write {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
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

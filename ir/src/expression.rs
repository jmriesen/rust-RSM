use std::ops::Deref;

use derive_visitor::Event;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumberLiteral(pub value::Number);

impl derive_visitor::Drive for NumberLiteral {
    fn drive<V: derive_visitor::Visitor>(&self, visitor: &mut V) {
        visitor.visit(self, Event::Enter);
        visitor.visit(self, Event::Exit);
    }
}
impl Deref for NumberLiteral {
    type Target = value::Number;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringLiteral(pub value::Value);

impl derive_visitor::Drive for StringLiteral {
    fn drive<V: derive_visitor::Visitor>(&self, visitor: &mut V) {
        visitor.visit(self, Event::Enter);
        visitor.visit(self, Event::Exit);
    }
}

impl Deref for StringLiteral {
    type Target = value::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Expression {
    Number(Spanned<NumberLiteral>),
    String(Spanned<StringLiteral>),
    Variable(Spanned<Variable>),
    IntrinsicVar(IntrinsicVar),
    InderectExpression(Box<Self>),
    UnaryExpression {
        op_code: operators::Unary,
        expresstion: Box<Self>,
    },
    BinaryExpression {
        left: Box<Self>,
        op_code: operators::Binary,
        right: Box<Self>,
    },
    ExtrinsicFunction(ExtrinsicFunction),
    ExternalCalls {
        args: Vec<Self>,
        op_code: ExternalCalls,
    },
    IntrinsicFunction(Box<IntrinsicFunction>),
}

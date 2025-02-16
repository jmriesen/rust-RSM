use super::*;

#[derive(Clone)]
pub enum Expression {
    Number(value::Number),
    String(value::Value),
    Variable(Variable),
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

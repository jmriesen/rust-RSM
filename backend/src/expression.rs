use ::value::Value;
use extrinsic_function::ExtrinsicFunctionContext;
use ir::Expression;
use variable::VarContext;

use crate::bite_code::BiteCode;

use super::*;
#[derive(Clone, Copy)]
pub enum ExpressionContext {
    Write = ffi::INDWRIT as isize,
    Eval = ffi::INDEVAL as isize,
    Close = ffi::INDCLOS as isize,
}

impl Compile for Expression {
    type Context = ExpressionContext;
    fn compile(&self, comp: &mut BiteCode, context: &ExpressionContext) {
        use Expression as E;
        match self {
            E::Number(num) => Value::from(num.clone()).compile(comp, &()),
            E::String(value) => value.compile(comp, &()),
            E::Variable(var) => var.compile(comp, &VarContext::Eval),
            E::IntrinsicVar(var) => var.compile(comp, &()),
            E::InderectExpression(exp) => {
                exp.compile(comp, &ExpressionContext::Eval);
                comp.push(*context as u8);
            }
            E::UnaryExpression {
                op_code,
                expresstion,
            } => {
                expresstion.compile(comp, &ExpressionContext::Eval);
                op_code.compile(comp, &())
            }
            E::BinaryExpression {
                left,
                op_code,
                right,
            } => {
                left.compile(comp, &ExpressionContext::Eval);
                right.compile(comp, &ExpressionContext::Eval);
                op_code.compile(comp, &())
            }
            E::ExtrinsicFunction(function) => {
                function.compile(comp, &ExtrinsicFunctionContext::Eval)
            }
            E::ExternalCalls { args, op_code } => {
                args.compile(comp, &ExpressionContext::Eval);

                // All External Commands expect at least 2 arguments
                // If less then 2 arguments exist in the IR just pass the empty value.
                for _ in args.len()..2 {
                    ::value::EMPTY.compile(comp, &());
                }
                op_code.compile(comp, &())
            }
            E::IntrinsicFunction(intrinsic) => {
                intrinsic.compile(comp, &());
            }
        }
    }
}

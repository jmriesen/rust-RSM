use std::task::Context;

use ::value::Value;
use extrinsic_function::ExtrinsicFunctionContext;
use ir::Expression;
use variable::VarContext;

use crate::{byte_code::ByteCode, runtime::StackAssembally};

use super::*;
#[derive(Clone, Copy)]
pub enum ExpressionContext {
    Write = 195_isize,
    Eval = 65_isize,
    Close = 181_isize,
}

impl Compile for Expression {
    type Context = ExpressionContext;
    fn compile(&self, comp: &mut ByteCode, context: &ExpressionContext) {
        use Expression as E;
        use StackAssembally as Asm;
        match self {
            E::Number(num) => comp.push_asm(Asm::Value(num.inner.0.clone().into())),
            E::String(value) => comp.push_asm(Asm::Value(value.inner.0.clone())),
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
                // If less than 2 arguments exist in the IR just pass the empty value.
                for _ in args.len()..2 {
                    ::value::Value::empty().compile(comp, &());
                }
                op_code.compile(comp, &())
            }
            E::IntrinsicFunction(intrinsic) => {
                intrinsic.compile(comp, &());
            }
        }
    }
}

impl Compile2 for Expression {
    type Context = ExpressionContext;
    fn compile_new(&self, byte_code: &mut Vec<StackAssembally>, context: &ExpressionContext) {
        use Expression as E;
        use StackAssembally as Asm;
        match self {
            E::Number(num) => byte_code.push(Asm::Value(num.inner.0.clone().into())),
            E::String(value) => byte_code.push(Asm::Value(value.inner.0.clone())),
            /*
                        E::Variable(var) => var.compile(comp, &VarContext::Eval),
                        E::IntrinsicVar(var) => var.compile(comp, &()),
                        E::InderectExpression(exp) => {
                            exp.compile(comp, &ExpressionContext::Eval);
                            comp.push(*context as u8);
                        }
            */
            E::UnaryExpression {
                op_code,
                expresstion,
            } => {
                expresstion.compile_new(byte_code, &ExpressionContext::Eval);
                byte_code.push(Asm::Unary(op_code.clone()));
            }
            E::BinaryExpression {
                left,
                op_code,
                right,
            } => {
                left.compile_new(byte_code, &ExpressionContext::Eval);
                right.compile_new(byte_code, &ExpressionContext::Eval);
                byte_code.push(Asm::Binary(op_code.clone()));
            }
            /*
                        E::ExtrinsicFunction(function) => {
                            function.compile(comp, &ExtrinsicFunctionContext::Eval)
                        }
                        E::ExternalCalls { args, op_code } => {
                            args.compile(comp, &ExpressionContext::Eval);

                            // All External Commands expect at least 2 arguments
                            // If less than 2 arguments exist in the IR just pass the empty value.
                            for _ in args.len()..2 {
                                ::value::Value::empty().compile(comp, &());
                            }
                            op_code.compile(comp, &())
                        }
                        E::IntrinsicFunction(intrinsic) => {
                            intrinsic.compile(comp, &());
                        }
            */
            _ => {}
        }
    }
}

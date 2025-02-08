use crate::{
    expression::{insert_value, parse_string_litteral, ExpressionContext},
    localvar::VarContext,
    ExtrinsicFunctionContext,
};

use super::*;

#[derive(Clone)]
pub enum Expression {
    Number(value::Number),
    String(value::Value),
    Variable(Variable),
    IntrinsicVar(IntrinsicVar),
    Expression(Box<Self>),
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

impl Expression {
    pub fn new(sitter: &lang_model::Expression<'_>, source_code: &str) -> Self {
        let nested_new = |exp| Box::new(Self::new(&exp, source_code));
        use lang_model::ExpressionChildren as S;
        use std::str::FromStr;
        match sitter.children() {
            S::number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                Self::Number(value::Number::from_str(num).unwrap().into())
            }
            S::string(value) => {
                let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                Self::String(parse_string_litteral(value).unwrap())
            }
            S::Variable(var) => Self::Variable(super::Variable::new(&var, source_code)),
            S::IntrinsicVar(var) => Self::IntrinsicVar(super::IntrinsicVar::new(var)),
            S::Expression(exp) => Self::Expression(nested_new(exp)),
            S::InderectExpression(exp) => Self::InderectExpression(nested_new(exp.children())),
            S::UnaryExpression(unary_exp) => Self::UnaryExpression {
                op_code: operators::Unary::new(unary_exp.opp()),
                expresstion: nested_new(unary_exp.exp()),
            },
            S::BinaryExpression(bin_exp) => Self::BinaryExpression {
                left: nested_new(bin_exp.exp_left()),
                op_code: operators::Binary::new(bin_exp.opp()),
                right: nested_new(bin_exp.exp_right()),
            },
            S::PaternMatchExpression(pat_exp) => {
                use lang_model::PaternMatchExpressionExp_right as E;
                let right = match pat_exp.exp_right() {
                    E::Expression(exp) => nested_new(exp),
                    E::Patern(value) => {
                        let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                        Box::new(Self::String(value::Value::from_str(value).unwrap()))
                    }
                };
                Self::BinaryExpression {
                    left: nested_new(pat_exp.exp_left()),
                    op_code: operators::Binary::new_pattern(pat_exp.opp()),
                    right,
                }
            }
            S::ExtrinsicFunction(x) => {
                Self::ExtrinsicFunction(super::ExtrinsicFunction::new(&x, source_code))
            }
            S::XCall(xcall) => Self::ExternalCalls {
                args: xcall
                    .args()
                    .iter()
                    .map(|x| Self::new(x, source_code))
                    .collect(),
                op_code: ExternalCalls::new(xcall.code()),
            },
            S::IntrinsicFunction(intrinsic) => {
                Self::IntrinsicFunction(Box::new(IntrinsicFunction::new(&intrinsic, source_code)))
            }
        }
    }
}

pub fn compile(exp: &Expression, comp: &mut Vec<u8>, context: ExpressionContext) {
    use Expression as E;
    match exp {
        E::Number(num) => insert_value(comp, num.clone().into()),
        E::String(value) => insert_value(comp, value.clone()),
        E::Variable(var) => super::variable::compile(&var, comp, VarContext::Eval),
        E::IntrinsicVar(var) => comp.push(var.op_code()),
        E::Expression(exp) => compile(exp, comp, ExpressionContext::Eval),
        E::InderectExpression(exp) => {
            compile(exp, comp, ExpressionContext::Eval);
            comp.push(context as u8);
        }
        E::UnaryExpression {
            op_code,
            expresstion,
        } => {
            compile(expresstion, comp, ExpressionContext::Eval);
            comp.push(op_code.op_code());
        }
        E::BinaryExpression {
            left,
            op_code,
            right,
        } => {
            compile(left, comp, ExpressionContext::Eval);
            compile(right, comp, ExpressionContext::Eval);
            comp.push(op_code.op_code());
        }
        E::ExtrinsicFunction(x) => {
            super::extrinsic_function::compile(x, comp, ExtrinsicFunctionContext::Eval)
        }
        E::ExternalCalls { args, op_code } => {
            for arg in args {
                compile(arg, comp, ExpressionContext::Eval);
            }

            for _ in args.len()..2 {
                insert_value(comp, value::EMPTY.clone());
            }
            comp.push(op_code.op_code());
        }
        E::IntrinsicFunction(intrinsic) => {
            super::intrinsic_functions::compile(&intrinsic, comp);
        }
    }
}

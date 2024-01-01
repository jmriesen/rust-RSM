mod literals;
mod intrinsicFunction;
pub use literals::*;

use crate::models::*;

use crate::localvar::VarTypes;
use crate::ExtrinsicFunctionContext;

pub enum ExpressionContext {
    Write = crate::bindings::INDWRIT as isize,
    Eval = crate::bindings::INDEVAL as isize,
    Close = crate::bindings::INDCLOS as isize,
}

impl<'a> Expression<'a> {
    pub fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExpressionContext) {
        use crate::bindings::PARTAB;
        use ExpressionChildren::*;
        match self.children() {
            number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                ncopy(num, &mut PARTAB::default(), comp);
            }
            string(value) => {
                //TODO remove duplication.
                let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                compile_string_literal(value, comp);
            }
            Variable(var) => var.compile(source_code, comp, VarTypes::Eval),
            IntrinsicVar(var) => {
                comp.push(var.op_code());
            }
            Expression(exp) => exp.compile(source_code, comp, ExpressionContext::Eval),
            InderectExpression(exp) => {
                exp.children()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(context as u8);
            }
            UnaryExpression(unary_exp) => {
                unary_exp
                    .exp()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(unary_exp.opp().op_code());
            }
            BinaryExpression(bin_exp) => {
                bin_exp
                    .exp_left()
                    .compile(source_code, comp, ExpressionContext::Eval);
                bin_exp
                    .exp_right()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(bin_exp.opp().op_code());
            }
            PaternMatchExpression(pat_exp) => {
                pat_exp
                    .exp_left()
                    .compile(source_code, comp, ExpressionContext::Eval);
                use PaternMatchExpressionExp_right as E;
                match pat_exp.exp_right() {
                    E::Expression(exp) => {
                        exp.compile(source_code, comp, ExpressionContext::Eval);
                    }
                    E::Patern(value) => {
                        //TODO remove duplication
                        let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                        compile_string_literal(&format!("\"{}\"", value), comp);
                    }
                }
                comp.push(pat_exp.opp().op_code());
            }
            ExtrinsicFunction(x) => {
                x.compile(source_code, comp, ExtrinsicFunctionContext::Eval);
            }
            XCall(x) => {
                x.args()
                    .iter()
                    .for_each(|x| x.compile(source_code, comp, ExpressionContext::Eval));

                for _ in x.args().len()..2 {
                    compile_string_literal("\"\"", comp);
                }
                comp.push(x.code().op_code());
            }

            IntrinsicFunction(intrinsic) => {
                intrinsic.compile(source_code, comp);
            }
        }
    }

    pub fn is_inderect(&self) -> bool {
        matches!(
            self.children(),
            ExpressionChildren::InderectExpression(_)
        )
    }
}

#[cfg(test)]
pub mod test;

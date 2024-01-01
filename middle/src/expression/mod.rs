mod literals;
pub use literals::*;

use crate::models::*;

use crate::function::{reserve_jump, write_jump};
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
                use IntrinsicFunctionChildren::*;
                match &intrinsic.children() {
                    ExpFunctions(exp_fun) => {
                        use ExpFunctionsChildren::*;
                        let children = &exp_fun.children();
                        let (opcode, args) = match children {
                            View(x) => (crate::bindings::FUNV2 - 2, x.args()),
                            //TODO Text handling should be more detailed.
                            Text(x) => (crate::bindings::FUNT - 1, vec![x.args()]),
                            Translate(x) => (crate::bindings::FUNTR2 - 2, x.args()),
                            Find(x) => (crate::bindings::FUNF2 - 2, x.args()),
                            Fnumber(x) => (crate::bindings::FUNFN2 - 2, x.args()),
                            Random(x) => (crate::bindings::FUNR - 1, vec![x.args()]),
                            Reverse(x) => (crate::bindings::FUNRE - 1, vec![x.args()]),
                            Piece(x) => (crate::bindings::FUNP2 - 2, x.args()),
                            Justify(x) => (crate::bindings::FUNJ2 - 2, x.args()),
                            Extract(x) => (crate::bindings::FUNE1 - 1, x.args()),
                            Ascii(x) => (crate::bindings::FUNA1 - 1, x.args()),
                            Length(x) => (crate::bindings::FUNL1 - 1, x.args()),
                            Stack(x) => (crate::bindings::FUNST1 - 1, x.args()),
                            Char(x) => (crate::bindings::FUNC, x.args()),
                            //TODO handle select. It dose not work like the others.
                        };
                        let count = args
                            .iter()
                            .map(|x| x.compile(source_code, comp, ExpressionContext::Eval))
                            .count();
                        if opcode == crate::bindings::FUNC {
                            if count > 254 {
                                panic!("Char has too many args");
                            } else {
                                comp.push(opcode);
                                comp.push(count as u8);
                            }
                        } else {
                            comp.push(opcode + count as u8);
                        }
                    }
                    VarFunctions(exp_fun) => {
                        use VarFunctionsChildren::*;
                        let children = &exp_fun.children();
                        let (opcode, var, args) = match children {
                            Name(x) => (crate::bindings::FUNNA1 - 1, x.var(), x.args()),
                            Order(x) => (crate::bindings::FUNO1 - 1, x.var(), x.args()),
                            Query(x) => (crate::bindings::FUNQ1 - 1, x.var(), x.args()),
                            Increment(x) => (crate::bindings::FUNI1 - 1, x.var(), x.args()),
                            Get(x) => (crate::bindings::FUNG1 - 1, x.var(), x.args()),
                            //TODO Next is an allisas for Order + hard coded param.
                            Next(x) => (crate::bindings::FUNO2 - 1, x.var(), None),
                            Data(x) => (crate::bindings::FUND - 1, x.var(), None),
                            Qlength(x) => (crate::bindings::FUNQL - 1, x.var(), None),
                            Qsubscript(x) => (crate::bindings::FUNQS - 2, x.var(), Some(x.args())),
                        };
                        let var_type = match children {
                            Data(_) | Get(_) | Increment(_) => VarTypes::Build,
                            Name(_) | Order(_) | Query(_) | Next(_) => VarTypes::BuildNullable,
                            Qlength(_) | Qsubscript(_) => VarTypes::Eval,
                        };

                        var.compile(source_code, comp, var_type);
                        let count = args
                            .iter()
                            .map(|x| x.compile(source_code, comp, ExpressionContext::Eval))
                            .count();
                        if let Next(_) = children {
                            ncopy("2", &mut PARTAB::default(), comp);
                        }

                        comp.push(opcode + count as u8 + 1);
                    }
                    Select(select) => {
                        let jump_indexs = select
                            .children()
                            .array_chunks::<2>()
                            .map(|[condition, value]| {
                                condition.compile(source_code, comp, ExpressionContext::Eval);
                                comp.push(crate::bindings::JMP0);
                                let try_next = reserve_jump(comp);

                                value.compile(source_code, comp, ExpressionContext::Eval);
                                comp.push(crate::bindings::JMP);
                                let exit = reserve_jump(comp);

                                (try_next, exit)
                            })
                            .collect::<Vec<_>>();

                        comp.push(crate::bindings::OPERROR);
                        let errm4 = (-(crate::bindings::ERRM4 as i16)).to_le_bytes();
                        comp.extend_from_slice(&errm4);

                        for (try_next, exit) in jump_indexs {
                            write_jump(try_next, exit, comp);
                            write_jump(exit, comp.len(), comp);
                        }
                    }
                }
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

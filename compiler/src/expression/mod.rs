/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
mod intrinsic_function;
mod literals;
pub use literals::*;

use lang_model::*;

use crate::{localvar::VarTypes, ExtrinsicFunctionContext, OpCode};

pub enum ExpressionContext {
    Write = crate::bindings::INDWRIT as isize,
    Eval = crate::bindings::INDEVAL as isize,
    Close = crate::bindings::INDCLOS as isize,
}

use crate::Compileable;
impl<'a> Compileable for Expression<'a> {
    type Context = ExpressionContext;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExpressionContext) {
        use ExpressionChildren::*;
        match self.children() {
            number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                ncopy(num, comp);
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
                intrinsic.compile(source_code, comp, ());
            }
        }
    }
}

#[cfg(test)]
pub mod test;

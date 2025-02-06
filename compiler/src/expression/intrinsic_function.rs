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
use crate::{expression::insert_value, localvar::VarContext};
use lang_model::*;

use super::ExpressionContext;

trait ExpFunctionsExt<'a> {
    fn opcode(&self) -> u8;
    fn args(&self) -> Vec<Expression<'a>>;
}

trait VarFunctionsExt {
    fn components(&self) -> (u8, Variable, Option<Expression>);
    fn var_types(&self) -> VarContext;
}

impl<'a> VarFunctionsExt for VarFunctions<'a> {
    fn components(&self) -> (u8, Variable, Option<Expression>) {
        use VarFunctionsChildren::*;
        let children = &self.children();
        //TODO Split up accessors or convert into a new type
        match children {
            Name(x) => (ffi::FUNNA1 - 1, x.var(), x.args()),
            Order(x) => (ffi::FUNO1 - 1, x.var(), x.args()),
            Query(x) => (ffi::FUNQ1 - 1, x.var(), x.args()),
            Increment(x) => (ffi::FUNI1 - 1, x.var(), x.args()),
            Get(x) => (ffi::FUNG1 - 1, x.var(), x.args()),
            //TODO Next is an allisas for Order + hard coded param.
            Next(x) => (ffi::FUNO2 - 1, x.var(), None),
            Data(x) => (ffi::FUND - 1, x.var(), None),
            Qlength(x) => (ffi::FUNQL - 1, x.var(), None),
            Qsubscript(x) => (ffi::FUNQS - 2, x.var(), Some(x.args())),
        }
    }

    fn var_types(&self) -> VarContext {
        use VarFunctionsChildren::*;
        match self.children() {
            Data(_) | Get(_) | Increment(_) => VarContext::Build,
            Name(_) | Order(_) | Query(_) | Next(_) => VarContext::BuildNullable,
            Qlength(_) | Qsubscript(_) => VarContext::Eval,
        }
    }
}

use crate::Compileable;
impl<'a> Compileable for IntrinsicFunction<'a> {
    type Context = ();
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, _context: Self::Context) {
        use IntrinsicFunctionChildren::*;
        let fun = crate::ir::intrinsic_functions::IntrinsicFunction::new(self, source_code);
        if let Some(fun) = fun {
            crate::ir::intrinsic_functions::compile(&fun, source_code, comp);
        } else {
            //TODO Consider Reducing Duplication betwen ExpFunc and VarFunc
            match &self.children() {
                ExpFunctions(_exp_fun) => unreachable!(),
                VarFunctions(var_fun) => {
                    let (opcode, var, args) = var_fun.components();
                    var.compile(source_code, comp, var_fun.var_types());

                    if let Some(arg) = &args {
                        arg.compile(source_code, comp, ExpressionContext::Eval)
                    }

                    if matches!(var_fun.children(), VarFunctionsChildren::Next(_)) {
                        use std::str::FromStr;
                        insert_value(
                            comp,
                            value::Number::from_str("2")
                                .expect("2 can always be parsed")
                                .into(),
                        );
                    }

                    comp.push(opcode + args.iter().len() as u8 + 1);
                }
                Select(_select) => unreachable!(),
            }
        }
    }
}

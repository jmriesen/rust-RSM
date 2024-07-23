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
use crate::{
    function::{reserve_jump, write_jump},
    localvar::VarTypes,
};
use lang_model::*;

use super::{ncopy, ExpressionContext};

trait ExpFunctionsExt<'a> {
    fn opcode(&self) -> u8;
    fn args(&self) -> Vec<Expression<'a>>;
}

impl<'a> ExpFunctionsExt<'a> for ExpFunctions<'a> {
    fn opcode(&self) -> u8 {
        use ExpFunctionsChildren::*;
        match self.children() {
            View(_) => ffi::FUNV2 - 2,
            //TODO Text handling should be more detailed.
            Text(_) => ffi::FUNT - 1,
            Translate(_) => ffi::FUNTR2 - 2,
            Find(_) => ffi::FUNF2 - 2,
            Fnumber(_) => ffi::FUNFN2 - 2,
            Random(_) => ffi::FUNR - 1,
            Reverse(_) => ffi::FUNRE - 1,
            Piece(_) => ffi::FUNP2 - 2,
            Justify(_) => ffi::FUNJ2 - 2,
            Extract(_) => ffi::FUNE1 - 1,
            Ascii(_) => ffi::FUNA1 - 1,
            Length(_) => ffi::FUNL1 - 1,
            Stack(_) => ffi::FUNST1 - 1,
            Char(_) => ffi::FUNC,
        }
    }
    fn args(&self) -> Vec<Expression<'a>> {
        use ExpFunctionsChildren::*;
        match self.children() {
            View(x) => x.args(),
            //TODO Text handling should be more detailed.
            Text(x) => vec![x.args()],
            Translate(x) => x.args(),
            Find(x) => x.args(),
            Fnumber(x) => x.args(),
            Random(x) => vec![x.args()],
            Reverse(x) => vec![x.args()],
            Piece(x) => x.args(),
            Justify(x) => x.args(),
            Extract(x) => x.args(),
            Ascii(x) => x.args(),
            Length(x) => x.args(),
            Stack(x) => x.args(),
            Char(x) => x.args(),
        }
    }
}

trait VarFunctionsExt {
    fn components(&self) -> (u8, Variable, Option<Expression>);
    fn var_types(&self) -> VarTypes;
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

    fn var_types(&self) -> VarTypes {
        use VarFunctionsChildren::*;
        match self.children() {
            Data(_) | Get(_) | Increment(_) => VarTypes::Build,
            Name(_) | Order(_) | Query(_) | Next(_) => VarTypes::BuildNullable,
            Qlength(_) | Qsubscript(_) => VarTypes::Eval,
        }
    }
}

use crate::Compileable;
impl<'a> Compileable for IntrinsicFunction<'a> {
    type Context = ();
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, _context: Self::Context) {
        use IntrinsicFunctionChildren::*;

        //TODO Consider Reducing Duplication betwen ExpFunc and VarFunc
        match &self.children() {
            ExpFunctions(exp_fun) => {
                let args = exp_fun.args();

                for arg in &args {
                    arg.compile(source_code, comp, ExpressionContext::Eval)
                }

                if matches!(exp_fun.children(), ExpFunctionsChildren::Char(_)) {
                    if args.len() > 254 {
                        panic!("Char has too many args");
                    } else {
                        comp.push(exp_fun.opcode());
                        comp.push(args.len() as u8);
                    }
                } else {
                    comp.push(exp_fun.opcode() + args.len() as u8);
                }
            }
            VarFunctions(var_fun) => {
                let (opcode, var, args) = var_fun.components();
                var.compile(source_code, comp, var_fun.var_types());

                if let Some(arg) = &args {
                    arg.compile(source_code, comp, ExpressionContext::Eval)
                }

                if matches!(var_fun.children(), VarFunctionsChildren::Next(_)) {
                    ncopy("2", comp);
                }

                comp.push(opcode + args.iter().len() as u8 + 1);
            }
            Select(select) => {
                let jump_indexs = select
                    .children()
                    .array_chunks::<2>()
                    .map(|[condition, value]| {
                        condition.compile(source_code, comp, ExpressionContext::Eval);
                        comp.push(ffi::JMP0);
                        let try_next = reserve_jump(comp);

                        value.compile(source_code, comp, ExpressionContext::Eval);
                        comp.push(ffi::JMP);
                        let exit = reserve_jump(comp);

                        (try_next, exit)
                    })
                    .collect::<Vec<_>>();

                comp.push(ffi::OPERROR);
                let errm4 = (-(ffi::ERRM4 as i16)).to_le_bytes();
                comp.extend_from_slice(&errm4);

                for (try_next, exit) in jump_indexs {
                    write_jump(try_next, exit, comp);
                    write_jump(exit, comp.len(), comp);
                }
            }
        }
    }
}

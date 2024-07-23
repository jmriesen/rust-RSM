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
use crate::models::{Expression, VariableHeading};

use crate::Compileable;
use lang_model as models;

pub enum VarTypes {
    Eval = ffi::OPVAR as isize,
    Build = ffi::OPMVAR as isize,
    BuildNullable = ffi::OPMVARN as isize,
    For = ffi::CMFORSET as isize,
}

trait HeadingExt {
    fn op_code(&self) -> u8;
    fn union_length(&self) -> bool;
    fn args(&self) -> Vec<Expression>;
    fn is_indirect(&self) -> bool;
}

impl<'a> HeadingExt for Option<VariableHeading<'a>> {
    fn op_code(&self) -> u8 {
        use models::VariableHeading::*;
        (if let Some(heading) = self {
            match heading {
                NakedVariable(_) => ffi::TYPVARNAKED,
                IndirectVariable(_) => ffi::TYPVARIND,
                GlobalVariable(_) => ffi::TYPVARGBL,
                GlobalUciVariable(_) => ffi::TYPVARGBLUCI,
                GlobalUciEnvVariable(_) => ffi::TYPVARGBLUCIENV,
            }
        } else {
            ffi::TYPVARNAM
        }) as u8
    }
    fn union_length(&self) -> bool {
        use models::VariableHeading::*;
        matches!(self, Some(GlobalVariable(_)) | None)
    }
    fn args(&self) -> Vec<Expression> {
        use models::VariableHeading::*;
        //TODO in theory I should be able to remove all of these alocations.
        if let Some(heading) = self {
            match heading {
                NakedVariable(_) => vec![],
                GlobalVariable(_) => vec![],
                GlobalUciVariable(exp) => vec![exp.children()],
                GlobalUciEnvVariable(exps) => exps.children(),
                IndirectVariable(exp) => vec![exp.children()],
            }
        } else {
            vec![]
        }
    }
    fn is_indirect(&self) -> bool {
        use models::VariableHeading::*;
        matches!(self, Some(IndirectVariable(_)))
    }
}
impl<'a> Compileable for crate::models::Variable<'a> {
    type Context = VarTypes;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: VarTypes) {
        use crate::expression::ExpressionContext;
        let heading = self.heading();

        for arg in heading.args() {
            arg.compile(source_code, comp, ExpressionContext::Eval);
        }

        if heading.is_indirect() {
            comp.push(ffi::INDMVAR);
        }

        //NOTE c docs says subscripts heading,
        //but that is not what the code outputs
        for subscript in self.subs() {
            subscript.compile(source_code, comp, ExpressionContext::Eval);
        }

        comp.push(context as u8);
        comp.push(heading.op_code());
        //Consider requiting this so we only push opcode once.
        if heading.union_length() {
            *comp.last_mut().unwrap() |= self.subs().len() as u8;
        } else {
            comp.push(self.subs().len() as u8);
        }

        if let Some(name) = self.name() {
            //TODO abstract away.
            let name: ffi::VAR_U = name
                .node()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .try_into()
                .unwrap();
            comp.extend(name.as_array())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{test_compile_command, test_harness::test::compile_c};
    use rstest::rstest;
    #[rstest]
    #[case("SomeString")]
    #[case("^SomeString")]
    #[case("^[atom]varName")]
    #[case("^|atom|varName")]
    #[case("^[atom1,atom2]varName")]
    #[case("SomeString(sub1)")]
    #[case("^SomeString(sub1)")]
    #[case("^(sub1)")]
    #[case("^|atom|varName(sub1)")]
    #[case("^[atom1,atom2]varName(sub1)")]
    #[case("SomeString(sub1,sub2)")]
    #[case("^SomeString(sub1,sub2)")]
    #[case("^|atom|varName(sub1,sub2)")]
    #[case("^[atom1,atom2]varName(sub1,sub2)")]
    #[case("@atom@(sub1,sub2)")]
    #[case("@varName")]
    //TODO index
    fn parse_var(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, ffi::parse);

        assert_eq!(orignal, test_compile_command(&source_code));
    }
}

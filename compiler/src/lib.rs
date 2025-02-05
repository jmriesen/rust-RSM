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
#![feature(array_chunks)]

use ffi::{self as bindings};

mod command;
mod dollar;
mod expression;
mod function;
mod ir;
mod localvar;
mod routine;
mod test_harness;

use crate::function::{reserve_jump, write_jump};

#[deprecated]
mod models {
    //TODO clean up and remove glob import.
    #[deprecated]
    pub use lang_model::*;
}

use crate::localvar::VarContext;

trait Compileable {
    type Context;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, contex: Self::Context);
}

trait OpCode {
    fn op_code(&self) -> u8;
}

///Test harness that for commands
///
///Wraps the provided command in additional formatting before calling compile.
///This is needed since the only type tree sitter can parse is the source_file.
///
#[cfg(test)]
pub fn test_compile_command(source_code: &str) -> Vec<u8> {
    let source_code = source_code.replace('\n', "\n ");
    let source_code = &format!("tag {source_code}\n");
    compile(source_code)
}

pub fn compile(source_code: &str) -> Vec<u8> {
    use expression::ExpressionContext;
    let tree = models::create_tree(dbg!(source_code));
    let tree = models::type_tree(&tree, source_code).unwrap();

    let mut comp = vec![];
    let tags = tree.children();
    let block = tags[0].block().unwrap();
    for line in block.children() {
        let line = match line {
            models::BlockChildren::line(line) => line,
            models::BlockChildren::Block(_line) => continue,
        };
        let mut for_jumps = vec![];
        let commands = line.children();
        for command in &commands {
            let post_condition = match &command.children() {
                E::WriteCommand(command) => command.post_condition(),
                E::BrakeCommand(command) => command.post_condition(),
                E::CloseCommand(command) => command.post_condition(),
                E::DoCommand(command) => command.post_condition(),
                E::For(_) => None,
                E::ElseCommand(_) => None,
                E::NewCommand(_) => None,
                E::QUITCommand(_) => todo!(),
            }
            .map(|condition| {
                condition.compile(source_code, &mut comp, ExpressionContext::Eval);
                comp.push(bindings::JMP0);
                reserve_jump(&mut comp)
            });
            use crate::models::commandChildren as E;
            match command.children() {
                E::WriteCommand(command) => {
                    for arg in command.args() {
                        use crate::models::WriteArgChildren as E;
                        match arg.children() {
                            E::Bang(_) => comp.push(bindings::CMWRTNL),
                            E::Clear(_) => comp.push(bindings::CMWRTFF),
                            E::Tab(tab) => {
                                tab.children().compile(
                                    source_code,
                                    &mut comp,
                                    ExpressionContext::Eval,
                                );
                                comp.push(bindings::CMWRTAB);
                            }
                            E::Expression(exp) => {
                                exp.compile(source_code, &mut comp, ExpressionContext::Write);
                                if !exp.is_inderect() {
                                    comp.push(bindings::CMWRTEX);
                                }
                            }
                        }
                    }
                }
                E::BrakeCommand(command) => {
                    let children = command.args();
                    if children.is_empty() {
                        comp.push(bindings::OPBRK0);
                    } else {
                        for arg in children {
                            arg.compile(source_code, &mut comp, ExpressionContext::Eval);
                            if !arg.is_inderect() {
                                comp.push(bindings::OPBRKN);
                            }
                        }
                    }
                }

                E::CloseCommand(command) => {
                    //TODO this should not allow for one 0 children;
                    let children = command.args();
                    for arg in children {
                        arg.compile(source_code, &mut comp, ExpressionContext::Close);
                        if !arg.is_inderect() {
                            comp.push(bindings::CMCLOSE);
                        }
                    }
                }

                E::ElseCommand(_) => {
                    comp.push(bindings::OPELSE);
                }
                E::NewCommand(_command) => {}
                E::DoCommand(command) => {
                    if command.args().is_empty() {
                        comp.push(bindings::CMDON);
                    } else {
                        for arg in command.args() {
                            let post_condion = arg.post_condition().map(|x| {
                                x.compile(source_code, &mut comp, ExpressionContext::Eval);
                                comp.push(bindings::JMP0);
                                reserve_jump(&mut comp)
                            });

                            arg.function().compile(
                                source_code,
                                &mut comp,
                                ExtrinsicFunctionContext::Do,
                            );
                            if let Some(jump) = post_condion {
                                write_jump(jump, comp.len(), &mut comp)
                            }
                        }
                    }
                }
                E::For(command) => match command.variable() {
                    Some(var) => {
                        var.compile(source_code, &mut comp, VarContext::For);
                        let offset_for_code = reserve_jump(&mut comp);
                        let exit = reserve_jump(&mut comp);

                        for args in command.args() {
                            for exp in args.children() {
                                exp.compile(source_code, &mut comp, ExpressionContext::Eval);
                            }

                            comp.push(match args.children().len() {
                                1 => bindings::CMFOR1,
                                2 => bindings::CMFOR2,
                                3 => bindings::CMFOR3,
                                _ => unreachable!(),
                            });
                        }

                        write_jump(offset_for_code, comp.len(), &mut comp);
                        for_jumps.push((exit, false));
                    }
                    None => {
                        comp.push(bindings::CMFOR0);
                        for_jumps.push((reserve_jump(&mut comp), true));
                    }
                },
                E::QUITCommand(_) => todo!(),
            }
            //NOTE C bug?
            //if the command has arguments C dosent consume the trailing white space.
            //this causes extra end commands to be added.
            if !command.argumentless() {
                comp.push(bindings::OPENDC);
            }
            if let Some(jump) = post_condition {
                write_jump(jump, comp.len(), &mut comp)
            }
            //For commans only end at the end of the line.
            if !matches!(command.children(), E::For(_)) {
                comp.push(bindings::OPENDC);
            }
        }
        if let Some(command) = &commands.last() {
            if !command.argumentless() {
                //NOTE C bug?
                //The last command in a line is not subjected to the bug.
                //This removes the addtional OPENDC I added to compensate for the other bug.
                comp.pop();
            } else if matches!(
                command.children(),
                crate::models::commandChildren::DoCommand(_)
            ) {
                comp.push(bindings::OPENDC);
            }
        }
        for (exit, argless) in for_jumps.into_iter().rev() {
            comp.push(bindings::OPENDC);
            if argless {
                //jump back to start of for loop.
                comp.push(bindings::JMP);
                let jump = reserve_jump(&mut comp);
                write_jump(jump, exit, &mut comp);
            } else {
                comp.push(bindings::CMFOREND);
            }
            //jump out of for loop
            write_jump(exit, comp.len(), &mut comp);
            comp.push(bindings::OPNOP);

            comp.push(bindings::OPENDC);
        }
        comp.push(bindings::ENDLIN);
    }
    comp
}

enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

impl<'a> Compileable for crate::models::ExtrinsicFunction<'a> {
    type Context = ExtrinsicFunctionContext;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExtrinsicFunctionContext) {
        let function = ir::extrinsic_function::ExtrinsicFunction::new(self, source_code);
        ir::extrinsic_function::compile(&function, source_code, comp, context);
    }
}

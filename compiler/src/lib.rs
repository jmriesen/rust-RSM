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
use ir::commands::{close::Close, r#break::Break, r#do::Do, r#for::For, Write};

mod command;
mod dollar;
mod expression;
mod function;
mod ir;
mod localvar;
mod routine;
mod test_harness;

use crate::function::{reserve_jump, write_jump};

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
    let tree = lang_model::create_tree(dbg!(source_code));
    let tree = lang_model::type_tree(&tree, source_code).unwrap();

    let mut comp = vec![];
    let tags = tree.children();
    let block = tags[0].block().unwrap();
    for line in block.children() {
        let line = match line {
            lang_model::BlockChildren::line(line) => line,
            lang_model::BlockChildren::Block(_line) => continue,
        };
        let mut for_jumps = vec![];
        let commands = line.children();
        for command in &commands {
            let jump_past_command = match &command.children() {
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
                ir::Expression::new(&condition, source_code)
                    .compile(&mut comp, ExpressionContext::Eval);
                comp.push(bindings::JMP0);
                reserve_jump(&mut comp)
            });
            use lang_model::commandChildren as E;
            match command.children() {
                E::WriteCommand(command) => {
                    for arg in command.args().iter().map(|x| Write::new(x, source_code)) {
                        arg.compile(&mut comp)
                    }
                }
                E::BrakeCommand(command) => {
                    for command in Break::new(&command, source_code) {
                        command.compile(&mut comp);
                    }
                }

                E::CloseCommand(command) => {
                    for command in Close::new(&command, source_code) {
                        command.compile(&mut comp);
                    }
                }

                E::ElseCommand(_) => {
                    comp.push(ffi::OPELSE);
                }
                E::NewCommand(_command) => {}
                E::DoCommand(command) => {
                    for command in Do::new(&command, source_code) {
                        command.compile(&mut comp);
                    }
                }
                E::For(command) => {
                    let loop_exit = For::new(&command, source_code).compile(&mut comp);
                    for_jumps.push(loop_exit)
                }
                E::QUITCommand(_) => todo!(),
            }
            //NOTE C bug?
            //If the command has arguments C doesn't consume the trailing white space.
            //This causes extra end commands to be added.
            if !command.argumentless() {
                comp.push(bindings::OPENDC);
            }
            if let Some(jump_past) = jump_past_command {
                write_jump(jump_past, comp.len(), &mut comp)
            }
            //For commands only end at the end of the line.
            if !matches!(command.children(), E::For(_)) {
                comp.push(bindings::OPENDC);
            }
        }
        if let Some(command) = &commands.last() {
            if !command.argumentless() {
                //NOTE C bug?
                //The last command in a line is not subjected to the bug.
                //This removes the additional OPENDC I added to compensate for the other bug.
                comp.pop();
            } else if matches!(
                command.children(),
                lang_model::commandChildren::DoCommand(_)
            ) {
                comp.push(bindings::OPENDC);
            }
        }
        for for_end_processing in for_jumps.into_iter().rev() {
            for_end_processing.compile(&mut comp);
        }
        comp.push(bindings::ENDLIN);
    }
    comp
}

enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

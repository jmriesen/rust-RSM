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

use backend::{BiteCode, Compile};
use ir::commands::Command;
mod command;
mod dollar;
mod expression;
mod function;
mod localvar;
mod routine;

///Test harness that for commands
///
///Wraps the provided command in additional formatting before calling compile.
///This is needed since the only type tree sitter can parse is the source_file.
///
#[cfg(test)]
pub fn test_compile_command(source_code: &str) -> Vec<u8> {
    use frontend::wrap_command_in_routine;

    let commands = wrap_command_in_routine(source_code);
    compile_routine(commands)
}

pub fn parse_routine(source_code: &str) -> Vec<Vec<Command>> {
    frontend::parse_routine(source_code)
}

pub fn compile_routine(routine: frontend::Routine) -> Vec<u8> {
    let mut comp = BiteCode::new();
    for line in routine {
        line.compile(&mut comp, &());
        comp.push(ffi::ENDLIN);
    }
    comp.get_raw()
}

pub enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

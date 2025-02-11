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

use bite_code::BiteCode;
use ir::{commands::Command, Compile};

pub mod bite_code;
mod command;
mod dollar;
mod expression;
mod function;
mod ir;
mod localvar;
mod routine;
mod test_harness;

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
    let tree = lang_model::create_tree(dbg!(source_code));
    let tree = lang_model::type_tree(&tree, source_code).unwrap();

    let mut comp = BiteCode::new();
    let tags = tree.children();
    let block = tags[0].block().unwrap();
    for line in block.children() {
        let line = match line {
            lang_model::BlockChildren::line(line) => line,
            lang_model::BlockChildren::Block(_line) => continue,
        };
        let mut commands = vec![];
        let mut line_tail = line.children().into_iter();
        while let Some(command) = line_tail.next() {
            commands.push(Command::new(&command, source_code, &mut line_tail));
        }
        commands.compile(&mut comp, &());

        comp.push(ffi::ENDLIN);
    }
    comp.get_raw()
}

pub enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

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
#![no_main]

use std::mem::transmute;

use interpreter::{
    key::CArrayString,
    symbol_table::{Table, MVAR},
};
use libfuzzer_sys::{
    arbitrary::{Arbitrary, Result, Unstructured},
    fuzz_target,
};

/*
#[derive(Arbitrary, Debug)]
enum TableCommands {
    Set(MVAR, CArrayString),
    Get(MVAR),
    Kill(MVAR),
}

fuzz_target!(|commands: Vec<TableCommands>| {
    let mut table = Table::new();
    let mut c_table = interpreter::bindings::symbol_table::Table::new();

    for command in commands {
        match command {
            TableCommands::Set(var, val) => {
                table.set(&var, &val);
                c_table.set(&unsafe { transmute(var) }, &val.as_ref())
            }
            TableCommands::Get(var) => todo!(),
            TableCommands::Kill(var) => todo!(),
        }
    }
});
*/

fuzz_target!(|commands: &[u8]| {});

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

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};
use symbol_table::{
    key::{Key, KeyBound},
    Direction, MVar, Table,
};
use value::Value;

#[derive(Arbitrary, Debug, Deserialize, Serialize)]
enum TableCommands {
    Set(MVar<Key>, Value),
    Get(MVar<Key>),
    Kill(MVar<Key>),
    Data(MVar<Key>),
    Query(MVar<KeyBound>, Direction),
    Order(MVar<KeyBound>, Direction),
}

fuzz_target!(|commands: Vec<TableCommands>| -> () {
    {
        let mut table = Table::new();
        let mut c_table = ffi::symbol_table::Table::new();
        for command in commands.into_iter().take(100) {
            match command {
                TableCommands::Set(var, val) => {
                    assert_eq!(table.set(&var, &val), c_table.set(&var, val));
                }
                TableCommands::Get(var) => {
                    assert_eq!(table.get(&var), c_table.get(&var).as_ref());
                }
                TableCommands::Kill(var) => {
                    table.kill(&var);
                    c_table.kill(&var);
                }
                TableCommands::Data(var) => {
                    assert_eq!(table.data(&var), c_table.data(&var));
                }
                TableCommands::Query(var, direction) => {
                    assert_eq!(
                        table
                            .query(&var, direction)
                            .map(Value::from)
                            .unwrap_or_default(),
                        c_table.query(&var, direction)
                    );
                }
                TableCommands::Order(var, direction) => {
                    assert_eq!(
                        table
                            .order(&var, direction)
                            .map(Value::from)
                            .unwrap_or_default(),
                        c_table.order(&var, direction)
                    );
                }
            }
        }
    }
});

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
use interpreter::{
    key::{NonNullableKey, NullableKey},
    symbol_table::{Direction, MVar, Table},
    value::Value,
};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum TableCommands {
    Set(MVar<NonNullableKey>, Value),
    Get(MVar<NonNullableKey>),
    Kill(MVar<NonNullableKey>),
    Data(MVar<NonNullableKey>),
    Query(MVar<NullableKey>, Direction),
}

fuzz_target!(|commands: Vec<TableCommands>| {
    let mut table = Table::new();
    let mut c_table = interpreter::bindings::symbol_table::Table::new();

    for command in commands.into_iter().skip(4).take(100) {
        match command {
            TableCommands::Set(var, val) => {
                table.set(&var, &val).unwrap();
                c_table.set(&var.into_cmvar(), &val.into_cstring()).unwrap();
            }
            TableCommands::Get(var) => {
                let rust_val = table.get(&var);
                let c_val = c_table.get(&var.into_cmvar()).map(|x| Value::from(&x));
                assert_eq!(rust_val.ok_or(&-6), c_val.as_ref());
            }
            TableCommands::Kill(var) => {
                table.kill(&var);
                c_table.kill(&var.into_cmvar());
            }
            TableCommands::Data(var) => {
                let (decedents, data) = c_table.data(&var.clone().into_cmvar());
                assert_eq!(decedents, table.data(&var).has_descendants);
                assert_eq!(data, table.data(&var).has_value);
            }
            TableCommands::Query(var, direction) => {
                let rust_val = table.query(&var, direction);
                let c_val = c_table.query(&var.into_cmvar(), direction == Direction::Backward);
                assert_eq!(rust_val.as_bytes(), c_val);
            }
        }
    }
});

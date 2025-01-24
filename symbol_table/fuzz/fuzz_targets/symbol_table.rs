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
use ffi::IntoC;
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

fuzz_target!(|commands: Vec<TableCommands>| {
    let mut table = Table::new();
    /*
    let commands: Vec<TableCommands> =
        ron::from_str(&std::fs::read_to_string("test.ron").unwrap()).unwrap();
        let _ = std::fs::write(
            "test.ron",
            ron::ser::to_string_pretty(&commands, ron::ser::PrettyConfig::default()).unwrap(),
        );
    */
    let mut c_table = ffi::symbol_table::Table::new();

    for command in commands.into_iter().take(100) {
        match command {
            TableCommands::Set(var, val) => {
                assert_eq!(
                    table.set(&var, &val).map_err(|err| err.error_code() as i32),
                    c_table.set(&var.into_c(), &val.into_c())
                );
            }
            TableCommands::Get(var) => {
                let rust_val = table.get(&var);
                let c_val = c_table.get(&var.into_c()).map(|x| Value::from(&x));
                assert_eq!(rust_val.ok_or(&-(ffi::ERRM6 as i32)), c_val.as_ref());
            }
            TableCommands::Kill(var) => {
                table.kill(&var);
                c_table.kill(&var.into_c());
            }
            TableCommands::Data(var) => {
                let (decedents, data) = c_table.data(&var.clone().into_c());
                assert_eq!(decedents, table.data(&var).has_descendants);
                assert_eq!(data, table.data(&var).has_value);
            }
            TableCommands::Query(var, direction) => {
                let rust_val = table.query(&var, direction);
                let c_val = c_table.query(&var.into_c(), direction == Direction::Backward);
                assert_eq!(
                    rust_val.map(Value::from).unwrap_or_default(),
                    Value::try_from(&c_val[..])
                        .expect("The buffer comming from C should always fit")
                );
            }
            TableCommands::Order(var, direction) => {
                let rust_val = table.order(&var, direction);
                let c_val = c_table.order(&var.into_c(), direction == Direction::Backward);
                assert_eq!(
                    rust_val.map(Value::from).unwrap_or_default(),
                    Value::try_from(&c_val[..])
                        .expect("The buffer comming from C should always fit")
                );
            }
        }
    }
});

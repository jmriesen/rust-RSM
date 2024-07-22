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

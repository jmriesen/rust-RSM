#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::all)]
#[allow(dead_code)]
pub mod bindings;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod dollar;
mod eval;
mod ffi;
mod function;
mod localvar;
mod op_code;
mod routine;
mod var;
mod command;

#[derive(Parser)]
#[grammar = "command.pest"]
#[grammar = "routine.pest"]
#[grammar = "function.pest"]
#[grammar = "dollar.pest"]
#[grammar = "localvar.pest"]
#[grammar = "exp.pest"]
#[grammar = "opcode.pest"]
#[grammar = "pattern.pest"]
pub struct SyntaxParser;

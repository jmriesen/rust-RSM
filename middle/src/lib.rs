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

mod eval;
mod ffi;
mod localvar;
mod op_code;
mod dollar;
mod function;

#[derive(Parser)]
#[grammar = "function.pest"]
#[grammar = "dollar.pest"]
#[grammar = "localvar.pest"]
#[grammar = "exp.pest"]
#[grammar = "opcode.pest"]
#[grammar = "pattern.pest"]
pub struct SyntaxParser;

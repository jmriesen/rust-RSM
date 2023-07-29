pub extern crate pest;
#[macro_use]
extern crate pest_derive;

/*
Something about how the pest code is generated causes
The parsing code to be rebuild when the doctests run
even if there were no code changes.


I solved this by using conditional compilation, and moving
the pest code to its own crate.
 */
#[cfg(not(doctest))]
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

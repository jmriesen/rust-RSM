use std::iter::{from_fn, repeat_n};

use symbol_table::{Ident, MVar, SymbolTable, key::Path};
use value::Value;

use crate::runtime::{DoFrame, RuntimeError, program_counter::ProgramCounter};

pub struct PushFrameArgs<A: Iterator<Item = (MVar<Path>, Value)>> {
    pub arguments: A,
    pub line_level_zero: bool,
}

//
// Attempts to build an argument list based on a parameter list signature.
pub fn build_arguments(
    number_of_arguments_passed: u8,
    parameters: &Option<Vec<Ident>>,
    r_values: &mut Vec<Value>,
) -> Result<impl Iterator<Item = (MVar<Path>, Value)>, RuntimeError> {
    let parameter_list_defined = parameters.is_some();

    let arguments_passed = from_fn(|| r_values.pop()).take(number_of_arguments_passed as usize);

    let expected_numb_parameters = parameters.as_ref().map(|x| x.len()).unwrap_or(0);
    let num_missing_arguments =
        expected_numb_parameters as isize - number_of_arguments_passed as isize;

    if num_missing_arguments.is_negative() {
        //We supplied too many arguments
        for _ in arguments_passed {
            //Pulling off arguments that were passed.
        }
        if parameter_list_defined {
            Err(RuntimeError::ToManyArgumentsPassed)
        } else {
            Err(RuntimeError::NoParameterListDefined)
        }
    } else {
        let arguments = parameters
            .iter()
            .flatten()
            .rev()
            .map(|name| MVar::new(name.clone(), Path::new([]).unwrap()))
            //NOTE: we know that parameter list will always run out first due to number_of_undefined_arguments>=0.
            .zip(repeat_n(Value::empty(), num_missing_arguments as usize).chain(arguments_passed));
        Ok(arguments)
    }
}

pub fn push_do_frame<'a>(
    symbol_table: &mut SymbolTable,
    stack: &mut Vec<DoFrame<'a>>,
    pc: ProgramCounter<'a>,
    PushFrameArgs {
        arguments,
        line_level_zero,
    }: PushFrameArgs<impl Iterator<Item = (MVar<Path>, Value)>>,
) {
    let new_frame = DoFrame {
        pc,
        for_stack: vec![],
        test: stack.last().map_or(false, |x| x.test),
        line_level: if line_level_zero {
            0
        } else {
            stack.last().expect("The only time the stack is empty is if we are starting a new job, and in that case line_level_zero should have been set.").line_level + 1
        },
    };

    stack.push(new_frame);
    symbol_table.push_new_frame();

    for (var, value) in arguments {
        symbol_table.new_var([var.name.clone()]).unwrap();
        symbol_table.set(&var, &value).unwrap();
    }
}

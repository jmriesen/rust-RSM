use std::iter::{from_fn, repeat_n};

use symbol_table::{Ident, MVar, SymbolTable, key::Path};

use crate::runtime::{DoFrame, RuntimeError, StackValue, program_counter::ProgramCounter};

pub struct PushFrameArgs<A: Iterator<Item = (MVar<Path>, StackValue)>> {
    pub arguments: A,
    pub line_level_zero: bool,
}

//
// Attempts to build an argument list based on a parameter list signature.
pub fn build_arguments(
    number_of_arguments_passed: u8,
    parameters: &Option<Vec<Ident>>,
    stack: &mut super::Stack,
) -> Result<impl Iterator<Item = (MVar<Path>, StackValue)>, RuntimeError> {
    let parameter_list_defined = parameters.is_some();

    let arguments_passed = from_fn(|| Some(stack.pop())).take(number_of_arguments_passed as usize);

    let expected_numb_parameters = parameters.as_ref().map(|x| x.len()).unwrap_or(0);
    dbg!(expected_numb_parameters);
    let num_missing_arguments =
        expected_numb_parameters as isize - number_of_arguments_passed as isize;
    dbg!(expected_numb_parameters);
    dbg!(num_missing_arguments);

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
            .zip(
                repeat_n(StackValue::VarUndefined, num_missing_arguments as usize)
                    .chain(arguments_passed),
            );
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
    }: PushFrameArgs<impl Iterator<Item = (MVar<Path>, StackValue)>>,
) {
    let new_frame = DoFrame {
        pc,
        for_stack: vec![],
        test: stack.last().is_some_and(|x| x.test),
        line_level: if line_level_zero {
            0
        } else {
            stack.last().expect("The only time the stack is empty is if we are starting a new job, and in that case line_level_zero should have been set.").line_level + 1
        },
    };

    stack.push(new_frame);
    symbol_table.push_new_frame();

    for (var, value) in arguments {
        match value {
            StackValue::Value(value) => {
                symbol_table.new_var([var.name.clone()]).unwrap();
                symbol_table.set(&var, &value).unwrap();
            }
            StackValue::Var(mvar) => {
                assert_eq!(mvar.key, Path::empty());
                //TODO: There were more checks
                symbol_table.connect_and_new(mvar.name, var.name).unwrap();
            }
            StackValue::VarUndefined => {
                symbol_table.new_var([var.name.clone()]).unwrap();
            }
        }
    }
}

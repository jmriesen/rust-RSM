use std::iter::{from_fn, repeat_n};

use symbol_table::{Ident, MVar, SymbolTable, key::Path};

use crate::runtime::{
    DoFrame, MAX_DO_FRAMES, RuntimeError, StackValue, program_counter::ProgramCounter,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoFrameKind {
    //Run,
    //Job,
    Do,
    DoArgLess,
    Extrinsic,
    //Xecute,
}

// Attempts to build an argument list based on a parameter list signature.
pub fn build_arguments(
    number_of_arguments_passed: u8,
    parameters: &Option<Vec<Ident>>,
    stack: &mut super::Stack,
) -> Result<Vec<(Ident, StackValue)>, RuntimeError> {
    let parameter_list_defined = parameters.is_some();

    let arguments_passed = from_fn(|| Some(stack.pop())).take(number_of_arguments_passed as usize);

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
            .cloned()
            //NOTE: we know that parameter list will always run out first due to number_of_undefined_arguments>=0.
            .zip(
                repeat_n(StackValue::VarUndefined, num_missing_arguments as usize)
                    .chain(arguments_passed),
            );
        Ok(arguments.collect())
    }
}

pub fn push_do_frame<'a>(
    symbol_table: &mut SymbolTable,
    stack: &mut Vec<DoFrame<'a>>,
    pc: ProgramCounter<'a>,
    arguments: Vec<(Ident, StackValue)>,
    kind: DoFrameKind,
) -> Result<(), RuntimeError> {
    if kind == DoFrameKind::DoArgLess {
        // DoArgLess by their vary nature can't have arguments.
        // If this is ever tripped then there is a bug in the interpreter.
        assert_eq!(arguments, vec![])
    }
    if stack.len() >= MAX_DO_FRAMES {
        Err(RuntimeError::DoStackOverflow)
    } else {
        symbol_table.push_new_frame();
        stack.push(DoFrame {
            pc,
            for_stack: vec![],
            test: stack.last().is_some_and(|x| x.test),
            kind,
            line_level: 
            match kind{
                DoFrameKind::DoArgLess => {
                    stack.last().expect("The only time the stack is empty is if we are starting a new job, and in that case line_level_zero should have been set.").line_level + 1
                },
                DoFrameKind::Do | DoFrameKind::Extrinsic => 0,
            }
        });
        initialize_arguments(symbol_table, arguments)?;
        Ok(())
    }
    //NOTE: overflow check intentionally being done after loading all the arguments.
    // This is done to insure that all of the arguments are popped off the stack even in the
    // event of an error.
}

fn initialize_arguments(
    symbol_table: &mut SymbolTable,
    arguments: Vec<(Ident, StackValue)>,
) -> Result<(), RuntimeError> {
    Ok(for (var, value) in arguments {
        match value {
            StackValue::Value(value) => {
                symbol_table.new_var([var.clone()])?;
                symbol_table.set(&MVar::new(var, Path::empty()), &value)?;
            }
            StackValue::Var(mvar) => {
                can_be_passed_by_reference(&mvar)
                    .ok_or(RuntimeError::CommandSyntaxError)
                    .expect("Parser should have caught this error.");
                symbol_table.connect_and_new(mvar.name, var)?;
            }
            StackValue::VarUndefined => {
                symbol_table.new_var([var])?;
            }
        }
    })
}

/// Only "Normal" variables can be passed by reference.
//Exempting from mutation testing because this check "should always" return true.
//Anything that would fail this "should have been" caught by the parser.
#[cfg_attr(test, mutants::skip)]
fn can_be_passed_by_reference(var: &MVar<Path>) -> bool {
    var.key == Path::empty() && var.user_class_identification_is_local_var()
}

use std::iter::{from_fn, repeat_n};

use symbol_table::{Ident, MVar, SymbolTable, key::Path};
use thiserror::Error;
use value::Value;

use crate::{
    commands::quit::QuitCodes,
    extrinsic_function::ExtrinsicFunctionCall,
    routine::CompiledRoutine,
    runtime::{
        DoFrame, Job, MAX_DO_FRAMES, RuntimeError, Stack, StackValue,
        program_counter::ProgramCounter,
    },
};

#[derive(Error, PartialEq, Debug)]
pub enum QuitExitErrors {
    #[error("QUIT with an argument required")]
    ExpectedArgument,
    #[error("QUIT with an argument not allowed")]
    ExpectedArgumentLess,
}

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
fn build_arguments(
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
            line_level:match kind{
                DoFrameKind::DoArgLess => {
                    stack.last().expect("The only time the stack is empty is if we are starting a new job, and in that case line_level_zero should have been set.").line_level + 1
                },
                DoFrameKind::Do | DoFrameKind::Extrinsic => 0,
            }
        });
        initialize_arguments(symbol_table, arguments)?;
        Ok(())
    }
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

impl<'a> Job<'a> {
    pub(crate) fn quit(
        quit_codes: QuitCodes,
        stack: &mut Stack,
        symbol_table: &mut SymbolTable,
        call_stack: &mut Vec<DoFrame<'a>>,
    ) -> Result<(), QuitExitErrors> {
        let do_frame = call_stack.last_mut().unwrap();
        let kind = do_frame.kind;

        let result = if let Some(for_frame) = do_frame.for_stack.pop() {
            do_frame.pc.jump(for_frame.r#break);
            match quit_codes {
                QuitCodes::WithoutArg => Ok(()),
                QuitCodes::WithArg => Err(QuitExitErrors::ExpectedArgumentLess),
            }
        } else {
            call_stack.pop();
            symbol_table.pop_new_frame();
            match kind {
                DoFrameKind::Do | DoFrameKind::DoArgLess => match quit_codes {
                    QuitCodes::WithoutArg => Ok(()),
                    QuitCodes::WithArg => Err(QuitExitErrors::ExpectedArgumentLess),
                },
                DoFrameKind::Extrinsic => match quit_codes {
                    QuitCodes::WithoutArg => Err(QuitExitErrors::ExpectedArgument),
                    QuitCodes::WithArg => Ok(()),
                },
            }
        };
        //Adding or removing a value to fix the stack.
        match result {
            Ok(()) => {}
            Err(QuitExitErrors::ExpectedArgument) => {
                stack.push_val(Value::empty());
            }
            Err(QuitExitErrors::ExpectedArgumentLess) => {
                stack.pop_val();
            }
        }
        result
    }

    pub(crate) fn call_extrinsic_function(
        routine: &'a CompiledRoutine,
        symbol_table: &mut SymbolTable,
        call_stack: &mut Vec<DoFrame<'a>>,
        stack: &mut Stack,
        call: ExtrinsicFunctionCall,
    ) -> Result<(), RuntimeError> {
        let (pc, parameters) = routine
            .tag(call.tag)
            .ok_or(RuntimeError::CouldNotFindTagOrCouldNotFindRoutine)?;
        let arguments = build_arguments(call.number_of_arguments_passed, parameters, stack)?;
        push_do_frame(
            symbol_table,
            call_stack,
            pc,
            arguments,
            if call.return_value_expected {
                DoFrameKind::Extrinsic
            } else {
                DoFrameKind::Do
            },
        )
    }
}

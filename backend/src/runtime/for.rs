use super::{
    Job,
    program_counter::{Location, ProgramCounter},
};
use crate::{
    commands::r#for::{ForArgType, ForMetaData},
    runtime::RuntimeError,
};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

#[derive(Debug, PartialEq)]
pub struct Arguments {
    initial_value: Number,
    increment: Number,
    end_value: Option<Number>,
}

#[derive(Debug, PartialEq)]
pub struct ForArgFrame {
    pc: Location,
    var: MVar<Path>,
    args: Option<Arguments>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ForFrame {
    loop_body: Location,
    pub r#break: Location,
    pub args_frame: Option<ForArgFrame>, //TODO: Direction
}

impl<'a> Job<'a> {
    pub(crate) fn initialize_for_loop(
        for_stack: &mut Vec<ForFrame>,
        r_values: &mut Vec<Value>,
        for_meta_data: ForMetaData,
    ) {
        let ForMetaData {
            loop_body,
            r#break,
            args,
        } = for_meta_data;
        let args = args.map(|args| {
            let var = Self::build_var(r_values, args.loop_variable);
            ForArgFrame {
                pc: args.argument_pc,
                var,
                args: None,
            }
        });
        let new_frame = ForFrame {
            loop_body,
            r#break,
            args_frame: args,
        };

        for_stack.push(new_frame);
    }

    pub(crate) fn start_for_arg(
        frame: &mut ForFrame,
        r#type: ForArgType,
        symbol_table: &mut SymbolTable,
        r_values: &mut Vec<Value>,
        pc: &mut ProgramCounter<'_>,
    ) {
        let (end_value, increment, initial_value) = match r#type {
            ForArgType::One => (
                None,
                Number::one().clone(),
                Number::from(r_values.pop().unwrap()),
            ),
            ForArgType::Two => (
                None,
                Number::from(r_values.pop().unwrap()),
                Number::from(r_values.pop().unwrap()),
            ),
            ForArgType::Three => (
                Some(Number::from(r_values.pop().unwrap())),
                Number::from(r_values.pop().unwrap()),
                Number::from(r_values.pop().unwrap()),
            ),
        };
        let arguments = Arguments {
            initial_value: initial_value.clone(),
            increment,
            end_value,
        };
        {
            let args = &mut &mut frame
                .args_frame
                .as_mut()
                .expect("For type should only be created if we are in an argument-ed for command");

            args.args = Some(arguments);
            symbol_table.set(&args.var, &initial_value.into()).unwrap();
            args.pc = pc.current_location();
        }
        pc.jump(frame.loop_body);
    }

    pub(crate) fn loop_body_post_check(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        let for_frame = for_stack.last().unwrap();
        if let Some(args_frame) = &for_frame.args_frame {
            let var = &args_frame.var;
            let args = args_frame.args.as_ref().unwrap();

            match increment_var(symbol_table, var, args.increment.clone()) {
                Ok(new_value) => {
                    if past_end_value(args, new_value) {
                        if are_there_more_arguments(for_frame) {
                            pc.jump(args_frame.pc);
                        } else {
                            pc.jump(for_frame.r#break);
                            for_stack.pop();
                        }
                    } else {
                        pc.jump(for_frame.loop_body);
                    }
                }
                Err(err) => {
                    *error = Some(err);
                    // Error handling is not fully fleshed out yet.
                    pc.jump(for_frame.r#break);
                    for_stack.pop();
                }
            }
        } else {
            // No arguments just loop again.
            pc.jump(for_frame.loop_body);
        }
    }
}

fn past_end_value(args: &Arguments, new_value: Number) -> bool {
    if let Some(end_value) = &args.end_value {
        new_value > *end_value
    } else {
        //No end value to go past.
        false
    }
}
fn are_there_more_arguments(frame: &ForFrame) -> bool {
    if let Some(args) = &frame.args_frame {
        dbg!(args.pc) != dbg!(frame.loop_body)
    } else {
        //This is an argument-less loop.
        false
    }
}

fn increment_var(
    symbol_table: &mut SymbolTable,
    var: &MVar<Path>,
    increment: Number,
) -> Result<Number, RuntimeError> {
    if let Some(loop_var) = symbol_table.get(&var) {
        let new_value = Number::from(loop_var.clone()) + increment;
        symbol_table.set(&var, &new_value.clone().into()).unwrap();
        Ok(new_value)
    } else {
        Err(RuntimeError::UndefinedIndexVariable)
    }
}

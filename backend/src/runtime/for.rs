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
    pub args: Option<ForArgFrame>, //TODO: Direction
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
            args,
        };

        for_stack.push(new_frame);
    }

    pub(crate) fn start_for_arg(
        frame: &mut ForArgFrame,
        r#type: ForArgType,
        symbol_table: &mut SymbolTable,
        r_values: &mut Vec<Value>,
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

        frame.args = Some(arguments);
        symbol_table.set(&frame.var, &initial_value.into()).unwrap();
    }

    pub(crate) fn loop_body_post_check(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        let for_frame = for_stack.last().unwrap();
        if let Some(args) = &for_frame.args {
            let var = &args.var;
            let args = args.args.as_ref().unwrap();
            if let Some(loop_var) = symbol_table.get(&var) {
                //Handel increment.
                let next_loop_var = Number::from(loop_var.clone()) + args.increment.clone();

                symbol_table
                    .set(&var, &next_loop_var.clone().into())
                    .unwrap();
                //Handle condition.
                if let Some(end_value) = &args.end_value
                    && next_loop_var > *end_value
                {
                    pc.jump(for_frame.r#break);
                    for_stack.pop();
                } else {
                    pc.jump(for_frame.loop_body);
                }
            } else {
                *error = Some(RuntimeError::UndefinedIndexVariable);
                // Error handling is not fully fleshed out yet.
                pc.jump(for_frame.r#break);
                for_stack.pop();
            }
        } else {
            // No arguments just loop again.
            pc.jump(for_frame.loop_body);
        }
    }
}

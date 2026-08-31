use super::{
    Job,
    program_counter::{Location, ProgramCounter},
};
use crate::{commands::r#for::ForArgType, runtime::RuntimeError};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

#[derive(Debug, PartialEq)]
pub(crate) struct ForFrame {
    var: MVar<Path>,
    loop_body: Location,
    pub r#break: Location,
    start_value: Number,
    increment: Number,
    end_value: Option<Number>,
    //TODO: Direction
}

impl<'a> Job<'a> {
    /*
    pub(crate) fn init_for_loop(
        for_stack: &mut Vec<ForFrame>,
        r_values: &mut Vec<Value>,
        for_preamble: &mut Option<ForSet>,
        symbol_table: &mut SymbolTable,
        for_start: ForArgType,
    ) {
        let (end_value, increment, start_value) = match for_start {
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
        let ForSet {
            loop_variable,
            loop_body,
            r#break,
        } = for_preamble.take().expect("preamble must come before set");
        let var = Self::build_var(r_values, loop_variable);
        let new_frame = ForFrame {
            start_value,
            increment,
            end_value,
            var,
            loop_body,
            r#break,
        };
        symbol_table
            .set(&new_frame.var, &new_frame.start_value.clone().into())
            .unwrap();
        for_stack.push(new_frame);
    }
    */

    pub(crate) fn loop_body_post_check(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        let for_frame = for_stack.last().unwrap();
        if let Some(loop_var) = symbol_table.get(&for_frame.var) {
            //Handel increment.
            let next_loop_var = Number::from(loop_var.clone()) + for_frame.increment.clone();
            symbol_table
                .set(&for_frame.var, &next_loop_var.clone().into())
                .unwrap();
            //Handle condition.
            if let Some(end_value) = &for_frame.end_value
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
    }
}

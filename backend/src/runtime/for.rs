use super::{
    Job,
    program_counter::{Location, ProgramCounter},
};
use crate::commands::r#for::{ForSet, ForStart};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

#[derive(Debug, PartialEq)]
pub(crate) struct ForFrame {
    var: MVar<Path>,
    loop_body: Location,
    pub r#break: Location,
    start_value: Number,
    increment: Number,
    end_value: Number,
    //TODO: Direction
}

impl<'a> Job<'a> {
    pub(crate) fn init_for_loop(
        for_stack: &mut Vec<ForFrame>,
        r_values: &mut Vec<Value>,
        for_preample: &mut Option<ForSet>,
        symbole_table: &mut SymbolTable,
        for_start: ForStart,
    ) {
        let (end_value, increment, start_value) = match for_start {
            ForStart::One => todo!(),
            ForStart::Two => todo!(),
            ForStart::Three => (
                Number::from(r_values.pop().unwrap()),
                Number::from(r_values.pop().unwrap()),
                Number::from(r_values.pop().unwrap()),
            ),
        };
        let ForSet {
            loop_variable,
            loop_body,
            r#break,
        } = for_preample.take().expect("preamble must come before set");
        let var = Self::build_var(r_values, loop_variable);
        let new_frame = ForFrame {
            start_value,
            increment,
            end_value,
            var,
            loop_body,
            r#break,
        };
        symbole_table
            .set(&new_frame.var, &new_frame.start_value.clone().into())
            .unwrap();
        for_stack.push(new_frame);
    }

    pub(crate) fn handel_for_preamble(
        for_stack: &mut Vec<ForFrame>,
        symbole_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
    ) {
        let for_frame = for_stack.last().unwrap();
        let loop_var = symbole_table
            .get(&for_frame.var)
            .expect("Loop variable must exist otherwise this is a runtime error")
            .clone();
        let next_loop_var = Number::from(loop_var) + for_frame.increment.clone();
        symbole_table
            .set(&for_frame.var, &next_loop_var.clone().into())
            .unwrap();

        if next_loop_var <= for_frame.end_value {
            pc.jump(for_frame.loop_body);
        } else {
            pc.jump(for_frame.r#break);
            for_stack.pop();
        }
    }
}

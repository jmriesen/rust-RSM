use super::{
    Job,
    program_counter::{Location, ProgramCounter},
};
use crate::{
    commands::r#for::{ForMetaData, ForRangeType},
    runtime::RuntimeError,
};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

/// The range a for loop should iterator over.
/// NOTE: single for loop command can have multiple ranges.
#[derive(Debug, PartialEq)]
pub struct Range {
    initial_value: Number,
    increment: Number,
    end_value: Option<Number>,
}

#[derive(Debug)]
enum RangeCheck {
    InBounds,
    OutOfBounds,
}

impl Range {
    fn increment(
        &self,
        var: &MVar<Path>,
        symbol_table: &mut SymbolTable,
    ) -> Result<RangeCheck, RuntimeError> {
        if let Some(loop_var) = symbol_table.get(&var) {
            let new_value = Number::from(loop_var.clone()) + self.increment.clone();
            symbol_table.set(&var, &new_value.clone().into()).unwrap();
            Ok(self.in_bounds(new_value))
        } else {
            Err(RuntimeError::UndefinedIndexVariable)
        }
    }
    fn in_bounds(&self, new_value: Number) -> RangeCheck {
        if let Some(end_value) = &self.end_value {
            if new_value <= *end_value {
                RangeCheck::InBounds
            } else {
                RangeCheck::OutOfBounds
            }
        } else {
            RangeCheck::InBounds
        }
    }
}

/// State required to track For command iteration.
#[derive(Debug, PartialEq)]
pub struct ForArgFrame {
    /// Instructions for calculating/loading the next `Range` Range.
    pc: Location,
    /// The loop variable.
    var: MVar<Path>,
    /// The range we currently iterating though. (May not be loaded yet)
    current_range: Option<Range>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ForFrame {
    loop_body: Location,
    pub r#break: Location,
    pub args_frame: Option<ForArgFrame>, //TODO: Direction
}

impl ForFrame {
    fn more_ranges_to_load(&self) -> bool {
        if let Some(args) = &self.args_frame {
            // arguments are stored right before the loop body.
            // If the args pc has gotten to the loop body we have read everything.
            args.pc != self.loop_body
        } else {
            // This is an argument-less loop.
            false
        }
    }
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
                pc: args.range_pc,
                var,
                current_range: None,
            }
        });
        let new_frame = ForFrame {
            loop_body,
            r#break,
            args_frame: args,
        };

        for_stack.push(new_frame);
    }

    pub(crate) fn initialize_for_range(
        frame: &mut ForFrame,
        r#type: ForRangeType,
        symbol_table: &mut SymbolTable,
        r_values: &mut Vec<Value>,
        pc: &mut ProgramCounter<'_>,
    ) {
        let (end_value, increment, initial_value) = match r#type {
            ForRangeType::One => (
                None,
                Number::one().clone(),
                Number::from(r_values.pop().unwrap()),
            ),
            ForRangeType::Two => (
                None,
                Number::from(r_values.pop().unwrap()),
                Number::from(r_values.pop().unwrap()),
            ),
            ForRangeType::Three => (
                Some(Number::from(r_values.pop().unwrap())),
                Number::from(r_values.pop().unwrap()),
                Number::from(r_values.pop().unwrap()),
            ),
        };
        let args = frame
            .args_frame
            .as_mut()
            .expect("For type should only be created if we are in an argument-ed for command");

        args.current_range = Some(Range {
            initial_value: initial_value.clone(),
            increment,
            end_value,
        });
        args.pc = pc.current_location();

        symbol_table.set(&args.var, &initial_value.into()).unwrap();
        pc.jump(frame.loop_body);
    }

    pub(crate) fn loop_condition_check_slash_increment(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        let for_frame = for_stack.last().unwrap();
        if let Some(args) = &for_frame.args_frame {
            let var = &args.var;
            let current_range = args.current_range.as_ref().expect("We are currently at the end of a for loop that took an argument. Therefore some Range must have been loaded when we started this iteration.");

            match current_range.increment(var, symbol_table) {
                Ok(bounds_check) => match bounds_check {
                    RangeCheck::InBounds => pc.jump(for_frame.loop_body),
                    RangeCheck::OutOfBounds => {
                        if for_frame.more_ranges_to_load() {
                            pc.jump(args.pc)
                        } else {
                            pc.jump(for_frame.r#break);
                            for_stack.pop();
                        }
                    }
                },
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

use crate::{
    byte_code::Location,
    commands::r#for::{ForMetaData, ForRangeType},
    runtime::{Job, RuntimeError, program_counter::ProgramCounter},
};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

/// The range a for loop should iterator over.
/// NOTE: single for loop command can have multiple ranges.
///
// 1) Initial value
// 2) Increment value
// 2) End Value
//
// NOTE:
// If there is no increment we only run the loop body once.
// If there is increment but no end value we keep going forever.
type Range = (Number, Option<(Number, Option<Number>)>);

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
pub struct ForFrame {
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
        current_location: Location,
        for_stack: &mut Vec<ForFrame>,
        l_values: &mut Vec<MVar<Path>>,
        for_meta_data: ForMetaData,
    ) {
        let ForMetaData {
            loop_body,
            r#break,
            kind,
        } = for_meta_data;
        let args = match kind {
            super::ForStart::Arguments => Some(ForArgFrame {
                pc: current_location,
                var: l_values.pop().expect("a built value."),
                current_range: None,
            }),
            super::ForStart::ArgumentLess => None,
        };

        let new_frame = ForFrame {
            loop_body: current_location + loop_body,
            r#break: current_location + r#break,
            args_frame: args,
        };

        for_stack.push(new_frame);
    }

    pub(crate) fn initialize_for_range(
        for_stack: &mut Vec<ForFrame>,
        r#type: ForRangeType,
        symbol_table: &mut SymbolTable,
        r_values: &mut Vec<Value>,
        pc: &mut ProgramCounter<'_>,
    ) {
        let mut frame = for_stack.last_mut();
        let frame = frame.as_mut().unwrap();
        let new_range = match r#type {
            ForRangeType::One => (Number::from(r_values.pop().unwrap()), None),
            ForRangeType::Two => {
                let increment = Number::from(r_values.pop().unwrap());
                (
                    Number::from(r_values.pop().unwrap()),
                    Some((increment, None)),
                )
            }
            ForRangeType::Three => {
                let end = Number::from(r_values.pop().unwrap());
                let increment = Number::from(r_values.pop().unwrap());
                (
                    Number::from(r_values.pop().unwrap()),
                    Some((increment, Some(end))),
                )
            }
        };

        let args = frame
            .args_frame
            .as_mut()
            .expect("For type should only be created if we are in an argument-ed for command");

        symbol_table
            .set(&args.var, &new_range.0.clone().into())
            .unwrap();

        args.current_range = Some(new_range.clone());
        args.pc = pc.current_location();
        pc.jump(frame.loop_body);

        if let (inital, Some((increment, Some(bound)))) = new_range
            && ((!increment.is_negative() && inital > bound)
                || (increment.is_negative() && bound > inital))
        {
            //already out of bounds keep advancing to the next argument.
            if frame.more_ranges_to_load() {
                pc.jump(pc.current_location())
            } else {
                pc.jump(frame.r#break);
                for_stack.pop();
            }
        }
    }

    pub(crate) fn loop_condition_check_slash_increment(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        let for_frame = for_stack.last().unwrap();
        enum Behavior {
            Continue,
            Error,
            CheckForMoreArgs,
        }
        let behavior: Behavior = if let Some(args) = &for_frame.args_frame {
            let var = &args.var;
            let current_range = args.current_range.as_ref().expect("We are currently at the end of a for loop that took an argument. Therefore some Range must have been loaded when we started this iteration.");

            if let (_, Some((increment, end_condition))) = &current_range {
                if let Some(loop_var) = symbol_table.get(var) {
                    //Increment
                    let new_value = Number::from(loop_var.clone()) + increment.clone();

                    // Check bound
                    if let Some(bound) = end_condition
                        && (
                            // Change bounds direction based on sign of increment
                            (!increment.is_negative() && new_value > *bound)
                                || (increment.is_negative() && *bound > new_value)
                        )
                    {
                        Behavior::CheckForMoreArgs
                    } else {
                        //Only increment if we are continuing.
                        symbol_table.set(var, &new_value.into()).unwrap();
                        Behavior::Continue
                    }
                } else {
                    *error = Some(RuntimeError::UndefinedIndexVariable);
                    Behavior::Error
                }
            } else {
                Behavior::CheckForMoreArgs
            }
        } else {
            Behavior::Continue
        };

        match behavior {
            Behavior::Continue => pc.jump(for_frame.loop_body),
            Behavior::Error => {
                // Error handling is not fully fleshed out yet.
                pc.jump(for_frame.r#break);
                for_stack.pop();
            }
            Behavior::CheckForMoreArgs => {
                if for_frame.more_ranges_to_load() {
                    pc.jump(
                        for_frame
                            .args_frame
                            .as_ref()
                            .expect("Can only check for more args if there were args")
                            .pc,
                    )
                } else {
                    pc.jump(for_frame.r#break);
                    for_stack.pop();
                }
            }
        };
    }
}

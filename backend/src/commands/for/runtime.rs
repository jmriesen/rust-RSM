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

        args.current_range = Some(new_range.clone());
        args.pc = pc.current_location();

        //NOTE: We are setting the variable before we do bounds checking.
        // This is different then the increment behavior.
        // Yes this inconsistency is odd, but I have checked the reference implementation and this is expected
        // behavior.
        symbol_table
            .set(&args.var, &new_range.0.clone().into())
            .unwrap();

        if let (inital, Some((increment, bound))) = new_range
            && !in_bounds(&inital, &increment, &bound)
        {
            //Already out of bounds keep advancing to the next argument.
            Behavior::CheckForMoreArgs
        } else {
            Behavior::Continue
        }
        .apply(for_stack, pc);
    }

    pub(crate) fn loop_condition_check_slash_increment(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        let for_frame = for_stack.last().unwrap();
        let behavior: Behavior = if let Some(args) = &for_frame.args_frame {
            let var = &args.var;
            let current_range = args.current_range.as_ref().expect("We are currently at the end of a for loop that took an argument. Therefore some Range must have been loaded when we started this iteration.");

            // Handle increment and bounds checking.
            // I considered breaking this down into smaller peaces but could not find the right
            // abstraction.
            //
            // We only increment the loop variable iff (if and only if) the value is in bounds.
            // This makes it hard to de-tangle the logic cleanly.
            if let (_, Some((increment, end_condition))) = &current_range {
                if let Some(loop_var) = symbol_table.get(var) {
                    let new_value = Number::from(loop_var.clone()) + increment.clone();
                    if in_bounds(&new_value, increment, end_condition) {
                        //NOTE: setting var happens after bounds check.
                        //Yes this is a bit weird but is how mumps works.
                        symbol_table.set(var, &new_value.into()).unwrap();
                        Behavior::Continue
                    } else {
                        Behavior::CheckForMoreArgs
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

        behavior.apply(for_stack, pc);
    }
}
/// Represents the end states that we could arrive at after checking the loop condition.
enum Behavior {
    /// In bounds loop again.
    Continue,
    /// Out of bounds/There were no bounds.
    /// Check if there are more arguments.
    CheckForMoreArgs,
    /// Encounter an error bail from the loop.
    Error,
}

impl Behavior {
    fn apply(&self, for_stack: &mut Vec<ForFrame>, pc: &mut ProgramCounter<'_>) {
        let current_frame = for_stack
            .last_mut()
            .expect("should only be called if we are in a floor loop");

        match self {
            Behavior::Continue => pc.jump(current_frame.loop_body),
            Behavior::Error => {
                // Error handling is not fully fleshed out yet.
                pc.jump(current_frame.r#break);
                for_stack.pop();
            }
            Behavior::CheckForMoreArgs => {
                if current_frame.more_ranges_to_load() {
                    pc.jump(
                        current_frame
                            .args_frame
                            .as_ref()
                            .expect("There must have been args if we are trying to load **More** of them.")
                            .pc,
                    )
                } else {
                    pc.jump(current_frame.r#break);
                    for_stack.pop();
                }
            }
        };
    }
}

/// Check if value has passed the ending bound.
/// NOTE: This does not increment the test value.
/// Increment is only used here to determine if this is a lower or upper bound.
fn in_bounds(loop_var: &Number, increment: &Number, bound: &Option<Number>) -> bool {
    bound.as_ref().map_or(true, |bound| {
        // Change bounds direction based on sign of increment
        if increment.is_negative() {
            bound <= loop_var
        } else {
            loop_var <= bound
        }
    })
}

pub mod byte_code;

use std::fmt::Debug;

use ir::operators::{Binary, Unary};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

use crate::{
    commands::{
        r#for::{ForEnd, ForSet, ForStart},
        write::WriteCodes,
    },
    runtime::{
        byte_code::{AssemballyDecoder, ByteCode, Location},
        macros::StackAssembally,
    },
    variable::{BuildVarInstructions, LoadVar, StoreVar},
};
mod macros;

#[derive(Debug)]
struct ForFrame {
    var: MVar<Path>,
    loop_body: Location,
    r#break: Location,
    start_value: Number,
    increment: Number,
    end_value: Number,
    //TODO: Direction
}

#[derive(Default)]
pub struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    stack: Vec<value::Value>,
    //Temporarily store loop metadata
    //This is needed since for loops are encoded as
    //Metadata expression expression expression loop body
    //so we need a place to put the metadata while evaluating the expressions
    for_preample: Option<ForSet>,
    // Metadata for all for loops.
    for_stack: Vec<ForFrame>,
    symbole_table: SymbolTable,
}
// Partial (or whole) assembly instruction.
pub trait Decode: Sized {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self>;
}
pub trait Encode: Sized {
    fn encode(&self) -> u8;
}

pub(crate) use macros::{OpCode, OpCodes, OpCodesForeign};
OpCode! {EndLine=0}
OpCode! {EndCommand=4}
OpCode! {NoOpCode=179}

#[derive(Debug)]
pub struct TEMP(u8);
#[cfg_attr(test, mutants::skip)]
impl Decode for TEMP {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        let [code] = decoder.consume_n();
        //Always accept remove before production but helps during testing adding new types
        Some(Self(code))
    }
}

pub(crate) trait StackAssemballyTrait: Decode {}
StackAssembally! {
    LoadVar,
    StoreVar,
    Value,
    WriteCodes,
    Binary,
    Unary,
    EndLine,
    EndCommand,
    ForSet,
    ForStart,
    ForEnd,
    TEMP,
    NoOpCode,

}
/// Marks something as a whole assembly instruction

impl JobState {
    pub fn run_code(&mut self, byte_code: &[u8]) {
        let mut byte_code = ByteCode::new(byte_code);
        #[cfg(test)]
        dbg!(&byte_code);
        while !byte_code.end() {
            match byte_code.next() {
                StackAssembally::Value(value) => {
                    self.stack.push(value);
                }
                StackAssembally::WriteCodes(_write_codes) => {
                    let value = self.stack.pop().unwrap();
                    self.buffer
                        .push_str(&String::from_utf8(value.content().to_vec()).unwrap());
                }
                StackAssembally::Binary(op) => {
                    let second = self.stack.pop().unwrap();
                    let first = self.stack.pop().unwrap();
                    let result: Value = match op {
                        Binary::Add => (Number::from(first) + Number::from(second)).into(),
                        Binary::Sub => (Number::from(first) - Number::from(second)).into(),
                        _ => {
                            todo!()
                        }
                    };
                    self.stack.push(result);
                }
                StackAssembally::Unary(op) => match op {
                    Unary::Minus => {
                        let first = self.stack.pop().unwrap();
                        let mut first = Number::from(first);
                        first.negate();
                        self.stack.push(first.into());
                    }
                    Unary::Plus => todo!(),
                    Unary::Not => todo!(),
                },
                StackAssembally::EndLine(_) | StackAssembally::EndCommand(_) => {}
                StackAssembally::ForSet(for_set) => self.for_preample = Some(for_set),
                StackAssembally::ForStart(for_start) => {
                    let (end_value, increment, start_value) = match for_start {
                        ForStart::One => todo!(),
                        ForStart::Two => todo!(),
                        ForStart::Three => (
                            Number::from(self.stack.pop().unwrap()),
                            Number::from(self.stack.pop().unwrap()),
                            Number::from(self.stack.pop().unwrap()),
                        ),
                    };
                    let ForSet {
                        loop_variable,
                        loop_body,
                        r#break,
                    } = self
                        .for_preample
                        .take()
                        .expect("preamble must come before set");
                    let var = self.build_var(loop_variable);
                    let new_frame = ForFrame {
                        start_value,
                        increment,
                        end_value,
                        var,
                        loop_body: loop_body.0,
                        r#break: r#break.0,
                    };
                    self.symbole_table
                        .set(&new_frame.var, &new_frame.start_value.clone().into())
                        .unwrap();
                    self.for_stack.push(new_frame);
                }
                StackAssembally::ForEnd(_for_end) => {
                    let for_frame = self.for_stack.last().unwrap();
                    let loop_var = self.symbole_table.get(&for_frame.var).unwrap().clone();
                    let next_loop_var = Number::from(loop_var) + for_frame.increment.clone();
                    self.symbole_table
                        .set(&for_frame.var, &next_loop_var.clone().into())
                        .unwrap();

                    if &next_loop_var <= &for_frame.end_value {
                        byte_code.jump_absolute(for_frame.loop_body);
                    } else {
                        byte_code.jump_absolute(for_frame.r#break);
                        self.for_stack.pop();
                    }
                }
                StackAssembally::NoOpCode(_no_op_code) => {}
                StackAssembally::LoadVar(load_var) => {
                    let var = self.build_var(load_var.var);
                    let val = self.symbole_table.get(&var).cloned().unwrap_or_default();
                    self.stack.push(val);
                }
                StackAssembally::StoreVar(store_var) => {
                    let var = self.build_var(store_var.var);
                    let val = self.stack.pop().expect("Value to store on the stack");
                    self.symbole_table.set(&var, &val).unwrap();
                }
                StackAssembally::TEMP { .. } => {}
            }
        }
    }
    fn build_var(&mut self, var: BuildVarInstructions) -> MVar<Path> {
        let mut subscripts = vec![];
        for _ in 0..var.subscripts {
            subscripts.push(self.stack.pop().unwrap());
        }
        MVar::new(var.name, Path::new(subscripts.iter()).unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::{runtime::JobState, test::compile_routine};
    use frontend::wrap_command_in_routine;
    use rstest::rstest;

    fn run_code_check_output(source: &str, output: &str) {
        let mut job = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        job.run_code(&byte_code);
        assert_eq!(job.buffer, output);
    }

    #[test]
    fn write() {
        run_code_check_output("w 5", "5");
    }
    #[rstest]
    #[case("w 5+10", "15")]
    #[case("w 5-10", "-5")]
    #[case("w --10", "10")]
    #[case("w 10-(5+4)", "1")]
    #[case("w i", "")]
    fn basic_math(#[case] source: &str, #[case] output: &str) {
        run_code_check_output(source, output);
    }

    #[rstest]
    //#[case("w i", "")] -- undefined local variable
    #[case("s i=5 w i", "5")]
    #[case("s i(\"foo\")=5 w i(\"foo\")", "5")]
    #[case("s i(\"foo\")=1 s i(\"bar\")=2 w i(\"bar\"),i(\"foo\")", "21")]
    fn load_store(#[case] source: &str, #[case] output: &str) {
        run_code_check_output(source, output);
    }

    #[rstest]
    #[case("f i=1:1:5 w \"foo \"", "foo foo foo foo foo ")]
    #[case::range_is_inclusive("f i=1:2:11 w i,\" \"", "1 3 5 7 9 11 ")]
    #[case::nested_for_loops("f i=1:1:2 f j=1:1:3 w \"foo \"", "foo foo foo foo foo foo ")]
    #[case::loop_var_is_converted_into_a_number_right_away(
        "f i=\"foo\":1:5 w i,\"_\"",
        "0_1_2_3_4_5_"
    )]
    #[case::loop_arguments_are_evaluated_once_before_the_loop_starts(
        "s n=2 f i=0:n:8+n s n=4 w i,\" \"",
        "0 2 4 6 8 10 "
    )]
    #[case::interacting_with_variable_is_ok("f i=1:1:5 s i=10 w \"foo\"", "foo")]
    fn for_loops(#[case] source: &str, #[case] output: &str) {
        run_code_check_output(source, output);
    }

    #[rstest]
    #[case::killing_the_index_variable_is_an_error(
        "f i=1:1:5 w \"k\" k i,",
        "$ECODE=,M15,\nUndefined index variable"
    )]
    fn todo(#[case] _source: &str, #[case] _output: &str) {
        //These are tests that should pass, but don't currently work since they rely on functionally
        //that has not been implemented.
        //They should be moved to the for_loop once the required functionality has ben added
    }
}

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
    runtime::byte_code::{ByteCode, Location},
};
mod macros;

#[derive(Debug)]
struct ForFrame {
    var: MVar<Path>,
    loop_body: Location,
    break_jump: Location,
    start_value: Value,
    increment: Number,
    end_value: Number,
    //TODO: Direction
}

#[derive(Default)]
pub struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    address_stack: Vec<value::Value>,
    //Temporarily store loop metadata
    //This is needed since for loops are encoded as
    //Metadata expression expression expression loop body
    //so we need a place to put the metadata while evaluating the expressions
    for_preample: Option<(Location, ForSet)>,
    // Metadata for all for loops.
    for_stack: Vec<ForFrame>,
    symbole_table: SymbolTable,
}
// Partial (or whole) assembly instruction.
pub trait Decode: Sized {
    fn decode(bytes: &[u8]) -> Option<(Self, &[u8])>;
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
    fn decode(bytes: &[u8]) -> Option<(Self, &[u8])> {
        if let ([code], tail) = bytes.split_at(1) {
            //Always accept remove before production but helps during testing adding new types
            Some((Self(*code), tail))
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum StackAssembally {
    Literal(Value),
    WriteCode(WriteCodes),
    BinaryOpCode(Binary),
    UnaryOp(Unary),
    EndLine(EndLine),
    EndCommand(EndCommand),
    ForSet {
        start_address: Location,
        set: ForSet,
    },
    ForStart(ForStart),
    ForEnd(ForEnd),
    TEMP {
        _inner: TEMP,
    },
    NoOpCode(NoOpCode),
}
/// Marks something as a whole assembly instruction
pub(crate) trait StackAssemballyTrait: Decode {}
impl StackAssemballyTrait for Value {}
impl StackAssemballyTrait for WriteCodes {}
impl StackAssemballyTrait for Binary {}
impl StackAssemballyTrait for Unary {}
impl StackAssemballyTrait for EndLine {}
impl StackAssemballyTrait for EndCommand {}
impl StackAssemballyTrait for ForSet {}
impl StackAssemballyTrait for ForStart {}
impl StackAssemballyTrait for ForEnd {}
impl StackAssemballyTrait for NoOpCode {}
impl StackAssemballyTrait for TEMP {}

impl JobState {
    pub fn run_code(&mut self, byte_code: &[u8]) {
        let mut byte_code = ByteCode::new(byte_code);
        while !byte_code.end() {
            match byte_code.next() {
                StackAssembally::Literal(value) => {
                    self.address_stack.push(value);
                }
                StackAssembally::WriteCode(_write_codes) => {
                    let value = self.address_stack.pop().unwrap();
                    self.buffer
                        .push_str(&String::from_utf8(value.content().to_vec()).unwrap());
                }
                StackAssembally::BinaryOpCode(op) => {
                    let second = self.address_stack.pop().unwrap();
                    let first = self.address_stack.pop().unwrap();
                    let result: Value = match op {
                        Binary::Add => (Number::from(first) + Number::from(second)).into(),
                        Binary::Sub => (Number::from(first) - Number::from(second)).into(),
                        _ => {
                            todo!()
                        }
                    };
                    self.address_stack.push(result);
                }
                StackAssembally::UnaryOp(op) => match op {
                    Unary::Minus => {
                        let first = self.address_stack.pop().unwrap();
                        let mut first = Number::from(first);
                        first.negate();
                        self.address_stack.push(first.into());
                    }
                    Unary::Plus => todo!(),
                    Unary::Not => todo!(),
                },
                StackAssembally::EndLine(_) | StackAssembally::EndCommand(_) => {}
                StackAssembally::ForSet { start_address, set } => {
                    self.for_preample = Some((start_address, set))
                }
                StackAssembally::TEMP { .. } => {}
                StackAssembally::ForStart(for_start) => {
                    let (end_value, increment, start_value) = match for_start {
                        ForStart::One => todo!(),
                        ForStart::Two => todo!(),
                        ForStart::Three => (
                            Number::from(self.address_stack.pop().unwrap()),
                            Number::from(self.address_stack.pop().unwrap()),
                            self.address_stack.pop().unwrap(),
                        ),
                    };
                    let (start_location, for_set) = self
                        .for_preample
                        .take()
                        .expect("preamble must come before set");
                    let new_frame = ForFrame {
                        start_value,
                        increment,
                        end_value,
                        var: for_set.var,
                        loop_body: Location(start_location.0 + for_set.jump_to_content.0 as usize),
                        break_jump: Location(start_location.0 + for_set.break_jump.0 as usize),
                    };
                    self.symbole_table
                        .set(&new_frame.var, &new_frame.start_value)
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
                        byte_code.jump_absolute(for_frame.break_jump);
                        self.for_stack.pop();
                    }
                }
                StackAssembally::NoOpCode(_no_op_code) => {}
            }
        }
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
    fn basic_math(#[case] source: &str, #[case] output: &str) {
        run_code_check_output(source, output);
    }

    #[rstest]
    #[case("f i=1:1:5 w \"foo \"", "foo foo foo foo foo ")]
    #[case::nested_for_loops("f i=1:1:2 f j=1:1:3 w \"foo \"", "foo foo foo foo foo foo ")]
    fn for_loops(#[case] source: &str, #[case] output: &str) {
        run_code_check_output(source, output);
    }

    #[rstest]
    #[case::range_is_inclusive("f i=1:2:11 w i,\" \"", "1 3 5 7 9 11 ")]
    #[case::loop_arguments_are_evaluated_once_before_the_loop_starts(
        "s n=2 f i=0:n:8+n s n=4 w i,\" \"",
        "0 2 4 6 8 10"
    )]
    #[case::loop_var_is_converted_into_a_number_right_away(
        "f i=\"foo\":1:5 w i,\"_\"",
        "0_1_2_3_4_5_"
    )]
    #[case::killing_the_index_variable_is_an_error(
        "f i=1:1:5 w \"k\" k i,",
        "$ECODE=,M15,\nUndefined index variable"
    )]
    #[case::interacting_with_variable_is_ok("f i=1:1:5 s i=10 w \"foo\"", "foo")]
    fn todo(#[case] _source: &str, #[case] _output: &str) {
        //These are tests that should pass, but don't currently work since they rely on functionally
        //that has not been implemented.
        //They should be moved to the for_loop once the required functionality has ben added
    }
}

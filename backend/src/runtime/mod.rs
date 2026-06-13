mod byte_code;

use std::fmt::Debug;

use ir::operators::{Binary, Unary};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

use crate::{
    commands::{
        r#for::{ForEnd, ForSet, ForStart},
        write::WriteCodes,
    },
    runtime::byte_code::ByteCode,
};
mod macros;

#[derive(Debug)]
struct ForFrame {
    var: MVar<Path>,
    loop_body: usize,
    break_jump: usize,
    start_value: Value,
    increment: Number,
    end_value: Number,
    //TODO: Direction
}

#[derive(Default)]
struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    address_stack: Vec<value::Value>,
    //Read metadata for a for loop
    for_preample: Option<(usize, ForSet)>,
    for_stack: Vec<ForFrame>,
    symbole_table: SymbolTable,
}
pub trait Decode: Sized {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])>;
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
impl Decode for TEMP {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        //Always accept remove before production but helps during testing adding new types
        Some((Self(code), tail))
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
    ForSet { start_address: usize, set: ForSet },
    ForStart(ForStart),
    ForEnd(ForEnd),
    TEMP(TEMP),
    NoOpCode(NoOpCode),
}

#[allow(unused)]
fn run_code(job_state: &mut JobState, byte_code: &[u8]) {
    let mut byte_code = ByteCode::new(byte_code);
    while !byte_code.end() {
        match byte_code.next() {
            StackAssembally::Literal(value) => {
                job_state.address_stack.push(value);
            }
            StackAssembally::WriteCode(_write_codes) => {
                let value = job_state.address_stack.pop().unwrap();
                job_state
                    .buffer
                    .push_str(&String::from_utf8(value.content().to_vec()).unwrap());
            }
            StackAssembally::BinaryOpCode(op) => {
                let second = job_state.address_stack.pop().unwrap();
                let first = job_state.address_stack.pop().unwrap();
                let result: Value = match op {
                    Binary::Add => (Number::from(first) + Number::from(second)).into(),
                    Binary::Sub => (Number::from(first) - Number::from(second)).into(),
                    _ => {
                        todo!()
                    }
                };
                job_state.address_stack.push(result);
            }
            StackAssembally::UnaryOp(op) => match op {
                Unary::Minus => {
                    let first = job_state.address_stack.pop().unwrap();
                    let mut first = Number::from(first);
                    first.negate();
                    job_state.address_stack.push(first.into());
                }
                Unary::Plus => todo!(),
                Unary::Not => todo!(),
            },
            StackAssembally::EndLine(_) | StackAssembally::EndCommand(_) => {}
            StackAssembally::ForSet { start_address, set } => {
                job_state.for_preample = Some((start_address, set))
            }
            StackAssembally::TEMP(_) => {}
            StackAssembally::ForStart(for_start) => {
                let (end_value, increment, start_value) = match for_start {
                    ForStart::One => todo!(),
                    ForStart::Two => todo!(),
                    ForStart::Three => (
                        Number::from(job_state.address_stack.pop().unwrap()),
                        Number::from(job_state.address_stack.pop().unwrap()),
                        job_state.address_stack.pop().unwrap(),
                    ),
                };
                let (start_location, for_set) = job_state
                    .for_preample
                    .take()
                    .expect("preamble must come before set");
                let new_frame = ForFrame {
                    start_value,
                    increment,
                    end_value,
                    var: for_set.var,
                    loop_body: start_location + for_set.jump_to_content as usize,
                    break_jump: start_location + for_set.break_jump as usize,
                };
                job_state
                    .symbole_table
                    .set(&new_frame.var, &new_frame.start_value);
                job_state.for_stack.push(new_frame);
            }
            StackAssembally::ForEnd(for_end) => {
                let for_frame = job_state.for_stack.last().unwrap();
                //TODO : Check if this should be unwrap or default requires kill interaction to
                //test
                let loop_var = job_state.symbole_table.get(&for_frame.var).unwrap().clone();
                let next_loop_var = Number::from(loop_var) + for_frame.increment.clone();
                job_state
                    .symbole_table
                    .set(&for_frame.var, &next_loop_var.clone().into())
                    .unwrap();

                if &next_loop_var <= &for_frame.end_value {
                    byte_code.jump_absolute(for_frame.loop_body);
                } else {
                    byte_code.jump_absolute(for_frame.break_jump);
                    job_state.for_stack.pop();
                }
            }
            StackAssembally::NoOpCode(no_op_code) => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        runtime::{JobState, run_code},
        test::compile_routine,
    };
    use frontend::wrap_command_in_routine;
    use rstest::rstest;

    #[test]
    fn write() {
        let source = "w 5";
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, "5");
    }
    #[rstest]
    #[case("w 5+10", "15")]
    #[case("w 5-10", "-5")]
    #[case("w --10", "10")]
    #[case("w 10-(5+4)", "1")]
    #[case("f i=1:1:5 w \"foo \"", "foo foo foo foo foo ")]
    //NOTE: range is includes
    //#[case("f i=1:2:11 w i,\" \"", "1 3 5 7 9 11 ")]
    //NOTE: Loop arguments are evaluated once before the loop starts.
    //#[case("s n=2 f i=0:n:8+n s n=4 w i,\" \"", "0 2 4 6 8 10")] -- needs set + variable lookup
    //NOTE: `i` is converted into a number right away.
    //#[case("f i=\"foo\":1:5 w i,\"_\"", "0_1_2_3_4_5_")] --needs variable lookup
    //NOTE: Nested for loops
    #[case("f i=1:1:2 f j=1:1:3 w \"foo \"", "foo foo foo foo foo foo ")]
    fn basic_math(#[case] source: &str, #[case] output: &str) {
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, output);
    }
}

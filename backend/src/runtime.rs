use std::{fmt::Debug, ops::Range};

use ir::operators::{Binary, Unary};
use symbol_table::{MVar, SymbolTable, key::Path};
use value::{Number, Value};

use crate::{
    commands::{
        r#for::{ForEnd, ForSet, ForStart, NoOpCode},
        write::WriteCodes,
    },
    operators::Decode,
};

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

//What are the primitives of the stack based language?
#[derive(Debug)]
pub struct EndLine;
impl Decode for EndLine {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        if code == 0 { Some((Self, tail)) } else { None }
    }
}

#[derive(Debug)]
pub struct EndCommand;
impl Decode for EndCommand {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        if code == 4 { Some((Self, tail)) } else { None }
    }
}

#[derive(Debug)]
struct TEMP(u8);
impl Decode for TEMP {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        //Always accept remove before productino but helps durring testing adding new types
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

#[derive(Clone)]
struct ByteCode<'a> {
    source: &'a [u8],
    program_counter: usize,
}
impl<'a> Debug for ByteCode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ByteCode")
            .field("program_counter", &self.program_counter)
            .field("parsed", &self.dbg_helper())
            .finish()
    }
}
impl<'a> ByteCode<'a> {
    fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            program_counter: 0,
        }
    }
    fn try_decode<T: Decode>(&mut self) -> Option<T> {
        if let Some((value, tail)) = T::decode(
            self.source[self.program_counter],
            &self.source[self.program_counter + 1..],
        ) {
            self.program_counter = self.source.len() - tail.len();
            Some(value)
        } else {
            None
        }
    }
    fn next(&mut self) -> StackAssembally {
        //Starting with none to get nice vertical alignment
        //Trusting that the compiler will optimize it away.
        None.or_else(|| self.try_decode().map(StackAssembally::WriteCode))
            .or_else(|| self.try_decode().map(StackAssembally::Literal))
            .or_else(|| self.try_decode().map(StackAssembally::UnaryOp))
            .or_else(|| self.try_decode().map(StackAssembally::BinaryOpCode))
            .or_else(|| self.try_decode().map(StackAssembally::EndLine))
            .or_else(|| self.try_decode().map(StackAssembally::EndCommand))
            .or_else(|| {
                self.try_decode().map(|set| StackAssembally::ForSet {
                    start_address: self.program_counter,
                    set: set,
                })
            })
            .or_else(|| self.try_decode().map(StackAssembally::ForStart))
            .or_else(|| self.try_decode().map(StackAssembally::ForEnd))
            .or_else(|| self.try_decode().map(StackAssembally::NoOpCode))
            .or_else(|| self.try_decode().map(StackAssembally::TEMP))
            .expect("Provided source was invalid/corruped")
    }
    fn end(&self) -> bool {
        self.program_counter == self.source.len()
    }

    fn dbg_helper(&self) -> Vec<(bool, Range<usize>, StackAssembally, &'a [u8])> {
        let mut scrach = self.clone();
        scrach.program_counter = 0;
        let mut vec: Vec<(bool, Range<usize>, StackAssembally, &'a [u8])> = vec![];
        while !scrach.end() {
            let start = scrach.program_counter;
            let asm = scrach.next();
            let end = scrach.program_counter;
            vec.push((
                (start..end).contains(&self.program_counter),
                (start..end),
                asm,
                &scrach.source[start..end],
            ));
        }
        vec
    }

    fn jump_absolute(&mut self, location: usize) {
        self.program_counter = location
    }
    fn jump_relative(&mut self, location: i16) {
        self.program_counter = dbg!((self.program_counter as isize) + (location as isize)) as usize;
    }
}
#[allow(unused)]
fn run_code(job_state: &mut JobState, byte_code: &[u8]) {
    let mut byte_code = ByteCode::new(byte_code);
    dbg!(&byte_code);
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
                        //TODO: add test about if number conversion is automatic
                        Number::from(job_state.address_stack.pop().unwrap()),
                        Number::from(job_state.address_stack.pop().unwrap()),
                        job_state.address_stack.pop().unwrap(),
                        //TODO: check if the comparesent happens as a number or a value
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
                let i = job_state.symbole_table.get(&for_frame.var).unwrap();
                let next_i = Number::from(i.clone()) + for_frame.increment.clone();
                job_state
                    .symbole_table
                    .set(&for_frame.var, &next_i.clone().into())
                    .unwrap();

                if dbg!(next_i.clone()) <= dbg!(for_frame.end_value.clone()) {
                    byte_code.jump_absolute(for_frame.loop_body - 2);
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
    //#[case("f i=1:2:11 w i,\" \"", "1 3 5 7 9 11 ")] // Note range is includes
    //#[case("f i=1:1+5:5 w \"foo \"", "foo foo foo foo foo ")]
    fn basic_math(#[case] source: &str, #[case] output: &str) {
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, output);
    }
}

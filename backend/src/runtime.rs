use ir::operators::{Binary, Unary};
use value::{Number, Value};

use crate::{
    commands::{
        r#for::{ForEnd, ForSet, ForStart, NoOpCode},
        write::WriteCodes,
    },
    operators::Decode,
};

#[derive(Default)]
struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    address_stack: Vec<value::Value>,
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
    ForSet(ForSet),
    ForStart(ForStart),
    ForEnd(ForEnd),
    TEMP(TEMP),
    NoOpCode(NoOpCode),
}

#[derive(Debug)]
struct ByteCode<'a>(&'a [u8]);
impl<'a> ByteCode<'a> {
    fn try_decode<T: Decode>(&mut self) -> Option<T> {
        if let Some((value, tail)) = T::decode(self.0[0], &self.0[1..]) {
            self.0 = tail;
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
            .or_else(|| self.try_decode().map(StackAssembally::ForSet))
            .or_else(|| self.try_decode().map(StackAssembally::ForStart))
            .or_else(|| self.try_decode().map(StackAssembally::ForEnd))
            .or_else(|| self.try_decode().map(StackAssembally::NoOpCode))
            .or_else(|| self.try_decode().map(StackAssembally::TEMP))
            .expect("Provided source was invalid/corruped")
    }
    fn end(&self) -> bool {
        self.0.is_empty()
    }

    #[allow(unused)]
    fn print_all(mut self) {
        let mut vec = vec![];
        while !self.end() {
            vec.push(self.next());
        }
        dbg!(vec);
    }
}
#[allow(unused)]
fn run_code(job_state: &mut JobState, byte_code: &[u8]) {
    let mut byte_code = ByteCode(byte_code);
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
            StackAssembally::ForSet(for_command) => todo!(),
            StackAssembally::TEMP(_) => {}
            StackAssembally::ForStart(for_start) => todo!(),
            StackAssembally::ForEnd(for_start) => todo!(),
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
    #[case("f i=1:1+5:5 w \"foo \"", "foo foo foo foo foo ")]
    fn basic_math(#[case] source: &str, #[case] output: &str) {
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        super::ByteCode(&byte_code).print_all();
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, output);
    }
}

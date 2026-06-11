use ir::operators::Unary;
use value::{Number, Value};

use crate::{commands::write::WriteCodes, operators::decode, value::STRING_OP};

#[derive(Default)]
struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    address_stack: Vec<value::Value>,
}

//What are the primitives of the stack based language?

#[derive(Debug)]
enum StackAssembally {
    Literal(Value),
    WriteCode(WriteCodes),
    BinaryOpCodeAdd,
    UnaryOp(Unary),
    NoOp,
}

#[derive(Debug)]
struct ByteCode<'a>(&'a [u8]);
impl<'a> ByteCode<'a> {
    fn next(&mut self) -> StackAssembally {
        let code = self.0[0];
        self.0 = &self.0[1..];
        match code {
            x if x == WriteCodes::Expression as u8 => {
                StackAssembally::WriteCode(WriteCodes::Expression)
            }
            x if let Some((value, tail)) = Value::decode(x, self.0) => {
                self.0 = tail;
                StackAssembally::Literal(value)
            }
            x if let Some((opcode, tail)) = Unary::decode(x, self.0) => {
                self.0 = tail;
                StackAssembally::UnaryOp(opcode)
            }
            10 => StackAssembally::BinaryOpCodeAdd,
            x => {
                dbg!(x);
                StackAssembally::NoOp
            }
        }
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
        match dbg!(dbg!(&mut byte_code).next()) {
            StackAssembally::Literal(value) => {
                job_state.address_stack.push(value);
            }
            StackAssembally::WriteCode(_write_codes) => {
                let value = job_state.address_stack.pop().unwrap();
                job_state
                    .buffer
                    .push_str(&String::from_utf8(value.content().to_vec()).unwrap());
            }
            StackAssembally::BinaryOpCodeAdd => {
                let second = job_state.address_stack.pop().unwrap();
                let first = job_state.address_stack.pop().unwrap();
                job_state
                    .address_stack
                    .push(Value::from(Number::from(first) + Number::from(second)));
            }
            StackAssembally::NoOp => {}
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
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        runtime::{ByteCode, JobState, run_code},
        test::compile_routine,
    };
    use frontend::wrap_command_in_routine;

    use super::StackAssembally;

    #[test]
    fn write() {
        let source = "w 5";
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, "5");
    }
    #[test]
    fn add() {
        let source = "w 5+10";
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, "15");
    }
    #[test]
    fn unary_minues() {
        let source = "w --10";
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        ByteCode(&byte_code).print_all();
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, "10");
    }
}

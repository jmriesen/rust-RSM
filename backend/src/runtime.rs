use ir::operators::{Binary, Unary};
use value::{Number, Value};

use crate::{commands::write::WriteCodes, operators::Decode, value::STRING_OP};

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
    BinaryOpCode(Binary),
    UnaryOp(Unary),
    NoOp,
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
            .or_else(|| {
                // Pulling off whatever is at the front and treating it as a no-op.
                // This is here so that the while let Some( ) = self.next loops are
                // guaranteed to make progress.
                // TODO: Remove once all cases are covered.
                let code = self.0[0];
                self.0 = &self.0[1..];
                dbg!(code);
                Some(StackAssembally::NoOp)
            })
            .expect("due to the last noop clause this value will always be some.")
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
    fn sub() {
        let source = "w 5-10";
        let mut job_state = JobState::default();
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);
        run_code(&mut job_state, &byte_code);
        assert_eq!(job_state.buffer, "-5");
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

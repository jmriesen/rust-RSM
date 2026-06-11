use value::Value;

use crate::{commands::write::WriteCodes, value::STRING_OP};

#[derive(Default)]
struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    address_stack: Vec<value::Value>,
}

//What are the primitives of the stack based language?

#[derive(Debug)]
enum StackLangItem {
    Value(Value),
    WriteCode(WriteCodes),
    BinaryOpCodeAdd,
    NoOp,
}

#[derive(Debug)]
struct ByteCode<'a>(&'a [u8]);
impl<'a> ByteCode<'a> {
    fn next(&mut self) -> StackLangItem {
        let code = self.0[0];
        self.0 = &self.0[1..];
        match code {
            x if x == WriteCodes::Expression as u8 => {
                StackLangItem::WriteCode(WriteCodes::Expression)
            }
            x if x == STRING_OP => {
                let (value, tail) = Value::from_bytes(self.0);
                self.0 = &tail[1..];
                StackLangItem::Value(value)
            }
            10 => StackLangItem::BinaryOpCodeAdd,
            x => {
                dbg!(x);
                StackLangItem::NoOp
            }
        }
    }
    fn end(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use frontend::wrap_command_in_routine;
    use value::{Number, Value};

    use crate::{
        runtime::{ByteCode, JobState},
        test::compile_routine,
    };

    use super::StackLangItem;
    fn run_code(job_state: &mut JobState, byte_code: &[u8]) {
        let mut byte_code = ByteCode(byte_code);
        while !byte_code.end() {
            match dbg!(byte_code.next()) {
                StackLangItem::Value(value) => {
                    job_state.address_stack.push(value);
                }
                StackLangItem::WriteCode(_write_codes) => {
                    let value = job_state.address_stack.pop().unwrap();
                    job_state
                        .buffer
                        .push_str(&String::from_utf8(value.content().to_vec()).unwrap());
                }
                StackLangItem::BinaryOpCodeAdd => {
                    let second = job_state.address_stack.pop().unwrap();
                    let first = job_state.address_stack.pop().unwrap();
                    job_state
                        .address_stack
                        .push(Value::from(Number::from(first) + Number::from(second)));
                }
                StackLangItem::NoOp => {}
            }
        }
    }

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
}

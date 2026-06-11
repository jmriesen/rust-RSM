#[derive(Default)]
struct JobState {
    //Replace with a proper output device later.
    buffer: String,
    address_stack: Vec<value::Value>,
}
#[cfg(test)]
mod test {
    use frontend::wrap_command_in_routine;
    use value::{Number, Value};

    use crate::{
        commands::write::WriteCodes, runtime::JobState, test::compile_routine, value::STRING_OP,
    };
    fn run_code(job_state: &mut JobState, mut byte_code: &[u8]) {
        while !dbg!(byte_code).is_empty() {
            let code = byte_code[0];
            byte_code = &byte_code[1..];
            match dbg!(code) {
                x if x == WriteCodes::Expression as u8 => {
                    let value = job_state.address_stack.pop().unwrap();
                    job_state
                        .buffer
                        .push_str(&String::from_utf8(value.content().to_vec()).unwrap());
                }
                x if x == STRING_OP => {
                    let (value, tail) = Value::from_bytes(byte_code);
                    job_state.address_stack.push(value);
                    byte_code = &tail[1..];
                }
                10 /*add op code*/ =>{
                    let second = job_state.address_stack.pop().unwrap();
                    let first= job_state.address_stack.pop().unwrap();
                    job_state.address_stack.push(Value::from(Number::from(first) +Number::from(second)));
                }
                x => {
                    dbg!(x);
                }
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

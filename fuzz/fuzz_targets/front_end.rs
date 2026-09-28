#![no_main]

use backend::compile_routine;
use frontend::parse_routine_print_errors;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|source_code: &str| {
    if let Ok(commands) = parse_routine_print_errors(source_code) {
        compile_routine(commands);
    }
});

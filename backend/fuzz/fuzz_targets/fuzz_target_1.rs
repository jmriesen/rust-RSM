#![no_main]

use libfuzzer_sys::fuzz_target;
use backend::compile_routine;
use frontend::parse_routine;

fuzz_target!(|source_code: &str| {
        if let Ok(commands) = parse_routine(source_code){
        compile_routine(commands);
    }

});

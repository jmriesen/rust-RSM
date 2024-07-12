#![no_main]

use interpreter::key::{a_b_testing::build, CArrayString};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|string: CArrayString| {
    build(&string);
});

#![no_main]

use interpreter::key::{a_b_testing::extract, CArrayString};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|string: CArrayString| {
    let _ = extract(string);
});

#![no_main]

use interpreter::key::{a_b_testing::string_key, CArrayString};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|keys: Vec<CArrayString>| {
    if keys.is_empty() {
    } else {
        let _ = string_key(&keys);
    }
});

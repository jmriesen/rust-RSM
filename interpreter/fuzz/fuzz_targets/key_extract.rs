#![no_main]

use interpreter::key::{a_b_testing, key_build, key_extract, CArrayString};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|string: CArrayString| {

    let _ = a_b_testing::extract(string.clone());
    if let Ok(key) = key_build(&string){
        let extracted = key_extract(&key);
        let contents = string.content();

        //NOTE for some reason negative numbers with a trailing decimal point
        //Are treated as numbers rather then strings
        //Therefore they are not the same when extracted.
        //TODO this could be a bug in the C code.
        if contents.starts_with(b"-")
            && contents.ends_with(b".")
            && contents[1..contents.len()-1].iter().all(|x| b'0'<= *x && *x<= b'9')
            //NOTE if there are leading zeros we DO treat it as a string.
            && contents[1] !=  b'0'
        {
            assert_eq!(&contents[..contents.len()-1],&extracted[..])
        }else{
            assert_eq!(&contents[..],&extracted[..])
        }
    }
});

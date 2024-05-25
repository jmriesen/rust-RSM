#![no_main]

use std::ptr::{from_mut, from_ref};

use interpreter::{key::key_extract, UTIL_Key_Build, UTIL_Key_Extract, CSTRING, MAX_STR_LEN};
use libfuzzer_sys::fuzz_target;

fn create_cstring(data:&[u8])->CSTRING{
    let mut buf = [0; MAX_STR_LEN as usize + 1];
    buf[..data.len()].copy_from_slice(data);
    CSTRING {
        len: data.len() as u16,
        buf,
    }
}

fuzz_target!(|data:&[u8]|{
    let string = create_cstring(data);
    let mut input_buffer = [0; MAX_STR_LEN as usize + 1];
    let _ = unsafe { UTIL_Key_Build(from_ref(&string).cast_mut(), input_buffer.as_mut_ptr()) };
    let mut output_buffer = [0; MAX_STR_LEN as usize + 1];
    let mut cnt = 0;
    //TODO figure out what temp correspond to.
    let len = unsafe {
        UTIL_Key_Extract(
            input_buffer.as_mut_ptr(),
            output_buffer.as_mut_ptr(),
            from_mut(&mut cnt),
        )
    };

    let output = key_extract(&input_buffer);
    assert_eq!(output[..], output_buffer[..len as usize]);
});

#![no_main]

use std::ptr::from_ref;

use interpreter::{
    key::{key_build, KeyError}, UTIL_Key_Build, CSTRING, ERRMLAST, ERRZ1, ERRZ5, MAX_STR_LEN
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut buf = [0; MAX_STR_LEN as usize + 1];
    buf[..data.len()].copy_from_slice(data);
    let string = CSTRING {
        len: data.len() as u16,
        buf,
    };
    let key = key_build(&string);

    let mut buffer = [0; MAX_STR_LEN as usize + 1];

    let len = unsafe { UTIL_Key_Build(from_ref(&string).cast_mut(), buffer.as_mut_ptr()) };
    match key {
        Ok(key) => {
            assert_eq!(key[..], buffer[..len as usize]);
            //The c code should never write past key's len, but this is good to check.
            assert!(buffer[len as usize..].iter().all(|x| *x == 0));
        }
        Err(KeyError::InputToLarge) => {
            assert_eq!(-len, (ERRZ1 + ERRMLAST) as i16)
        },
        Err(KeyError::ContainsNull) =>{
            assert_eq!(-len, (ERRZ5 + ERRMLAST) as i16)
        }
    }
});

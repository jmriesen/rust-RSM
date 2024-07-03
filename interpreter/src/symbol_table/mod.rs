//TODO remove once this module is actually being used.
#![allow(dead_code)]
use std::{array::from_fn, mem::transmute, ptr::null_mut};

#[allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
mod c_code {
    use ffi::CSTRING;
    use ffi::VAR_U;
    use std::sync::Mutex;
    pub static lock: Mutex<()> = Mutex::new(());
    include!(concat!(env!("OUT_DIR"), "/symbol_table_c.rs"));
}

//TODO remove and replace with derive once type move over to Rust
mod hash;
mod manual;

use c_code::Table;
type Tab = c_code::symtab_struct;

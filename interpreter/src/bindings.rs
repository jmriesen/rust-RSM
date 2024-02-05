#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::all)]
//bingen dose not seem to handle size of properly
pub const MAX_MAP_SIZE: u32 =
    (MAX_DATABASE_BLKS / 8 + (std::mem::size_of::<label_block>() as u32)) / 1024 + 1;
pub const IDX_START: u16 = (std::mem::size_of::<DB_Block>() as u16) / 2;
pub use rsm::bindings::*;
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

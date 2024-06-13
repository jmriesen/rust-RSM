#![feature(int_roundings)]
#![feature(let_chains)]
#![feature(slice_from_ptr_range)]
#![warn(clippy::all, clippy::pedantic)]
#![feature(array_windows)]
#![feature(slice_split_once)]
//TODO remove these allows.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc
)]
#[allow(clippy::all, unused)]
pub mod bindings;
pub mod create;
#[allow(unused)]
pub mod run;
mod shared_seg;
pub mod start;
mod symbol_table;
pub mod units;
pub mod util;
pub mod var_u;
pub use bindings::*;
mod buildmvar;

#[cfg(test)]
pub mod test_helper;

pub mod key;

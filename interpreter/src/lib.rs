#![feature(int_roundings)]
#![feature(let_chains)]
#![feature(slice_from_ptr_range)]
#![warn(clippy::all, clippy::pedantic)]
#![feature(array_windows)]
//TODO remove these allows.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc
)]
mod shared_seg;
#[allow(clippy::all, unused)]
pub mod bindings;
pub mod create;
#[allow(unused)]
pub mod run;
pub mod start;
pub mod units;
pub mod util;
pub mod var_u;
pub use bindings::*;
mod buildmvar;

#[cfg(test)]
pub mod test_helper;

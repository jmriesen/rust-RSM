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
#[allow(clippy::all, unused)]
pub mod bindings;
pub mod create;

pub mod alloc;
pub mod lock_tab;
pub mod global_buf;
#[allow(unused)]
pub mod run;
pub mod start;
pub mod sys_tab;
pub mod units;
pub mod util;
pub mod var_u;
pub mod vol_def;
pub use bindings::*;

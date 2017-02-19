//! A fast, simple, single-pass command line utility that securely wipes files
//! by zeroing them out. `zero` is non-recursive and non-verbose by default.
//! Inputs must be explicitly provided or no overwrites will occur.
extern crate getopts;
#[macro_use]
pub mod error;
pub mod core;
pub mod opts;
pub mod text;

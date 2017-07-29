//! Zero is a fast, simple, single-pass command line utility that securely
//! erases files by zero-filling them. It's non-recursive and non-verbose by
//! default, and always prompts the user before descending into absolute paths.
extern crate getopts;
#[macro_use]
pub mod macros;
pub mod core;
pub mod opts;
pub mod status;
pub mod text;

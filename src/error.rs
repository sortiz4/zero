//! Defines common error codes (`E`) and error messages (`M`).

/// Successful execution.
pub const ESUCCESS: i32 = 0x00;
/// Invalid usage.
pub const EUSAGE: i32 = 0x01;

/// The file or directory cannot be accessed.
pub const MACCESS: &str = "cannot access";
/// The file or directory cannot be found.
pub const MNOTFOUND: &str = "cannot not be found";
/// A usage error where conflicting options are present.
pub const MCONFLICT: &str = "conflicting options";
/// The program cannot read from the standard input.
pub const MSTDINERR: &str = "cannot read from stdin";

#[macro_export]
macro_rules! eprint {
    ($fmt:expr) => {{
        use std::io::{self, Write};
        write!(&mut io::stderr(), $fmt).unwrap();
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        use std::io::{self, Write};
        write!(&mut io::stderr(), $fmt, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}

/// Prepends the program name to the given message.
#[macro_export]
macro_rules! sformat {
    ($fmt:expr) => (format!(concat!("{}: ", $fmt), text::NAME));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!("{}: ", $fmt), text::NAME, $($arg)*));
}

/// Writes a formatted system message to the standard error.
#[macro_export]
macro_rules! sprint {
    ($fmt:expr) => (eprint!("{}", sformat!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (eprint!("{}", sformat!($fmt, $($arg)*)));
}

/// Writes a formatted system message to the standard error with a new line.
#[macro_export]
macro_rules! sprintln {
    () => (sprint!("\n"));
    ($fmt:expr) => (sprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (sprint!(concat!($fmt, "\n"), $($arg)*));
}

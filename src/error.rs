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

/// Prepends the program name to the given message.
#[macro_export]
macro_rules! format_sys {
    ($fmt:expr) => (format!(concat!("{}: ", $fmt), text::NAME));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!("{}: ", $fmt), text::NAME, $($arg)*));
}

/// Writes a formatted system message to the standard error.
#[macro_export]
macro_rules! sys {
    ($fmt:expr) => (write!(&mut ::std::io::stderr(), "{}", format_sys!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (write!(&mut ::std::io::stderr(), "{}", format_sys!($fmt, $($arg)*)));
}

/// Writes a formatted system message to the standard error with a new line.
#[macro_export]
macro_rules! sysln {
    ($fmt:expr) => (sys!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (sys!(concat!($fmt, "\n"), $($arg)*));
}

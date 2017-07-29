/// Prepends the program name to the given message.
#[macro_export]
macro_rules! formats {
    ($fmt:expr) => (format!(concat!("{}: ", $fmt), text::NAME));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!("{}: ", $fmt), text::NAME, $($arg)*));
}

/// Writes a formatted system message to the standard error.
#[macro_export]
macro_rules! sprint {
    ($fmt:expr) => (eprint!("{}", formats!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (eprint!("{}", formats!($fmt, $($arg)*)));
}

/// Writes a formatted system message to the standard error with a new line.
#[macro_export]
macro_rules! sprintln {
    () => (sprint!("\n"));
    ($fmt:expr) => (sprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (sprint!(concat!($fmt, "\n"), $($arg)*));
}

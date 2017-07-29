//! Defines common exit codes (`E`) and error messages (`M`).

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

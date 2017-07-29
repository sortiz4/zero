//! Defines common string fragments.

/// The program name.
pub const NAME: &str = "zero";
/// The program version.
pub const VERSION: &str = "1.0";
/// A generic help message.
pub const HELP: &str = "Try 'zero --help' for more information.";
/// A brief description of the program and its usage.
pub const USAGE: &str = concat![
    "Securely erase files (single-pass).\n\n",
    "Usage:\n", "    zero [OPTIONS] [PATHS]",
];

/// A descriptor that maps to the `Auth::Absolute` context.
pub const ABSOLUTE: &str = "is absolute";
/// A descriptor that maps to the `Auth::Interactive` context.
pub const INTERACTIVE: &str = "will be overwritten";

/// A descriptor that maps to the 'dry-run' option.
pub const DRYRUN: &str = "byte(s) will be overwritten";
/// A descriptor that maps to the 'verbose' option.
pub const OVERWRITE: &str = "byte(s) overwritten";

/// A continue prompt.
pub const CONTINUE: &str = "- continue? [y/n]";
/// Indicates an action has been skipped.
pub const SKIP: &str = "skipped";
/// Represents a 'yes' response.
pub const YES: &str = "y";
/// Represents a 'no' response.
pub const NO: &str = "n";

//! Defines common string fragments.

/// The program name.
pub const NAME: &'static str = "zero";
/// The program version.
pub const VERSION: &'static str = "1.0";
/// A generic help message.
pub const HELP: &'static str = "\
Try 'zero --help' for more information.";
/// A brief description of the program and its usage.
pub const USAGE: &'static str = "\
Securely erase files (single-pass).
\nUsage:\n    zero [OPTIONS] [DIRS|FILES]";

/// A descriptor that maps to the `Auth::Absolute` context.
pub const ABSOLUTE: &'static str = "is absolute";
/// A descriptor that maps to the `Auth::Interactive` context.
pub const INTERACTIVE: &'static str = "will be overwritten";

/// A descriptor that maps to the 'dry-run' option.
pub const DRYRUN: &'static str = "byte(s) will be overwritten";
/// A descriptor that maps to the 'verbose' option.
pub const OVERWRITE: &'static str = "byte(s) overwritten";

/// A continue prompt.
pub const CONTINUE: &'static str = "- continue? [y/n]";
/// Indicates an action has been skipped.
pub const SKIP: &'static str = "skipped";
/// Represents a 'yes' response.
pub const YES: &'static str = "y";
/// Represents a 'no' response.
pub const NO: &'static str = "n";

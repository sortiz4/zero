//! Defines the options available.
use getopts::Matches;
use getopts::Options;
use super::error;
use super::text;

/// Used to define a new option flag.
pub struct Option<'a> {
    /// A short option (used with `-`).
    pub short: &'a str,
    /// A long option (used with `--`).
    pub long: &'a str,
    /// A brief description.
    pub description: &'a str,
}

/// Defines the 'dry-run' option flag.
pub const DRYRUN: Option<'static> = Option {
    short: "d",
    long: "dry-run",
    description: "Do not overwrite any files (verbose)"
};

/// Defines the 'help' option flag.
pub const HELP: Option<'static> = Option {
    short: "h",
    long: "help",
    description: "Output this message",
};

/// Defines the 'interactive' option flag.
pub const INTERACTIVE: Option<'static> = Option {
    short: "i",
    long: "interactive",
    description: "Prompt before overwriting each file"
};

/// Defines the 'recursive' option flag.
pub const RECURSIVE: Option<'static> = Option {
    short: "r",
    long: "recursive",
    description: "Recursively descend into directories"
};

/// Defines the 'suppress' option flag.
pub const SUPPRESS: Option<'static> = Option {
    short: "s",
    long: "suppress",
    description: "Suppress all interaction"
};

/// Defines the 'verbose' option flag.
pub const VERBOSE: Option<'static> = Option {
    short: "v",
    long: "verbose",
    description: "Explain what's being done"
};

/// Defines the 'version' option flag.
pub const VERSION: Option<'static> = Option {
    short: "V",
    long: "version",
    description: "Output version information"
};

/// Reformats the getopts error message.
macro_rules! format_opts_error {
    ($var:expr) => ($var.to_string().to_lowercase().trim_right_matches(".").to_owned());
}

/// Appends the help string to the end of the given message.
macro_rules! format_help_error {
    ($fmt:expr) => (format!(concat!($fmt, "\n{}"), text::HELP));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\n{}"), $($arg)*, text::HELP));
}

/// Initializes a set of options from the option definitions.
pub fn create_options() -> Options {
    let mut options = Options::new();
    options.optflag(DRYRUN.short, DRYRUN.long, DRYRUN.description);
    options.optflag(HELP.short, HELP.long, HELP.description);
    options.optflag(INTERACTIVE.short, INTERACTIVE.long, INTERACTIVE.description);
    options.optflag(RECURSIVE.short, RECURSIVE.long, RECURSIVE.description);
    options.optflag(SUPPRESS.short, SUPPRESS.long, SUPPRESS.description);
    options.optflag(VERBOSE.short, VERBOSE.long, VERBOSE.description);
    options.optflag(VERSION.short, VERSION.long, VERSION.description);
    return options;
}

/// Parses a set of arguments into a set of matches.
pub fn parse_options(args: &Vec<String>, options: &Options) -> Result<Matches, String> {
    let matches = match options.parse(&args[1..]) {
        Ok(v) => v,
        Err(e) => return Err(format_help_error!("{}", format_opts_error!(e))),
    };
    return Ok(matches);
}

/// Checks for conflicts in the set of matches.
pub fn check_conflicts(matches: &Matches) -> Result<(), String> {
    if matches.opt_present(INTERACTIVE.short) && matches.opt_present(SUPPRESS.short) {
        return Err(format_help_error!("{}: '{}', '{}'", error::MCONFLICT, INTERACTIVE.long, SUPPRESS.long));
    }
    return Ok(());
}

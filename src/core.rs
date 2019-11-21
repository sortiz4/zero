use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::Stderr;
use std::io::Stdin;
use std::io::Stdout;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use super::Error;
use super::Result;

enum Context {
    /// The path is absolute (has root).
    Absolute,
    /// The `interactive` option is present.
    Interactive,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Securely erase files (single-pass).")]
struct Options {
    /// Do not overwrite any files (verbose).
    #[structopt(short = "d", long = "dry-run")]
    dry_run: bool,

    /// Prompt before overwriting each file.
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,

    /// Recursively descend into directories.
    #[structopt(short = "r", long = "recursive")]
    recursive: bool,

    /// Suppress all interaction.
    #[structopt(short = "s", long = "suppress")]
    suppress: bool,

    /// Explain what's being done.
    #[structopt(short = "V", long = "verbose")]
    verbose: bool,

    /// Show this message.
    #[structopt(short = "h", long = "help")]
    help: bool,

    /// Show the version.
    #[structopt(short = "v", long = "version")]
    version: bool,

    /// The paths to be accessed by the tool.
    #[structopt(name = "PATHS", parse(from_str))]
    paths: Vec<PathBuf>,
}

pub struct Zero {
    options: Options,
    stderr: Stderr,
    stdout: Stdout,
    stdin: Stdin,
}

impl Zero {
    /// Constructs this program from an iterable of arguments.
    pub fn from_iter<I>(iter: I) -> Result<Self>
    where
        Self: Sized,
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        return Ok(
            Self {
                options: Options::from_iter_safe(iter)?,
                stderr: io::stderr(),
                stdout: io::stdout(),
                stdin: io::stdin(),
            }
        );
    }

    /// Replaces the standard error stream for this program.
    pub fn stderr(&mut self, stderr: Stderr) -> &mut Self {
        self.stderr = stderr;
        return self;
    }

    /// Replaces the standard output stream for this program.
    pub fn stdout(&mut self, stdout: Stdout) -> &mut Self {
        self.stdout = stdout;
        return self;
    }

    /// Replaces the standard input stream for this program.
    pub fn stdin(&mut self, stdin: Stdin) -> &mut Self {
        self.stdin = stdin;
        return self;
    }

    /// Runs this program and writes all errors.
    pub fn run(&mut self) -> Result<()> {
        match self.run_inner() {
            Ok(val) => {
                return Ok(val);
            },
            Err(err) => {
                writeln!(self.stderr, "Error: {}", err)?;
                return Err(err);
            },
        }
    }

    /// Runs this program.
    fn run_inner(&mut self) -> Result<()> {
        // Write the help or version message
        if self.options.help {
            return self.help();
        }
        if self.options.version {
            return self.version();
        }

        // Check for conflicting options
        self.validate()?;

        // Loop through the paths
        return self.overwrite();
    }

    /// Checks for conflicts in the options.
    fn validate(&self) -> Result<()> {
        return if self.options.interactive && self.options.suppress {
            Err(Error::Conflict)
        } else {
            Ok(())
        };
    }

    /// Writes the help message to the standard error stream.
    fn help(&mut self) -> Result<()> {
        Options::clap().write_help(&mut self.stderr)?;
        writeln!(self.stderr, "")?;
        return Ok(());
    }

    /// Writes the version message to the standard error stream.
    fn version(&mut self) -> Result<()> {
        Options::clap().write_version(&mut self.stderr)?;
        writeln!(self.stderr, "")?;
        return Ok(());
    }

    /// Authorizes directory and file access by prompting the user and reading
    /// from the standard input stream.
    fn auth(&mut self, path: &PathBuf, context: Context) -> Result<bool> {
        // Determine the appropriate prompt
        let prompt = match context {
            Context::Absolute => "is absolute",
            Context::Interactive => "will be overwritten",
        };

        let mut input = String::new();
        loop {
            // Prompt the user and normalize the input
            write!(self.stderr, r#""{}" {} - continue? [y/n] "#, path.display(), prompt)?;
            self.stdin.read_line(&mut input)?;

            // The response must be `y` or `n`
            match input.trim().to_lowercase().as_str() {
                "n" => {
                    if self.options.verbose {
                        writeln!(self.stderr, "Skipped.")?;
                    }
                    return Ok(false);
                },
                "y" => {
                    return Ok(true);
                },
                _ => {
                    input.clear();
                },
            }
        }
    }

    /// Overwrites all paths provided by the user. Authorization may be
    /// requested if the `suppress` option is not present.
    fn overwrite(&mut self) -> Result<()> {
        for path in self.options.paths.to_owned() {
            if !self.options.suppress && path.has_root() {
                // Authorize absolute paths (optional)
                if let Ok(false) = self.auth(&path, Context::Absolute) {
                    continue;
                }
            }

            if path.is_file() {
                // The path is a file
                self.overwrite_file(&path)?;
            } else {
                // Try the path as a directory
                self.overwrite_dir(&path)?;
            }
        }
        return Ok(());
    }

    /// Overwrites all files in the given directory and writes all errors.
    fn overwrite_dir(&mut self, path: &PathBuf) -> Result<()> {
        return if let Err(err) = self.overwrite_dir_inner(path) {
            self.write_error("Cannot access", path, &err)
        } else {
            Ok(())
        };
    }

    /// Overwrites all files in the given directory. If the `recursive` option
    /// is present, all files under the given directory will overwritten.
    fn overwrite_dir_inner(&mut self, path: &PathBuf) -> Result<()> {
        for entry in path.read_dir()? {
            let path = entry?.path();

            // Recurse if the entry is a directory (optional)
            if self.options.recursive && path.is_dir() {
                self.overwrite_dir(&path)?;
            } else if path.is_file() {
                self.overwrite_file(&path)?;
            }
        }
        return Ok(());
    }

    /// Overwrites the given file and writes all errors.
    fn overwrite_file(&mut self, path: &PathBuf) -> Result<()> {
        return if let Err(err) = self.overwrite_file_inner(path) {
            self.write_error("Cannot overwrite", path, &err)
        } else {
            Ok(())
        };
    }

    /// Overwrites the given file. Authorization may be requested and
    /// additional information may be written if the `interactive` and
    /// `verbose` options are present. The file will not be overwritten
    /// during a `dry-run`.
    fn overwrite_file_inner(&mut self, path: &PathBuf) -> Result<()> {
        if self.options.interactive {
            // Authorize every file (optional)
            if let Ok(false) = self.auth(path, Context::Interactive) {
                return Ok(());
            }
        }
        let metadata = path.metadata()?;

        if !self.options.dry_run {
            // Open the file and wrap it in a buffer
            let mut file = OpenOptions::new().write(true).open(path)?;
            let mut buf = BufWriter::new(file);

            // Overwrite the file
            for _ in 0..metadata.len() {
                buf.write(&[0])?;
            }

            // Flush the buffer to the disk
            file = buf.into_inner()?;
            file.sync_all()?;

            if self.options.verbose {
                // Write the results (optional)
                self.write_result("overwritten", path, metadata.len())?;
            }
        } else {
            // Perform a dry run (optional)
            self.write_result("will be overwritten", path, metadata.len())?;
        }
        return Ok(());
    }

    /// Writes a path related error to the standard error stream.
    fn write_error(&mut self, msg: &str, path: &PathBuf, err: &Error) -> Result<()> {
        writeln!(self.stderr, r#"Error: {} "{}": {}"#, msg, path.display(), err)?;
        return Ok(());
    }

    /// Writes the result of an operation to the standard output stream.
    fn write_result(&mut self, msg: &str, path: &PathBuf, len: u64) -> Result<()> {
        writeln!(self.stdout, r#""{}": {} byte(s) {}."#, path.display(), len, msg)?;
        return Ok(());
    }
}

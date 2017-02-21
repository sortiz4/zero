//! Defines the core functionality.
use getopts::Matches;
use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use super::error;
use super::opts;
use super::text;

/// Named constants used to indicate the authorization context.
pub enum Auth {
    /// The path is absolute (has root).
    Absolute,
    /// The 'interactive' option is present.
    Interactive,
}

/// Authorizes directory and file access by prompting the user and reading the
/// standard input. `true` will be returned if the user grants permission and
/// `false` will be returned otherwise.
/// # Issues
/// - If the standard input is closed or empty, no error will be raised and the
/// loop will continue indefinitely.
pub fn auth(path: &Path, auth: Auth) -> bool {
    let stdin_err = format_sys!("{}", error::MSTDINERR);

    // Determine the appropriate prompt
    let prompt = match auth {
        Auth::Absolute => text::ABSOLUTE,
        Auth::Interactive => text::INTERACTIVE,
    };

    loop {
        // The input buffer must be reset with every pass
        let mut input = String::new();

        // Print a confirmation prompt and wait for input
        sys!("'{}' {} {} ", path.display(), prompt, text::CONTINUE).unwrap();
        io::stdin().read_line(&mut input).expect(&stdin_err);

        // Normalize the input for comparison
        input = input.trim().to_lowercase();

        // The response must be YES or NO
        match input.as_str() {
            text::YES => {
                return true;
            },
            text::NO => {
                sysln!("{}", text::SKIP).unwrap();
                return false;
            },
            _ => continue,
        }
    }
}

/// Adds all files *in* the given directory to the `list`. If the 'recursive'
/// option is present, all files *under* the given directory will be added to
/// the `list`.
pub fn collect_files(dir: &Path, list: &mut Vec<PathBuf>, matches: &Matches) -> Result<()> {

    // Iterate over all entries in the directory
    for entry in try!(dir.read_dir()) {
        let path = try!(entry).path();

        // Recurse if the entry is a directory (optional)
        if matches.opt_present(opts::RECURSIVE.short) && path.is_dir() {
            try!(collect_files(&path, list, matches));

        // If the entry is a file, add it to the list
        } else if path.is_file() {
            list.push(path);
        }
    }
    return Ok(());
}

/// Overwrites the given file. Authorization may be requested and additional
/// information may be printed if the 'interactive' and 'verbose' options are
/// present. The file will not be overwritten during a 'dry-run'.
pub fn overwrite_file(path: &Path, matches: &Matches) -> Result<()> {

    // Authorize every file (optional)
    if matches.opt_present(opts::INTERACTIVE.short) && !matches.opt_present(opts::SUPPRESS.short) {
        match auth(&path, Auth::Interactive) {
            true => (),
            false => return Ok(()),
        }
    }

    // Unwrap the file metadata
    let metadata = try!(path.metadata());
    
    // Overwrite the file or perform a dry run (optional)
    if !matches.opt_present(opts::DRYRUN.short) {

        // Open the file and wrap it in a buffered writer
        let mut file = try!(OpenOptions::new().write(true).open(path));
        let mut buffer = BufWriter::new(file);

        // Overwrite the file
        for _ in 0..metadata.len() {
            try!(buffer.write(b"\0"));
        }

        // Flush the buffer and write to the disk
        file = try!(buffer.into_inner());
        try!(file.sync_all());

        // Print the result to the standard output (optional)
        if matches.opt_present(opts::VERBOSE.short) {
            println!("'{}': {} {}", path.display(), metadata.len(), text::OVERWRITE);
        }
    } else {
        println!("'{}': {} {}", path.display(), metadata.len(), text::DRYRUN);
    }
    return Ok(());
}

/// Calls `overwrite_file` for each file in the `list` and handles all
/// associated errors internally.
pub fn overwrite_files(list: &Vec<PathBuf>, matches: &Matches) {
    for file in list.iter() {
        match overwrite_file(file, matches) {
            Ok(_) => (),
            Err(e) => {
                sysln!("{} '{}': {}", error::MACCESS, file.display(), e).unwrap();
                continue;
            },
        }
    }
}

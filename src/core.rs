//! Defines the core functionality behind `zero`.
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
    /// The given path is absolute (has root).
    Absolute,
    /// The user has requested interaction.
    Interactive,
}

/// Authorizes directory and file access by prompting the user and reading the
/// standard input. `true` will be returned if the user grants permission and
/// `false` will be returned otherwise.
/// # Issues
/// - If the standard input is closed or empty, no error will be raised and the
/// loop will continue indefinitely.
pub fn auth(path: &Path, auth: Auth) -> bool {
    let stdin_err = formatsys!("{}", error::MSTDINERR);

    // Determine the appropriate prompt
    let msg = match auth {
        Auth::Absolute => text::ABSOLUTE,
        Auth::Interactive => text::INTERACTIVE,
    };

    loop {
        // The input buffer must be reset with every pass
        let mut input = String::new();

        // Print a confirmation prompt and wait for input
        sys!("'{}' {} {} ", path.display(), msg, text::CONTINUE).unwrap();
        io::stdin().read_line(&mut input).expect(stdin_err.as_str());

        // Normalize the input for comparison
        input = input.trim().to_lowercase();
        let normal = input.as_str();

        // The response must be YES or NO
        if normal == text::NO {
            sysln!("{}", text::SKIP).unwrap();
            return false;
        } else if normal == text::YES {
            return true;
        }
    }
}

/// Adds all files in `dir` to `list`. If the 'recursive' option is given,
/// `collect_files` will add all files under `dir` to `list`.
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

/// Zero-outs the given file. Authorization may be requested and additional
/// information may be printed if the 'interactive' and 'verbose' options are
/// given. The file will not be overwritten during a 'dry-run'.
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

        // Zero-out the buffer
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

/// Calls `overwrite_file` for each file in `list` and handles all associated errors.
pub fn overwrite_files(list: &Vec<PathBuf>, matches: &Matches) {

    // Overwrite all files in the list
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

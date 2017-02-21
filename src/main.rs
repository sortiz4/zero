#[macro_use]
extern crate zero;
use std::env;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use zero::core;
use zero::core::Auth;
use zero::error;
use zero::opts;
use zero::text;

fn main() {
    process::exit(zero(env::args().collect()));
}

fn zero(args: Vec<String>) -> i32 {

    // Create and parse the command line options
    let options = opts::create_options();
    let matches = match opts::parse_options(&args, &options) {
        Ok(v) => v,
        Err(e) => {
            sysln!("{}", e).unwrap();
            return error::EUSAGE;
        },
    };

    // Check for conflicting options
    match opts::check_conflicts(&matches) {
        Ok(_) => (),
        Err(e) => {
            sysln!("{}", e).unwrap();
            return error::EUSAGE;
        },
    }

    // Display the help message and exit (optional)
    if matches.opt_present(opts::HELP.short) {
        print!("{}", options.usage(text::USAGE));
        return error::ESUCCESS;
    }

    // Display the version and exit (optional)
    if matches.opt_present(opts::VERSION.short) {
        println!("{} {}", text::NAME, text::VERSION);
        return error::ESUCCESS;
    }

    // Loop through the free arguments (directories and files)
    for item in matches.free.iter() {

        // Create a path and a new list of files
        let path = Path::new(item);
        let mut list: Vec<PathBuf> = Vec::new();

        // Authorize absolute paths (optional)
        if !matches.opt_present(opts::SUPPRESS.short) && path.has_root() {
            match core::auth(&path, Auth::Absolute) {
                true => (),
                false => continue,
            }
        }

        // Collect files in a directory
        if path.is_dir() {
            match core::collect_files(&path, &mut list, &matches) {
                Ok(_) => (),
                Err(e) => {
                    sysln!("{} '{}': {}", error::MACCESS, item, e).unwrap();
                    continue;
                },
            }

        // Collect single files
        } else if path.is_file() {
            list.push(path.to_owned());

        // The path could not be found otherwise
        } else {
            sysln!("'{}' {}", item, error::MNOTFOUND).unwrap();
        }

        // Overwrite each file in the list
        if list.len() > 0 {
            core::overwrite_files(&list, &matches);
        }
    }
    return error::ESUCCESS;
}

#[macro_use]
extern crate zero;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use zero::core;
use zero::core::Auth;
use zero::opts;
use zero::status;
use zero::text;

fn main() {
    process::exit(zero(env::args().collect()));
}

fn zero(args: Vec<String>) -> i32 {
    // Parse the command line options
    let options = opts::create_options();
    let matches = match opts::parse_options(&args, &options) {
        Ok(val) => val,
        Err(err) => {
            sprintln!("{}", err);
            return status::EUSAGE;
        },
    };

    // Check for conflicting options
    if let Err(err) = opts::check_conflicts(&matches) {
        sprintln!("{}", err);
        return status::EUSAGE;
    }

    // Display the help message and exit (optional)
    if matches.opt_present(opts::HELP.short) {
        print!("{}", options.usage(text::USAGE));
        return status::ESUCCESS;
    // Display the version and exit (optional)
    } else if matches.opt_present(opts::VERSION.short) {
        println!("{} {}", text::NAME, text::VERSION);
        return status::ESUCCESS;
    }

    // Loop through the free arguments (paths)
    for item in matches.free.iter() {

        // Create a path and a new list of files
        let path = Path::new(item);
        let mut list: Vec<PathBuf> = Vec::new();

        // Authorize absolute paths (optional)
        if !matches.opt_present(opts::SUPPRESS.short) && path.has_root() {
            if let false = core::auth(&path, Auth::Absolute) {
                continue;
            }
        }

        // Collect files in a directory
        if path.is_dir() {
            if let Err(err) = core::collect_files(&path, &mut list, &matches) {
                sprintln!("{} '{}': {}", status::MACCESS, item, err);
                continue;
            }
        // Collect single files
        } else if path.is_file() {
            list.push(path.to_owned());
        // The path could not be found otherwise
        } else {
            sprintln!("'{}' {}", item, status::MNOTFOUND);
        }

        // Overwrite each file in the list
        if list.len() > 0 {
            core::overwrite_files(&list, &matches);
        }
    }
    return status::ESUCCESS;
}

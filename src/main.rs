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
    let options = opts::create();
    let matches = match opts::parse(&args, &options) {
        Ok(val) => val,
        Err(err) => {
            sprintln!("{}", err);
            return status::EUSAGE;
        },
    };

    // Display the help message or version and exit (optional)
    if matches.opt_present(opts::HELP.short) {
        print!("{}", options.usage(text::USAGE));
        return status::ESUCCESS;
    } else if matches.opt_present(opts::VERSION.short) {
        println!("{} {}", text::NAME, text::VERSION);
        return status::ESUCCESS;
    }

    // Check for conflicting options
    if let Err(err) = opts::validate(&matches) {
        sprintln!("{}", err);
        return status::EUSAGE;
    }

    // Loop through the free arguments (paths)
    let mut list: Vec<PathBuf> = Vec::new();
    for item in matches.free.iter() {
        let path = Path::new(item);

        // Authorize absolute paths (optional)
        if !matches.opt_present(opts::SUPPRESS.short) && path.has_root() {
            if let false = core::auth(&path, Auth::Absolute) {
                continue;
            }
        }

        // Verify that the path exists in the file system
        if path.exists() {
            // Collect files in directories or collect single files
            if path.is_dir() {
                if let Err(err) = core::collect_files(&path, &mut list, &matches) {
                    sprintln!("{} '{}': {}", status::MACCESS, item, err);
                    continue;
                }
            } else if path.is_file() {
                list.push(path.to_owned());
            }
        } else {
            sprintln!("'{}' {}", item, status::MNOTFOUND);
        }

        // Overwrite each file in the list
        if list.len() > 0 {
            core::overwrite_files(&list, &matches);
            list.clear(); // Truncate the list
        }
    }
    return status::ESUCCESS;
}

use std::env;
use std::process;
use zero::Zero;

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    if let Ok(mut zero) = Zero::from_iter(env::args()) {
        if let Ok(_) = zero.run() {
            return 0;
        }
    }
    return 1;
}

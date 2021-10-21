use bklt;
use std::process;

fn main() {
    if let Err(e) = bklt::run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

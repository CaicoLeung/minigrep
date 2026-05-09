use std::process;

use clap::Parser;

fn main() {
    let config = minigrep::Config::parse();
    if let Err(err) = minigrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

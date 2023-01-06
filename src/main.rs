extern crate mkmk;

use std::process;

use clap::Parser;

/// `mkdir -p` and `touch`.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The path to the file to create
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let path_strings = args.paths;
    let mut exit_code = 0;

    for arg in path_strings {
        if let Err(e) = mkmk::run(&arg) {
            eprintln!("Error: {}", e);
            exit_code = 1;
        }
    }

    process::exit(exit_code);
}

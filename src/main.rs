extern crate mkmk;

use std::{path::PathBuf, process};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// The path to the file to create
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let mut exit_code = 0;

    for path in args.paths {
        if let Err(e) = mkmk::run(&path) {
            eprintln!("Error: {}", e);
            exit_code = 1;
        }
    }

    process::exit(exit_code);
}

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

    let exit_code = mkmk::run(args.paths);

    process::exit(exit_code);
}

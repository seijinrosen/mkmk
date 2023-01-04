extern crate mkmk;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("ヘルプメッセージ");
        process::exit(0);
    }

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if let Err(e) = mkmk::run(arg) {
            eprintln!("error: {}: {}", arg, e);
        }
    }
}

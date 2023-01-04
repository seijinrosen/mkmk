use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("ヘルプメッセージ");
        process::exit(0);
    }

    run(args);
}

fn run(args: Vec<String>) {
    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let path = Path::new(arg);

        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                println!("mkdir error: {:?} {}", parent, e);
                continue;
            }
        }

        if let Err(e) = File::create(path) {
            println!("touch error: {:?} {}", path, e);
        }
    }
}

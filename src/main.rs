use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let path = Path::new(arg);
        let parent = path.parent();

        if let Some(parent) = parent {
            fs::create_dir_all(parent).unwrap_or_else(|err| {
                println!("エラー発生: {}", err);
                process::exit(1);
            });
        }

        File::create(path).unwrap_or_else(|err| {
            println!("エラー発生: {}", err);
            process::exit(1);
        });
    }
}

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

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let path = Path::new(arg);

        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => (),
                Err(e) => {
                    println!("mkdir error: {:?} {}", parent, e);
                    continue;
                }
            }
        }

        match File::create(path) {
            Ok(_) => (),
            Err(e) => {
                println!("touch error: {:?} {}", path, e);
            }
        }
    }
}
